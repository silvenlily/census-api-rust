pub mod api_subscription_names {
    pub static PLAYER_LOGIN: &'static str = "PlayerLogin";
    pub static PLAYER_LOGOUT: &'static str = "PlayerLogout";
    pub static CONTINENT_LOCK: &'static str = "ContinentLock";
    pub static CONTINENT_UNLOCK: &'static str = "ContinentUnlock";
    pub static FACILITY_CONTROL: &'static str = "FacilityControl";
    pub static METAGAME_EVENT: &'static str = "MetagameEvent";
    pub static ACHIEVEMENT_EARNED: &'static str = "AchievementEarned";
    pub static BATTLE_RANK_UP: &'static str = "BattleRankUp";
    pub static DEATH: &'static str = "Death";
    pub static ITEM_ADDED: &'static str = "ItemAdded";
    pub static SKILL_ADDED: &'static str = "SkillAdded";
    pub static VEHICLE_DESTROY: &'static str = "VehicleDestroy";
    pub static GAIN_EXPERIENCE: &'static str = "GainExperience";
    pub static PLAYER_FACILITY_CAPTURE: &'static str = "PlayerFacilityCapture";
    pub static PLAYER_FACILITY_DEFEND: &'static str = "PlayerFacilityDefend";
}

pub enum ApiSubscriptionName {
    PlayerLogin,
    PlayerLogout,
    ContinentLock,
    ContinentUnlock,
    FacilityControl,
    MetagameEvent,
    AchievementEarned,
    BattleRankUp,
    Death,
    ItemAdded,
    SkillAdded,
    VehicleDestroy,
    GainExperience,
    PlayerFacilityCapture,
    PlayerFacilityDefend,
}

impl ApiSubscriptionName {
    pub fn str(&self) -> &'static str {
        match self {
            ApiSubscriptionName::PlayerLogin => api_subscription_names::PLAYER_LOGIN,
            ApiSubscriptionName::PlayerLogout => api_subscription_names::PLAYER_LOGOUT,
            ApiSubscriptionName::ContinentLock => api_subscription_names::CONTINENT_LOCK,
            ApiSubscriptionName::ContinentUnlock => api_subscription_names::CONTINENT_UNLOCK,
            ApiSubscriptionName::FacilityControl => api_subscription_names::FACILITY_CONTROL,
            ApiSubscriptionName::MetagameEvent => api_subscription_names::METAGAME_EVENT,
            ApiSubscriptionName::AchievementEarned => api_subscription_names::ACHIEVEMENT_EARNED,
            ApiSubscriptionName::BattleRankUp => api_subscription_names::BATTLE_RANK_UP,
            ApiSubscriptionName::Death => api_subscription_names::DEATH,
            ApiSubscriptionName::ItemAdded => api_subscription_names::ITEM_ADDED,
            ApiSubscriptionName::SkillAdded => api_subscription_names::SKILL_ADDED,
            ApiSubscriptionName::VehicleDestroy => api_subscription_names::VEHICLE_DESTROY,
            ApiSubscriptionName::GainExperience => api_subscription_names::GAIN_EXPERIENCE,
            ApiSubscriptionName::PlayerFacilityCapture => {
                api_subscription_names::PLAYER_FACILITY_CAPTURE
            }
            ApiSubscriptionName::PlayerFacilityDefend => {
                api_subscription_names::PLAYER_FACILITY_DEFEND
            }
        }
    }
}

pub enum ApiEvent {
    ServiceStateChange(super::ServiceStateChange),
    ConnectionStateChange(super::ConnectionStateChange),
    Help,
    PlayerLogin(super::PlayerLogin),
    PlayerLogout(super::PlayerLogout),
    ContinentLock(super::ContinentLock),
    ContinentUnlock(super::ContinentUnlock),
    FacilityControl(super::FacilityControl),
    MetagameEvent(super::MetagameEvent),
    AchievementEarned(super::AchievementEarned),
    BattleRankUp(super::BattleRankUp),
    Death(super::Death),
    ItemAdded(super::ItemAdded),
    SkillAdded(super::SkillAdded),
    VehicleDestroy(super::VehicleDestroy),
    GainExperience(super::GainExperience),
    PlayerFacilityCapture(super::PlayerFacilityCapture),
    PlayerFacilityDefend(super::PlayerFacilityDefend),
}

