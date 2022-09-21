use std::{sync::Arc, time::SystemTime};

use super::{query_builder::Resolveable, CensusValue, RestClient};

enum OutfitResolves {
    Leader,
    LeaderName,
    LeadersStatHistory,
    Member,
    Rank,
    Membercharacter,
    MemberCharacterName,
    MembercharactersStatHistory,
    MemberOnlineStatus,
}

impl Resolveable for OutfitResolves {
    fn from_resolve_string(resolve: &str) -> Option<Self>
    where
        Self: Sized,
    {
        match resolve {
            "leader" => Some(OutfitResolves::Leader),
            "leader_name" => Some(OutfitResolves::LeaderName),
            "leaders_stat_history" => Some(OutfitResolves::LeadersStatHistory),
            "member" => Some(OutfitResolves::Member),
            "rank" => Some(OutfitResolves::Rank),
            "member_character" => Some(OutfitResolves::Membercharacter),
            "memer_character_name" => Some(OutfitResolves::MemberCharacterName),
            "member_character_stat_history" => Some(OutfitResolves::MembercharactersStatHistory),
            "member_online_status" => Some(OutfitResolves::MemberOnlineStatus),
            _ => None,
        }
    }

    fn to_resolve_string(&self) -> String {
        match self {
            OutfitResolves::Leader => "leader".to_string(),
            OutfitResolves::LeaderName => "leader_name".to_string(),
            OutfitResolves::LeadersStatHistory => "leaders_stat_history".to_string(),
            OutfitResolves::Member => "member".to_string(),
            OutfitResolves::Rank => "rank".to_string(),
            OutfitResolves::Membercharacter => "member_character".to_string(),
            OutfitResolves::MemberCharacterName => "member_character_name".to_string(),
            OutfitResolves::MembercharactersStatHistory => {
                "member_characters_stat_history".to_string()
            }
            OutfitResolves::MemberOnlineStatus => "member_online_status".to_string(),
        }
    }
}

#[derive(Clone)]
pub struct Outfit {
    owning_client: Arc<RestClient>,
    // resolved by default
    pub id: String,
    name: CensusValue<String>,
    tag: CensusValue<String>,
    created_at: CensusValue<SystemTime>,
    leader_character_id: CensusValue<String>,
    member_count: u64,
}
