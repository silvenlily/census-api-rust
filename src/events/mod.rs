use std::sync::{Arc, Mutex};

use futures_util::{
    SinkExt,
    stream::{SplitSink, SplitStream}, StreamExt,
};
use native_tls::TlsConnector;
use serde_json::Value;
use tokio::net::TcpStream;
use tokio_tungstenite::{
    connect_async_tls_with_config, Connector, MaybeTlsStream, tungstenite::Message, WebSocketStream,
};

use crate::events::api_events::{
    ContinentLock, ContinentUnlock, FacilityControl, MetagameEvent, PlayerLogin, PlayerLogout,
};
use crate::events::api_events::Event;
use crate::events::api_events::event_types::ApiEvent;
use crate::utils::CensusError;

use self::api_command::ApiCommand;

pub mod api_command;
pub mod api_events;

pub mod environments {
    pub static PC: &str = "ps2";
    pub static PS4_US: &str = "ps2ps4us";
    pub static PS4_EU: &str = "ps2ps4eu";
}

async fn connect_tls_stream(
    environment: &str,
    serviceid: &str,
    reconnect_count: &f64,
) -> Result<WebSocketStream<MaybeTlsStream<TcpStream>>, CensusError> {
    if reconnect_count > &10_f64 {
        return Err(CensusError {
            err_msg: "Connection dropped too many times".to_string(),
            parent_err: (None),
        });
    }

    let try_tls = TlsConnector::new();

    match try_tls {
        Err(err) => {
            return Err(CensusError {
                err_msg: "Unable to create TLS connector".to_string(),
                parent_err: Some(err.to_string()),
            });
        }
        Ok(tls) => {
            let url: String = "wss://push.planetside2.com/streaming?environment=".to_string()
                + environment
                + "&service-id=s:"
                + serviceid;

            let try_connect =
                connect_async_tls_with_config(url, None, Some(Connector::NativeTls(tls))).await;

            match try_connect {
                Err(err) => {
                    return Err(CensusError {
                        err_msg: "Unable to connect to census events api".to_string(),
                        parent_err: Some(err.to_string()),
                    });
                }

                Ok(connect) => {
                    let (ws_stream, _) = connect;
                    return Ok(ws_stream);
                }
            }
        }
    }
}

pub async fn connect(environment: &str, serviceid: &str) -> Result<EventClient, CensusError> {
    let tls_streams = connect_tls_stream(environment, serviceid, &0_f64).await?;

    let (ws_write, ws_read) = tls_streams.split();

    return Ok(EventClient {
        reconnect_count: 0_f64,
        serviceid: serviceid.to_string(),
        environment: environment.to_string(),
        ws_write: Arc::new(Mutex::new(ws_write)),
        ws_read: Arc::new(Mutex::new(ws_read)),
    });
}

#[derive(Clone)]
pub struct EventClient {
    reconnect_count: f64,
    serviceid: String,
    environment: String,
    ws_write: Arc<Mutex<SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>>>,
    ws_read: Arc<Mutex<SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>>>,
}

