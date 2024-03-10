use std::collections::HashMap;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct Performance {
    #[serde(alias = "playID")]
    pub(crate) play_id: String,
    pub(crate) audience: u64,
}

#[derive(Deserialize)]
pub struct Invoice {
    pub(crate) customer: String,
    pub(crate) performances: Vec<Performance>,
}

#[derive(Deserialize)]
pub struct Play {
    pub(crate) name: String,
    #[serde(alias = "type")]
    pub(crate) p_type: String,
}

pub type Plays = HashMap<String, Play>;
