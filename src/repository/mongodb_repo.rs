use std::env;
// extern crate dotenv indicates that you want to link against 
// an external library and brings the top-level crate name into scope (equivalent to use foo).
// use dotenv is a shorthand for referencing fully-qualified symbols
extern crate dotenv;
use dotenv::dotenv;

use mongodb::{
    bson::{extjson::de::Error, oid::ObjectId, doc}, //modify here
    results::{InsertOneResult},
    sync::{Client, Collection},
};
use crate::models::user_model::User;

pub struct MongoRepo {
    col: Collection<User>,
}
// The impl keyword is primarily used to define implementations on types. 
// Inherent implementations are standalone, while trait implementations are used to 
// implement traits for types, or other traits.
impl MongoRepo {

    pub fn init() -> Self {
        dotenv().ok();
        let uri = match env::var("DATABASE_URL") {
            Ok(v) => v.to_string(),
            Err(_) => format!("Error loading env variable"),
        };
        let client = Client::with_uri_str(uri).unwrap();
        let db = client.database("rust");
        let col: Collection<User> = db.collection("User");
        MongoRepo { col }
    }

    pub fn create_user(&self, new_user: User) -> Result<InsertOneResult, Error> {
        let new_doc = User {
            id: None,
            name: new_user.name,
            location: new_user.location,
            title: new_user.title,
        };
        let user = self
            .col
            .insert_one(new_doc, None)
            .ok()
            .expect("Error creating user");
        Ok(user)
    }

    pub fn get_user(&self, id: &String) -> Result<User, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let user_detail = self
            .col
            .find_one(filter, None)
            .ok()
            .expect("Error getting user's detail");
        Ok(user_detail.unwrap())
    }
}