impl EventClient {
    #[async_recursion::async_recursion(? Send)]
    pub async fn next_event(&mut self) -> Result<ApiEvent, CensusError> {
        let try_next_ws_msg = self.next_ws_msg().await;

        match try_next_ws_msg {
            Err(err) => {
                return Err(err);
            }
            Ok(msg) => {
                if msg.is_close() {
                    let tls_streams = connect_tls_stream(
                        &self.environment,
                        &self.serviceid,
                        &self.reconnect_count,
                    )
                        .await?;

                    let (ws_write, ws_read) = tls_streams.split();

                    self.ws_write = Arc::new(Mutex::new(ws_write));
                    self.ws_read = Arc::new(Mutex::new(ws_read));

                    return self.next_event().await;
                }

                let try_event_txt = msg.into_text();

                match try_event_txt {
                    Err(err) => {
                        return Err(CensusError {
                            err_msg: "Could not parse ws message to text".to_string(),
                            parent_err: Some(err.to_string()),
                        });
                    }
                    Ok(event_text) => {
                        println!("got ws message: {}",event_text);

                        let try_event_json: Result<Value, serde_json::Error> =
                            serde_json::from_str(&event_text);

                        match try_event_json {
                            Err(err) => {
                                return Err(CensusError {
                                    err_msg: "Could not parse ws message to json".to_string(),
                                    parent_err: Some(err.to_string()),
                                });
                            }
                            Ok(event_json) => {
                                let event_type_v = &event_json["type"];
                                if event_type_v.is_string() {
                                    let event_type = event_type_v.to_string();

                                    match event_type.as_str() {
                                        "\"heartbeat\"" => {
                                            return self.next_event().await;
                                        }
                                        "\"serviceMessage\"" => {
                                            return parse_service_message(event_json);
                                        }
                                        "\"serviceStateChanged\"" => {
                                            return self.next_event().await;
                                        }
                                        "\"connectionStateChanged\"" => {
                                            return self.next_event().await;
                                        }
                                        _ => {
                                            let msg =
                                                "Unknown event type: ".to_string() + &event_type;
                                            return Err(CensusError {
                                                err_msg: msg,
                                                parent_err: None,
                                            });
                                        }
                                    }
                                }
                                let subscribe_response_v = &event_json["subscribe"];
                                if subscribe_response_v.is_null() {
                                    return self.next_event().await;
                                }

                                return Err(CensusError {
                                    err_msg: "Could not determine event type".to_string(),
                                    parent_err: None,
                                });
                            }
                        }
                    }
                }
            }
        }
    }
    pub async fn next_ws_msg(&mut self) -> Result<Message, CensusError> {
        if self.reconnect_count > 1_f64 {
            self.reconnect_count -= 0.1_f64;
        }

        let try_ws_read_lock = self.ws_read.lock();

        match try_ws_read_lock {
            Err(err) => {
                return Err(CensusError {
                    err_msg: "Poisoned websocket connection".to_string(),
                    parent_err: Some(err.to_string()),
                });
            }
            Ok(mut ws_read_lock) => {
                let try_next = ws_read_lock.next().await;

                match try_next {
                    Some(next) => {
                        match next {
                            Err(err) => {
                                return Err(CensusError {
                                    err_msg: "Unable to get next websocket message".to_string(),
                                    parent_err: Some(err.to_string()),
                                });
                            }
                            Ok(msg) => {
                                return Ok(msg);
                            }
                        };
                    }
                    None => {
                        return Err(CensusError {
                            err_msg: "Unable to send message to census api".to_string(),
                            parent_err: None,
                        });
                    }
                };
            }
        };
    }
    pub async fn send(&mut self, command: &impl ApiCommand) -> Result<(), CensusError> {
        let payload = command.to_json().to_string();

        let try_ws_write_lock = self.ws_write.lock();

        match try_ws_write_lock {
            Err(err) => Err(CensusError {
                err_msg: "Poisoned websocket connection".to_string(),
                parent_err: Some(err.to_string()),
            }),
            Ok(mut ws_write_lock) => {
                let res = ws_write_lock
                    .send(tokio_tungstenite::tungstenite::Message::Text(payload))
                    .await;

                match res {
                    Ok(_) => Ok(()),
                    Err(err) => Err(CensusError {
                        err_msg: "Unable to send message to census api".to_string(),
                        parent_err: Some(err.to_string()),
                    }),
                }
            }
        }
    }
}

pub fn parse_service_message(message: Value) -> Result<ApiEvent, CensusError> {
    let payload = &message["payload"];
    let event_name = &payload["event_name"];

    if !event_name.is_string() {
        return Err(CensusError {
            err_msg: "Not a service message".to_string(),
            parent_err: None,
        });
    }

    match event_name.as_str().unwrap() {
        "PlayerLogin" => {
            return Ok(ApiEvent::PlayerLogin(PlayerLogin::from_json(payload.clone())?));
        }
        "PlayerLogout" => {
            return Ok(ApiEvent::PlayerLogout(PlayerLogout::from_json(payload.clone())?));
        }
        "FacilityControl" => {
            return Ok(ApiEvent::FacilityControl(FacilityControl::from_json(payload.clone())?));
        }
        "ContinentLock" => {
            return Ok(ApiEvent::ContinentLock(ContinentLock::from_json(payload.clone())?));
        }
        "ContinentUnlock" => {
            return Ok(ApiEvent::ContinentUnlock(ContinentUnlock::from_json(payload.clone())?));
        }
        "MetagameEvent" => {
            return Ok(ApiEvent::MetagameEvent(MetagameEvent::from_json(payload.clone())?));
        }
        _ => {
            let msg = "Unknown event name: ".to_string() + event_name.as_str().unwrap();
            return Err(CensusError {
                err_msg: msg,
                parent_err: None,
            });
        }
    }
}
