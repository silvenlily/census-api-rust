use serde_json::Value;

use crate::events::api_events::parse_helpers::*;
use crate::utils::CensusError;

pub mod event_types;
mod parse_helpers;

// Status Events
#[derive(Debug)]
pub struct ServiceStateChange {
    pub endpoint: String,
    pub online: bool,
}

#[derive(Debug)]
pub struct ConnectionStateChange {
    pub connected: bool,
}

// Player Connect/Disconnect Events
#[derive(Debug)]
pub struct PlayerLogin {
    pub character_id: String,
    pub world_id: u8,
    pub timestamp: String,
}

impl Event for PlayerLogin {
    fn from_json(json: Value) -> Result<Self, CensusError> {
        return Ok(Self {
            character_id: parse_character_id(&json)?,
            timestamp: parse_timestamp(&json)?,
            world_id: parse_world_id(&json)?,
        });
    }
}

#[derive(Debug)]
pub struct PlayerLogout {
    pub character_id: String,
    pub world_id: u8,
    pub timestamp: String,
}

impl Event for PlayerLogout {
    fn from_json(json: Value) -> Result<Self, CensusError> {
        return Ok(Self {
            character_id: parse_character_id(&json)?,
            timestamp: parse_timestamp(&json)?,
            world_id: parse_world_id(&json)?,
        });
    }
}

// World Events
#[derive(Debug)]
pub struct ContinentLock {
    pub timestamp: String,
    pub world_id: u8,
    pub zone_id: u32,
    pub triggering_faction: String,
    pub previous_faction: String,
    pub vs_population: String,
    pub nc_population: String,
    pub tr_population: String,
    pub metagame_event_id: String,
    pub event_type: String,
}

impl Event for ContinentLock {
    fn from_json(json: Value) -> Result<Self, CensusError> {
        return Ok(Self {
            timestamp: parse_timestamp(&json)?,
            world_id: parse_world_id(&json)?,
            zone_id: parse_zone_id(&json)?,
            triggering_faction: parse_string("triggering_faction", &json)?,
            previous_faction: parse_string("previous_faction", &json)?,
            vs_population: parse_string("vs_population", &json)?,
            nc_population: parse_string("nc_population", &json)?,
            tr_population: parse_string("tr_population", &json)?,
            metagame_event_id: parse_string("metagame_event_id", &json)?,
            event_type: parse_string("event_type", &json)?,
        });
    }
}

#[derive(Debug)]
pub struct ContinentUnlock {
    pub timestamp: String,
    pub world_id: u8,
    pub zone_id: u32,
    pub triggering_faction: String,
    pub previous_faction: String,
    pub vs_population: String,
    pub nc_population: String,
    pub tr_population: String,
    pub metagame_event_id: String,
    pub event_type: String,
}

impl Event for ContinentUnlock {
    fn from_json(json: Value) -> Result<Self, CensusError> {
        return Ok(Self {
            timestamp: parse_timestamp(&json)?,
            world_id: parse_world_id(&json)?,
            zone_id: parse_zone_id(&json)?,
            triggering_faction: parse_string("triggering_faction", &json)?,
            previous_faction: parse_string("previous_faction", &json)?,
            vs_population: parse_string("vs_population", &json)?,
            nc_population: parse_string("nc_population", &json)?,
            tr_population: parse_string("tr_population", &json)?,
            metagame_event_id: parse_string("metagame_event_id", &json)?,
            event_type: parse_string("event_type", &json)?,
        });
    }
}

#[derive(Debug)]
pub struct FacilityControl {
    pub timestamp: String,
    pub world_id: u8,
    pub zone_id: u32,
    pub facility_id: String,
    pub new_faction_id: u8,
    pub old_faction_id: u8,
    pub outfit_id: String,
    pub duration_held: String,
}

