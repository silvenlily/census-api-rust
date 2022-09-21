use std::sync::Arc;

use super::RestClient;

#[derive(Clone)]
pub struct Item {
    owning_client: Arc<RestClient>,
    pub id: u128,
    stack_count: u64,
}