impl ApiEvent {
    pub fn to_type(self) -> ApiEventTypes {
        match self {
            ApiEvent::PlayerLogin(e) => {
                return ApiEventTypes::Connection(ApiConnectionEvents::PlayerLogin(e));
            }
            ApiEvent::PlayerLogout(e) => {
                return ApiEventTypes::Connection(ApiConnectionEvents::PlayerLogout(e));
            }
            ApiEvent::ContinentLock(e) => {
                return ApiEventTypes::World(ApiWorldEvents::ContinentLock(e));
            }
            ApiEvent::ContinentUnlock(e) => {
                return ApiEventTypes::World(ApiWorldEvents::ContinentUnlock(e));
            }
            ApiEvent::FacilityControl(e) => {
                return ApiEventTypes::World(ApiWorldEvents::FacilityControl(e));
            }
            ApiEvent::MetagameEvent(e) => {
                return ApiEventTypes::World(ApiWorldEvents::MetagameEvent(e));
            }
            ApiEvent::AchievementEarned(e) => {
                return ApiEventTypes::Character(ApiCharacterEvents::AchievementEarned(e));
            }
            ApiEvent::BattleRankUp(e) => {
                return ApiEventTypes::Character(ApiCharacterEvents::BattleRankUp(e));
            }
            ApiEvent::Death(e) => {
                return ApiEventTypes::Character(ApiCharacterEvents::Death(e));
            }
            ApiEvent::ItemAdded(e) => {
                return ApiEventTypes::Character(ApiCharacterEvents::ItemAdded(e));
            }
            ApiEvent::SkillAdded(e) => {
                return ApiEventTypes::Character(ApiCharacterEvents::SkillAdded(e));
            }
            ApiEvent::VehicleDestroy(e) => {
                return ApiEventTypes::Character(ApiCharacterEvents::VehicleDestroy(e));
            }
            ApiEvent::GainExperience(e) => {
                return ApiEventTypes::Character(ApiCharacterEvents::GainExperience(e));
            }
            ApiEvent::PlayerFacilityCapture(e) => {
                return ApiEventTypes::Character(ApiCharacterEvents::PlayerFacilityCapture(e));
            }
            ApiEvent::PlayerFacilityDefend(e) => {
                return ApiEventTypes::Character(ApiCharacterEvents::PlayerFacilityDefend(e));
            }
            ApiEvent::ServiceStateChange(e) => {
                return ApiEventTypes::Status(ApiStatusEvents::ServiceStateChange(e));
            }
            ApiEvent::ConnectionStateChange(e) => {
                return ApiEventTypes::Status(ApiStatusEvents::ConnectionStateChange(e));
            }
            ApiEvent::Help => {
                return ApiEventTypes::Status(ApiStatusEvents::Help);
            }
        }
    }
}

pub enum ApiEventTypes {
    Status(ApiStatusEvents),
    Connection(ApiConnectionEvents),
    World(ApiWorldEvents),
    Character(ApiCharacterEvents),
}

pub enum ApiStatusEvents {
    ServiceStateChange(super::ServiceStateChange),
    ConnectionStateChange(super::ConnectionStateChange),
    Help,
}

pub enum ApiConnectionEvents {
    PlayerLogin(super::PlayerLogin),
    PlayerLogout(super::PlayerLogout),
}

pub enum ApiWorldEvents {
    ContinentLock(super::ContinentLock),
    ContinentUnlock(super::ContinentUnlock),
    FacilityControl(super::FacilityControl),
    MetagameEvent(super::MetagameEvent),
}

pub enum ApiCharacterEvents {
    AchievementEarned(super::AchievementEarned),
    BattleRankUp(super::BattleRankUp),
    Death(super::Death),
    ItemAdded(super::ItemAdded),
    SkillAdded(super::SkillAdded),
    VehicleDestroy(super::VehicleDestroy),
    GainExperience(super::GainExperience),
    PlayerFacilityCapture(super::PlayerFacilityCapture),
    PlayerFacilityDefend(super::PlayerFacilityDefend),
}
