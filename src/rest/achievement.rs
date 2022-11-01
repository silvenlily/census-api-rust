use std::{sync::Arc, time::SystemTime};

use num_traits::ToPrimitive;
use serde_json::{json, Value};

use crate::utils::CensusError;

use super::{census_value::CensusValue, item::Item, query_builder::Resolveable, RestClient};

pub enum AchievementResolves {}

impl Resolveable for AchievementResolves {
    fn from_resolve_string(resolve: &str) -> Option<Self> {
        return None;
    }

    fn to_resolve_string(&self) -> String {
        return "".to_string();
    }
}

///
/// Represents a character
///
#[derive(Clone, Debug)]
pub struct Achievement {
    pub owning_client: Arc<RestClient>,
    pub id: u64,
    pub item_id: CensusValue<String>,
    pub name: CensusValue<String>,
    pub reward_id: CensusValue<String>,
    pub repeatable: CensusValue<bool>,
    pub description: CensusValue<String>,
    pub image_set_id: CensusValue<String>,
    pub image_id: CensusValue<String>,
    pub image_path: CensusValue<String>,
}

impl Achievement {
    fn update(&mut self, json: &Value) {
        self.name.update(&json["name"]["en"]);
        self.item_id.update(&json[""]);
        self.name.update(&json[""]);
        self.reward_id.update(&json[""]);
        self.repeatable.update(&json[""]);
        self.description.update(&json[""]);
        self.image_set_id.update(&json[""]);
        self.image_id.update(&json[""]);
        self.image_path.update(&json[""]);
    }

    pub fn new(id: u64, rest_client: Arc<RestClient>) -> Self {
        Achievement {
            owning_client: rest_client,
            id,
            item_id: CensusValue { last_updated: None, value: None },
            name: CensusValue { value: None, last_updated: None },
            reward_id: CensusValue { last_updated: None, value: None },
            repeatable: CensusValue { last_updated: None, value: None },
            description: CensusValue { last_updated: None, value: None },
            image_set_id: CensusValue { last_updated: None, value: None },
            image_id: CensusValue { last_updated: None, value: None },
            image_path: CensusValue { last_updated: None, value: None },
        }
    }

    pub async fn fetch(
        &mut self,
    ) -> Result<(), CensusError> {
        let mut query = self.owning_client.get_query_builder("achievement");

        query.limit(1);

        query.search("achievement_id".to_string(), self.id.to_string());

        let char = query.get().await;

        match char {
            Err(err) => return Err(err),
            Ok(jsonval) => {
                let jsonchar = jsonval["achievement_list"][0].clone();

                self.update(&jsonchar);

                return Ok(());
            }
        }
    }

    /// Creates a character and prefetches it
    pub async fn new_prefeched(
        rest_client: Arc<RestClient>,
        id: u64,
    ) -> Result<Self, CensusError> {
        let mut achievement = Achievement::new(id, rest_client);

        achievement.fetch().await?;

        return Ok(achievement);
    }

    //fn from_json_value(json: &Value, rest_client: Arc<RestClient>) -> Result<Self, CensusError> {
    //    let id_v = &json["character_id"];
    //
    //    if !(id_v.is_string() || id_v.is_number()) {
    //        return Err(CensusError {
    //            err_msg: "Could not get character id".to_string(),
    //            parent_err: None,
    //        });
    //    }
    //
    //
    //
    //    if !(id_v.is_string() || id_v.is_number()) {
    //        return Err(CensusError {
    //            err_msg: "Could not get character id".to_string(),
    //            parent_err: None,
    //        });
    //    }
    //
    //    let mut char = Achievement::new(id_v.to_string(), rest_client);
    //
    //    char.update(json);
    //
    //    return Ok(char);
    //}
}
