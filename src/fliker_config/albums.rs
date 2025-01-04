use serde::{Deserialize, Serialize};
use crate::fliker_config::utils::string_to_i32;

#[derive(Serialize, Deserialize, Debug)]
pub struct Album {
    #[serde(deserialize_with = "string_to_i32")]
    photo_count: i32,
    id: String,
    url: String,
    title: String,
    description: String,
    view_count: String,
    created: String,
    last_updated: String,
    cover_photo: String,
    photos: Vec<String>,
}