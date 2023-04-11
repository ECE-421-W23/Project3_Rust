use std::cmp::{max, min};

use serde::{Deserialize, Serialize};

#[derive(Clone,Debug, Serialize, Deserialize)]
pub struct Game {
	pub gametype: String,
    pub player1: String,
    pub player2: String,
	pub winner: String,
	pub date: String,
}