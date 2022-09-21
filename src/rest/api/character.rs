use serde::Deserialize;

#[derive(Deserialize)]
pub enum CharListIndex {
    character(u64),
    return_count(String),
}

#[derive(Deserialize)]
pub enum CharListVal {
    character(ApiCharacter),
    return_count(u64),
}

#[derive(Deserialize)]
pub struct ApiCharacterTimes {
    pub creation: String,
    pub creation_date: String,
    pub last_save: String,
    pub last_save_date: String,
    pub last_login: String,
    pub last_login_date: String,
    pub login_count: String,
    pub minutes_played: String,
}

#[derive(Deserialize)]
pub struct ApiCharacterName {
    pub name: String,
    pub name_lower: String,
}

#[derive(Deserialize)]
pub struct ApiCharacterCurrency {
    pub earned_points: String,
    pub gifted_points: String,
    pub spent_points: String,
    pub available_points: String,
    pub percent_to_next: String,
}

#[derive(Deserialize)]
pub struct ApiCharacterBR {
    pub percent_to_next: String,
    pub value: String,
}

#[derive(Deserialize)]
pub struct ApiCharacterDailyRibbon {
    pub count: String,
    pub time: String,
    pub date: String,
}

#[derive(Deserialize)]
pub struct ApiCharacter {
    pub character_id: String,
    pub name: ApiCharacterName,
    pub faction_id: String,
    pub head_id: String,
    pub title_id: String,
    pub profile_id: String,
    pub prestige_level: String,
    pub times: ApiCharacterTimes,
    pub certs: ApiCharacterCurrency,
    pub battle_rank: ApiCharacterBR,
    pub daily_ribbon: ApiCharacterDailyRibbon,
}
