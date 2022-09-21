use std::sync::{Arc, Mutex};

use futures_util::{
    stream::{SplitSink, SplitStream},
    SinkExt, StreamExt,
};
use native_tls::TlsConnector;
use tokio::net::TcpStream;
use tokio_tungstenite::{
    connect_async_tls_with_config, tungstenite::Message, Connector, MaybeTlsStream, WebSocketStream,
};

use crate::utils::CensusError;

use self::api_command::ApiCommand;

pub mod api_command;
pub mod api_events;

pub mod environments {
    pub static PC: &'static str = "ps2";
    pub static PS4_US: &'static str = "ps2ps4us";
    pub static PS4_EU: &'static str = "ps2ps4eu";
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
        serviceid: serviceid.clone().to_string(),
        environment: environment.clone().to_string(),
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
    pub async fn next_event(&mut self) -> Result<(), CensusError> {
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

                    return Ok(());
                }

                println!("ws msg: {}", msg.to_text().unwrap());

                // TODO: parse incoming api events

                return Ok(());
            }
        }
    }
    pub async fn next_ws_msg(&mut self) -> Result<Message, CensusError> {
        if self.reconnect_count > 1_f64 {
            self.reconnect_count = self.reconnect_count - 0.1_f64;
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
