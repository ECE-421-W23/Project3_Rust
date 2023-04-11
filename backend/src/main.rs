#[macro_use]
extern crate rocket;
use rocket::{serde::json::Json, State};

use mongodb::options::ClientOptions;
use mongodb::sync::{Client, Database};
use mongodb::bson::{doc, Document};
use mongodb::results::InsertOneResult;
use std::error::Error;
use serde::{Deserialize, Serialize};
use rocket::http::Header;
use rocket::{Request, Response};
use rocket::fairing::{Fairing, Info, Kind};
use chrono::prelude::*;

#[derive(Clone, Debug)]
pub struct DB {
	database: Database,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Game {
	pub gametype: String,
    pub player1: String,
    pub player2: String,
	pub winner: String,
	pub date: String,
}

impl DB {
	pub fn new() -> Result<Self, Box<dyn Error>>{
		let client_options = ClientOptions::parse("mongodb://localhost:27017").unwrap();
		let client = Client::with_options(client_options).unwrap();
		let db = client.database("Project3");
		Ok(Self {
			database: db,
		})
	}

	pub fn insert_game(&self, gametype: &str, player1: &str, player2: &str, winner: &str) -> Result<InsertOneResult, Box<dyn Error>> {
		// List the names of the collections in that database.
		let coll = self.database.collection::<Game>("Games");
		let now: DateTime<Utc> = Utc::now();
		let game = Game{
			gametype: gametype.to_string(),
			player1: player1.to_string(),
			player2: player2.to_string(),
			winner: winner.to_string(),
			date: format!("{}", now.format("%I:%M%p on %b %d, %Y")),
		};
		Ok(coll.insert_one(game, None).unwrap())
	}

	pub fn get_games(&self) -> Result<Vec<Game>, Box<dyn Error>> {
		// List the names of the collections in that database.
		let mut cursor = self.database.collection("Games").find(None, None)?;
		// regular Stream uses next() and iterates over Option<Result<T>>
		let mut result: Vec<Game> = Vec::new();
		while let Some(doc) = cursor.next() {
			result.push(self.doc_to_game(&doc?)?)
		}
        Ok(result)
	}

	fn doc_to_game(&self, doc: &Document) -> Result<Game, Box<dyn Error>> {
		let result = Game{
			gametype: doc.get_str("gametype")?.to_string(),
			player1: doc.get_str("player1")?.to_string(),
			player2: doc.get_str("player2")?.to_string(),
			winner: doc.get_str("winner")?.to_string(),
			date: doc.get_str("date")?.to_string(),
		};

		Ok(result)
	}
}

#[get("/games")]
fn get_games(db: &State<DB>) -> Json<Vec<Game>> {
	let result = db.get_games();
	match result {
		Ok(v) => Json(v),
		_=> Json(Vec::new()),
	}
}

#[post("/games", data="<game>")]
fn add_games(game: Json<Game>, db: &State<DB>) {
    db.insert_game(&game.gametype,&game.player1,&game.player2,&game.winner);
}

pub struct CORS;
#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new("Access-Control-Allow-Methods", "POST, GET, PATCH, OPTIONS"));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

#[launch]
fn rocket() -> _ {
	let db = DB::new().unwrap();
    rocket::build()
	.manage(db)
	.mount("/", routes![get_games])
	.mount("/post", routes![add_games])
	.attach(CORS)
}