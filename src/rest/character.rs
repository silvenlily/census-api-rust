use std::{sync::Arc, time::SystemTime};

use serde_json::Value;

use crate::utils::CensusError;

use super::{item::Item, query_builder::Resolveable, CensusValue, RestClient};

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
#[derive(Clone)]
pub struct Character {
    owning_client: Arc<RestClient>,
    pub id: String,
    name: CensusValue<String>,
    faction_id: CensusValue<u8>,
    head_id: CensusValue<u16>,
    title_id: CensusValue<u16>,
    created_at: CensusValue<SystemTime>,
    last_updated: CensusValue<SystemTime>,
    last_login: CensusValue<SystemTime>,
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
    daily_ribbon_time: CensusValue<SystemTime>,
    is_asp: CensusValue<bool>,
    // resolved by item or item_full
    items: CensusValue<Vec<Item>>,
    // resolved by profile
    class: CensusValue<CharacterClass>,
    //
}

impl Character {
    fn from_json_value(
        json_value: &Value,
        rest_client: &Arc<RestClient>,
    ) -> Result<Self, CensusError> {
        todo!()
    }

    pub async fn new(id: String) -> Self {
        //return Character {}
        todo!()
    }

    /// Creates a character and prefetches the given list of resolves
    pub async fn new_prefeched(
        rest_client: Arc<RestClient>,
        id: String,
        resolves: Option<Vec<CharacterResolves>>,
    ) -> Result<Self, CensusError> {
        let mut query = rest_client.get_query_builder("character");

        match resolves {
            Some(resolves) => {
                for r in resolves {
                    query.resolve(&r.to_resolve_string());
                }
            }
            None => {}
        }

        query.limit(1);

        query.search("character_id".to_string(), id);

        let char = query.get().await;

        match char {
            Err(err) => return Err(err),
            Ok(jsonval) => {
                let jsonchar = jsonval["character_list"][0].clone();

                let char_w = Character::from_json_value(&jsonchar, &rest_client);

                match char_w {
                    Ok(char) => return Ok(char),
                    Err(err) => return Err(err),
                }
            }
        }
    }

    /// Gets characters by a vec of ids with the included resolves.
    ///
    /// For vecs of ids larger then 64 ids will be divided into blocks of 64 and a separate request will be sent for each block.
    pub async fn from_ids_with_resolves(
        rest_client: Arc<RestClient>,
        ids: Vec<String>,
        resolves: Option<Vec<CharacterResolves>>,
    ) -> Result<Vec<Result<Self, CensusError>>, CensusError> {
        let mut query = rest_client.get_query_builder("character");

        let chunked_ids = ids.chunks(64);

        let mut chars: Vec<Result<Self, CensusError>> = Vec::with_capacity(ids.len());

        for idblock in chunked_ids {
            match resolves {
                Some(resolves) => {
                    for r in resolves {
                        query.resolve(&r.to_resolve_string());
                    }
                }
                None => {}
            }

            query.limit(idblock.len().try_into().unwrap());

            let idsearch: String = idblock.join(",");

            query.search("character_id".to_string(), idsearch);

            match query.get().await {
                Err(err) => return Err(err),
                Ok(jsonval) => {
                    let jsonchars = jsonval["character_list"].clone();

                    let mut char_index: u16 = 0;
                    loop {
                        let nextchar = jsonchars[char_index.to_string()].clone();

                        if nextchar.is_null() {
                            break;
                        }

                        char_index = char_index + 1;

                        let char_w = Character::from_json_value(&nextchar, &rest_client);

                        match char_w {
                            Ok(char) => {
                                chars.push(Ok(char));
                            }
                            Err(err) => {
                                chars.push(Err(err));
                            }
                        }
                    }

                    return Ok(chars);
                }
            }
        }

        return Ok(chars);
    }
}
