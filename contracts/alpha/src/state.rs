use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::Addr;
use cw_storage_plus::Map;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Question {
    pub questions: String,
    pub handle: String,
    pub owner: Addr
}

pub const STATE: Map<&str, Question> = Map::new("state");

