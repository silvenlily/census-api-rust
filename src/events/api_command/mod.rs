use serde_json::{json, Value};

use super::api_events::event_types::ApiSubscriptionName;

pub struct Echo {
    pub payload: Value,
}

impl ApiCommand for Echo {
    fn to_json(&self) -> Value {
        return json!({
          "action": "echo",
          "payload": self.payload,
          "service": "event"
        });
    }
}

pub struct Subscribe {
    pub subscription_names: Vec<ApiSubscriptionName>,
    pub character_ids: Option<Vec<String>>,
    pub server_ids: Option<Vec<String>>,
}

impl ApiCommand for Subscribe {
    fn to_json(&self) -> Value {
        let mut events: Vec<Value> = Vec::with_capacity(self.subscription_names.len());
        for event in &self.subscription_names {
            events.push(Value::String(event.str().to_string()));
        }

        let mut chars;
        match &self.character_ids {
            Some(characters) => {
                chars = Vec::with_capacity(characters.len());
                for char in characters {
                    chars.push(Value::String(char.clone()));
                }
            }
            None => {
                chars = Vec::new();
            }
        }

        let mut worlds;

        match &self.server_ids {
            Some(servers) => {
                worlds = Vec::with_capacity(servers.len());
                for world in servers {
                    worlds.push(Value::String(world.to_string()));
                }
            }
            None => {
                worlds = Vec::new();
            }
        }

        return json!({
            "action": "subscribe",
            "characters": Value::Array(chars),
            "eventNames": Value::Array(events),
            "worlds": Value::Array(worlds),
            "service": "event"

        });
    }
}

pub trait ApiCommand {
    fn to_json(&self) -> Value;
}
