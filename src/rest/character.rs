use std::{sync::Arc, time::SystemTime};

use num_traits::ToPrimitive;
use serde_json::{json, Value};

use crate::utils::CensusError;

use super::{census_value::CensusValue, item::Item, query_builder::Resolveable, RestClient};

pub enum CharacterResolves {
    Item,
    ItemFull,
    Profile,
    Faction,
    Stat,
    StatByFaction,
    WeaponStat,
    WeaponStatByFaction,
    StatHistory,
    OnlineStatus,
    Friends,
    World,
    Outfit,
    OutfitMember,
    OutfitMemberExtended,
    Currency,
}

impl Resolveable for CharacterResolves {
    fn from_resolve_string(resolve: &str) -> Option<Self> {
        match resolve {
            "item" => Some(CharacterResolves::Item),
            "item_full" => Some(CharacterResolves::ItemFull),
            "profile" => Some(CharacterResolves::Profile),
            "faction" => Some(CharacterResolves::Faction),
            "stat" => Some(CharacterResolves::Stat),
            "stat_by_faction" => Some(CharacterResolves::StatByFaction),
            "weapon_stat" => Some(CharacterResolves::WeaponStat),
            "weapon_stat_by_faction" => Some(CharacterResolves::WeaponStatByFaction),
            "stat_history" => Some(CharacterResolves::StatHistory),
            "online_status" => Some(CharacterResolves::OnlineStatus),
            "friends" => Some(CharacterResolves::Friends),
            "world" => Some(CharacterResolves::World),
            "outfit" => Some(CharacterResolves::Outfit),
            "outfit_memeber" => Some(CharacterResolves::OutfitMember),
            "outfit_member_extended" => Some(CharacterResolves::OutfitMemberExtended),
            "currency" => Some(CharacterResolves::Currency),
            _ => {
                return None;
            }
        }
    }

    fn to_resolve_string(&self) -> String {
        match self {
            CharacterResolves::Item => "item".to_string(),
            CharacterResolves::ItemFull => "item_full".to_string(),
            CharacterResolves::Profile => "profile".to_string(),
            CharacterResolves::Faction => "faction".to_string(),
            CharacterResolves::Stat => "stat".to_string(),
            CharacterResolves::StatByFaction => "stat_by_faction".to_string(),
            CharacterResolves::WeaponStat => "weapon_stat".to_string(),
            CharacterResolves::WeaponStatByFaction => "weapon_stat_by_faction".to_string(),
            CharacterResolves::StatHistory => "stat_history".to_string(),
            CharacterResolves::OnlineStatus => "online_status".to_string(),
            CharacterResolves::Friends => "friends".to_string(),
            CharacterResolves::World => "world".to_string(),
            CharacterResolves::Outfit => "outfit".to_string(),
            CharacterResolves::OutfitMember => "outfit_member".to_string(),
            CharacterResolves::OutfitMemberExtended => "outfit_member_extended".to_string(),
            CharacterResolves::Currency => "currency".to_string(),
        }
    }
}

#[derive(Clone)]
pub enum CharacterClass {
    Infiltrator,
    LightAssault,
    CombatMedic,
    Engineer,
    HeavyAssault,
    Max,
}

impl CharacterClass {
    pub fn new(name_or_id: &str) -> Option<CharacterClass> {
        match name_or_id {
            "infiltrator" => Some(CharacterClass::Infiltrator),
            "max" => Some(CharacterClass::Max),
            "engineer" => Some(CharacterClass::Engineer),
            "light assault" => Some(CharacterClass::LightAssault),
            "combat medic" => Some(CharacterClass::CombatMedic),
            "heavy assault" => Some(CharacterClass::HeavyAssault),
            "light_assault" => Some(CharacterClass::LightAssault),
            "combat_medic" => Some(CharacterClass::CombatMedic),
            "heavy_assault" => Some(CharacterClass::HeavyAssault),
            "lightassault" => Some(CharacterClass::LightAssault),
            "combatmedic" => Some(CharacterClass::CombatMedic),
            "heavyassault" => Some(CharacterClass::HeavyAssault),
            "1" => Some(CharacterClass::Infiltrator),
            "3" => Some(CharacterClass::LightAssault),
            "4" => Some(CharacterClass::CombatMedic),
            "5" => Some(CharacterClass::Engineer),
            "6" => Some(CharacterClass::HeavyAssault),
            "7" => Some(CharacterClass::Max),
            _ => None,
        }
    }
    pub fn to_id(&self) -> String {
        match self {
            CharacterClass::Infiltrator => "1".to_string(),
            CharacterClass::LightAssault => "3".to_string(),
            CharacterClass::CombatMedic => "4".to_string(),
            CharacterClass::Engineer => "5".to_string(),
            CharacterClass::HeavyAssault => "6".to_string(),
            CharacterClass::Max => "7".to_string(),
        }
    }
}