impl Event for FacilityControl {
    fn from_json(json: Value) -> Result<Self, CensusError> {
        return Ok(Self {
            timestamp: parse_timestamp(&json)?,
            world_id: parse_world_id(&json)?,
            zone_id: parse_zone_id(&json)?,
            facility_id: parse_string("facility_id", &json)?,
            new_faction_id: try_parse_to("new_faction_id", &json)?,
            old_faction_id: try_parse_to("old_faction_id", &json)?,
            outfit_id: parse_string("outfit_id", &json)?,
            duration_held: parse_string("duration_held", &json)?,
        });
    }
}

#[derive(Debug)]
pub struct MetagameEvent {
    pub timestamp: String,
    pub world_id: u8,
    pub zone_id: u32,
    pub experience_bonus: String,
    pub faction_nc: String,
    pub faction_tr: String,
    pub faction_vs: String,
    pub metagame_event_id: String,
    pub metagame_event_state: String,
}

impl Event for MetagameEvent {
    fn from_json(json: Value) -> Result<Self, CensusError> {
        return Ok(Self {
            timestamp: parse_timestamp(&json)?,
            world_id: parse_world_id(&json)?,
            zone_id: parse_zone_id(&json)?,
            experience_bonus: parse_string("experience_bonus", &json)?,
            faction_nc: parse_string("faction_nc", &json)?,
            faction_tr: parse_string("faction_tr", &json)?,
            faction_vs: parse_string("faction_vs", &json)?,
            metagame_event_id: parse_string("metagame_event_id", &json)?,
            metagame_event_state: parse_string("metagame_event_state", &json)?,
        });
    }
}

#[derive(Debug)]
pub struct AchievementEarned {
    pub character_id: String,
    pub timestamp: String,
    pub world_id: u8,
    pub achievment_id: u64,
    pub zone_id: u32,
}

#[derive(Debug)]
pub struct BattleRankUp {
    pub character_id: String,
    pub timestamp: String,
    pub world_id: u8,
    pub zone_id: u32,
    pub battle_rank: u8,
}

#[derive(Debug)]
pub struct Death {
    pub attacker_character_id: String,
    pub attacker_fire_mod_id: String,
    pub attacker_loadout_id: String,
    pub attacker_vehicle_id: String,
    pub attacker_weapon_id: String,
    pub character_id: String,
    pub character_loadout_id: String,
    pub is_critical: bool,
    pub is_headshot: bool,
    pub timestamp: String,
    pub vehicle_id: String,
    pub world_id: u8,
    pub zone_id: u32,
}

#[derive(Debug)]
pub struct ItemAdded {
    pub character_id: String,
    pub context: String,
    pub item_count: u64,
    pub item_id: String,
    pub timestamp: String,
    pub world_id: u8,
    pub zone_id: u32,
}

#[derive(Debug)]
pub struct SkillAdded {
    pub character_id: String,
    pub skill_id: String,
    pub timestamp: String,
    pub world_id: u8,
    pub zone_id: u32,
}

#[derive(Debug)]
pub struct VehicleDestroy {
    pub attacker_character_id: String,
    pub attacker_loadout_id: String,
    pub attacker_vehicle_id: String,
    pub attacker_weapon_id: String,
    pub character_id: String,
    pub facility_id: String,
    pub faction_id: u8,
    pub timestamp: String,
    pub vehicle_id: String,
    pub world_id: u8,
    pub zone_id: u32,
}

#[derive(Debug)]
pub struct GainExperience {
    pub character_id: String,
    pub timestamp: String,
    pub world_id: u8,
    pub zone_id: u32,
    pub amount: String,
    pub experience_id: String,
    pub loudout_id: String,
    pub other_id: String,
}

#[derive(Debug)]
pub struct PlayerFacilityCapture {
    pub character_id: String,
    pub timestamp: String,
    pub world_id: u8,
    pub zone_id: u32,
    pub facility_id: String,
    pub outfit_id: String,
}

#[derive(Debug)]
pub struct PlayerFacilityDefend {
    pub character_id: String,
    pub timestamp: String,
    pub world_id: u8,
    pub zone_id: u32,
    pub facility_id: String,
    pub outfit_id: String,
}

pub trait Event: Sized {
    fn from_json(json: Value) -> Result<Self, CensusError>;
}
