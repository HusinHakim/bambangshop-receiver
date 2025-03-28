use rocket::serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct SubscriberRequest {
    pub url: String,
    pub name: String,
}

