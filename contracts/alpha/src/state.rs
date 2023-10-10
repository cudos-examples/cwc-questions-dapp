use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::Addr;
use cw_storage_plus::Item;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    // pub count: i32,
    pub questions: Vec<String>,
    pub owner: Addr,
}

// pub struct Question {
//
// }
//
pub const STATE: Item<State> = Item::new("state");