///
/// Represents a character
///
#[derive(Clone, Debug)]
pub struct Character {
    owning_client: Arc<RestClient>,
    id: String,
    name: CensusValue<String>,
    faction_id: CensusValue<u8>,
    head_id: CensusValue<String>,
    title_id: CensusValue<String>,
    created_at: CensusValue<String>,
    last_updated: CensusValue<String>,
    last_login: CensusValue<String>,
    login_count: CensusValue<u64>,
    minutes_played: CensusValue<u64>,
    certs_earned: CensusValue<u64>,
    certs_gifted: CensusValue<u64>,
    certs_spent: CensusValue<u64>,
    certs_available: CensusValue<u64>,
    certs_progress: CensusValue<u8>,
    battle_rank: CensusValue<u8>,
    battle_rank_progress: CensusValue<u8>,
    profile_id: CensusValue<u8>,
    daily_ribbon_count: CensusValue<u8>,
    daily_ribbon_time: CensusValue<String>,
    is_asp: CensusValue<bool>,
    // resolved by item or item_full
    //items: CensusValue<Vec<Item>>,
    // resolved by profile
    //class: CensusValue<CharacterClass>,
    //
}

impl Character {
    fn update(&mut self,json:&Value) {
        self.name.update(&json["name"]["first"]);
        self.faction_id.update(&json["faction_id"]);
        self.head_id.update(&json["head_id"]);
        self.title_id.update(&json["title_id"]);
        self.created_at.update(&json["times"]["creation"]);
        self.last_updated.update(&json["times"]["last_save"]);
        self.last_login.update(&json["times"]["last_login"]);
        self.login_count.update(&json["times"]["login_count"]);
        self.minutes_played.update(&json["times"]["minutes_played"]);
        self.certs_earned.update(&json["certs"]["earned_points"]);
        self.certs_gifted.update(&json["certs"]["gifted_points"]);
        self.certs_spent.update(&json["certs"]["spent_points"]);
        self.certs_available.update(&json["certs"]["available_points"]);
        self.certs_progress.update(&json["certs"]["percent_to_next"]);
        self.battle_rank.update(&json["battle_rank"]["value"]);
        self.battle_rank_progress.update(&json["battle_rank"]["percent_to_next"]);
        self.profile_id.update(&json["profile_id"]);
        self.daily_ribbon_count.update(&json["daily_ribbon"]["count"]);
        self.daily_ribbon_time.update(&json["daily_ribbon"]["time"]);
        self.is_asp.update(&json["prestige_level"]);
    }

    pub fn new(id: String, rest_client: Arc<RestClient>) -> Self {
        Character {
            owning_client: rest_client,
            id,
            name: CensusValue { value: None, last_updated: None },
            faction_id: CensusValue { value: None, last_updated: None },
            head_id: CensusValue { value: None, last_updated: None },
            title_id: CensusValue { value: None, last_updated: None },
            created_at: CensusValue { value: None, last_updated: None },
            last_updated: CensusValue { value: None, last_updated: None },
            last_login: CensusValue { value: None, last_updated: None },
            login_count: CensusValue { value: None, last_updated: None },
            minutes_played: CensusValue { value: None, last_updated: None },
            certs_earned: CensusValue { value: None, last_updated: None },
            certs_gifted: CensusValue { value: None, last_updated: None },
            certs_spent: CensusValue { value: None, last_updated: None },
            certs_available: CensusValue { value: None, last_updated: None },
            certs_progress: CensusValue { value: None, last_updated: None },
            battle_rank: CensusValue { value: None, last_updated: None },
            battle_rank_progress: CensusValue { value: None, last_updated: None },
            profile_id: CensusValue { value: None, last_updated: None },
            daily_ribbon_count: CensusValue { value: None, last_updated: None },
            daily_ribbon_time: CensusValue { value: None, last_updated: None },
            is_asp: CensusValue { value: None, last_updated: None },
        }
    }

    pub async fn fetch_resolves(&mut self, resolves: Option<Vec<CharacterResolves>> ) -> Result<(),CensusError> {
        let mut query = self.owning_client.get_query_builder("character");

        if let Some(resolves) = resolves {
            for r in resolves {
                query.resolve(&r.to_resolve_string());
            }
        }

        query.limit(1);

        query.search("character_id".to_string(), self.id.clone());

        let char = query.get().await;

        match char {
            Err(err) => return Err(err),
            Ok(jsonval) => {
                let jsonchar = jsonval["character_list"][0].clone();

                self.update(&jsonchar);

                return Ok(());

            }
        }

    }

    /// Creates a character and prefetches the given list of resolves
    pub async fn new_prefeched(
        rest_client: Arc<RestClient>,
        id: String,
        resolves: Option<Vec<CharacterResolves>>,
    ) -> Result<Self, CensusError> {
        let mut char = Character::new(id,rest_client);

        char.fetch_resolves(resolves).await?;

        return Ok(char);

    }

    fn from_json_value(json: &Value, rest_client: Arc<RestClient>) -> Result<Self, CensusError> {

        let id_v = &json["character_id"];

        if !id_v.is_string() {
            return Err(CensusError {
                err_msg: "Could not get character id".to_string(),
                parent_err: None,
            });
        }

        let mut char = Character::new(id_v.to_string(),rest_client);

        char.update(json);

        return Ok(char);
    }

    //pub async fn get_id(&self) -> String {
    //
    //}
}
