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
    #[serde(deserialize_with = "string_to_i32")]
    view_count: i32,
    created: String,
    last_updated: String,
    cover_photo: String,
    photos: Vec<String>,
}
#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_album_deserialization() {
        let data = json!({
            "photo_count": "10",
            "id": "album1",
            "url": "http://example.com/album1",
            "title": "My Album",
            "description": "This is a test album",
            "view_count": "100",
            "created": "2023-01-01",
            "last_updated": "2023-01-02",
            "cover_photo": "photo1",
            "photos": ["photo1", "photo2"]
        });

        let album: Album = serde_json::from_value(data).expect("Deserialization failed");

        assert_eq!(album.photo_count, 10);
        assert_eq!(album.id, "album1");
        assert_eq!(album.url, "http://example.com/album1");
        assert_eq!(album.title, "My Album");
        assert_eq!(album.description, "This is a test album");
        assert_eq!(album.view_count, 100);
        assert_eq!(album.created, "2023-01-01");
        assert_eq!(album.last_updated, "2023-01-02");
        assert_eq!(album.cover_photo, "photo1");
        assert_eq!(album.photos, vec!["photo1", "photo2"]);
    }
}
