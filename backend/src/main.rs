use mongodb::options::{UpdateOptions,IndexOptions, ClientOptions};
use mongodb::sync::{Client, Database};
use mongodb::bson::{doc, Document, Bson};
use mongodb::results::{UpdateResult,InsertOneResult};
use mongodb::IndexModel;
use std::env;
use std::error::Error;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug)]
pub struct DB {
	database: Database,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Game {
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
	pub fn insert_user(&self, collection: &str) -> Result<UpdateResult, Box<dyn Error>> {
		// List the names of the collections in that database.
		let coll = self.database.collection::<String>("Users");

		let doc = doc! { "name": "John" };
		let options = UpdateOptions::builder()
			.upsert(Some(true))
			.build();
		let update = doc! { "$inc": { "id": 1 }};
		Ok(coll.update_one(doc, update,options).unwrap())
	}

	pub fn get_users(&self) -> Result<(), Box<dyn Error>> {
		// List the names of the collections in that database.
		let mut cursor = self.database.collection("Users").find(None, None)?;
		// regular Stream uses next() and iterates over Option<Result<T>>
		while let Some(doc) = cursor.next() {
			let ss: Document = doc?;
			println!("Yes");
			println!("{:#?}", ss);
		}
        Ok(())
	}

	pub fn insert_game(&self, player1: &str, player2: &str, winner: &str) -> Result<InsertOneResult, Box<dyn Error>> {
		// List the names of the collections in that database.
		let coll = self.database.collection::<Game>("Games");
		let game = Game{
			player1: player1.to_string(),
			player2: player2.to_string(),
			winner: winner.to_string(),
			date: "t".to_string(),
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
			player1: doc.get_str("player1")?.to_string(),
			player2: doc.get_str("player2")?.to_string(),
			winner: doc.get_str("winner")?.to_string(),
			date: doc.get_str("date")?.to_string(),
		};

		Ok(result)
	}
}

fn main() {
	let db = DB::new().unwrap();
	//db.insert_user("user2");
	//db.get_users();
	println!("{:#?}",db.insert_game("user3","user3","user3"));
	println!("{:#?}", db.get_games());
}
