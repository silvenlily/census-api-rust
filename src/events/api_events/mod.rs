pub mod event_types;

// Status Events
pub struct ServiceStateChange {
    pub endpoint: String,
    pub online: bool,
}
pub struct ConnectionStateChange {
    pub connected: bool,
}

// Player Connect/Disconnect Events
pub struct PlayerLogin {}
pub struct PlayerLogout {}

// World Events
pub struct ContinentLock {
    pub event_name: String,
    pub timestamp: String,
    pub world_id: String,
    pub zone_id: String,
    pub triggering_faction: String,
    pub previous_faction: String,
    pub vs_population: String,
    pub nc_population: String,
    pub tr_population: String,
    pub metagame_event_id: String,
    pub event_type: String,
}
pub struct ContinentUnlock {
    pub event_name: String,
    pub timestamp: String,
    pub world_id: String,
    pub zone_id: String,
    pub triggering_faction: String,
    pub previous_faction: String,
    pub vs_population: String,
    pub nc_population: String,
    pub tr_population: String,
    pub metagame_event_id: String,
    pub event_type: String,
}
pub struct FacilityControl {
    pub event_name: String,
    pub timestamp: String,
    pub world_id: String,
    pub zone_id: String,
    pub triggering_faction: String,
    pub previous_faction: String,
    pub vs_population: String,
    pub nc_population: String,
    pub tr_population: String,
    pub metagame_event_id: String,
    pub event_type: String,
}
pub struct MetagameEvent {
    pub event_name: String,
    pub timestamp: String,
    pub world_id: String,
    pub zone_id: String,
    pub experience_bonus: String,
    pub faction_nc: String,
    pub faction_tr: String,
    pub faction_vs: String,
    pub metagame_event_id: String,
    pub metagame_event_state: String,
}
pub struct AchievementEarned {}
pub struct BattleRankUp {}
pub struct Death {}
pub struct ItemAdded {}
pub struct SkillAdded {}
pub struct VehicleDestroy {}
pub struct GainExperience {}
pub struct PlayerFacilityCapture {}
pub struct PlayerFacilityDefend {}

trait Event {
    fn get_event_type(&self) -> String;
}

trait CharacterEvent {
    fn get_character_id(&self) -> String;
}
