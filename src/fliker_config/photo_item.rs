use serde::{Deserialize, Serialize};
use crate::fliker_config::utils::string_to_i32;

#[derive(Serialize, Deserialize, Debug)]
pub struct Geo {
    latitude: String,
    longitude: String,
    accuracy: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Tag {
    tag: String,
    user: String,
    date_create: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PhotoItem {
    id: String,
    name: String,
    description: String,
    #[serde(deserialize_with = "string_to_i32")]
    count_views: i32,
    #[serde(deserialize_with = "string_to_i32")]
    count_faves: i32,
    #[serde(deserialize_with = "string_to_i32")]
    count_comments: i32,
    date_taken: String,
    #[serde(deserialize_with = "string_to_i32")]
    count_tags: i32,
    #[serde(deserialize_with = "string_to_i32")]
    count_notes: i32,
    rotation: i32,
    date_imported: String,
    photopage: String,
    original: String,
    license: String,
    geo: Vec<Geo>,
    groups: Vec<String>,
    albums: Vec<String>,
    tags: Vec<Tag>,
    people: Vec<String>,
    notes: Vec<String>,
    privacy: String,
    comment_permissions: String,
    tagging_permissions: String,
    safety: String,
    comments: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_deserialize_photo_item() {
        let data = json!({
            "id": "1",
            "name": "Test Photo",
            "description": "A test photo",
            "count_views": "100",
            "count_faves": "50",
            "count_comments": "25",
            "date_taken": "2023-01-01",
            "count_tags": "10",
            "count_notes": "5",
            "rotation": 0,
            "date_imported": "2023-01-02",
            "photopage": "http://example.com/photo/1",
            "original": "http://example.com/photo/1/original",
            "license": "CC BY-SA",
            "geo": [
                {
                    "latitude": "40.7128",
                    "longitude": "-74.0060",
                    "accuracy": "16"
                }
            ],
            "groups": ["group1", "group2"],
            "albums": ["album1", "album2"],
            "tags": [
                {
                    "tag": "tag1",
                    "user": "user1",
                    "date_create": "2023-01-01"
                }
            ],
            "people": ["person1", "person2"],
            "notes": ["note1", "note2"],
            "privacy": "public",
            "comment_permissions": "everyone",
            "tagging_permissions": "friends",
            "safety": "safe",
            "comments": ["comment1", "comment2"]
        });

        let photo_item: PhotoItem = serde_json::from_value(data).unwrap();

        assert_eq!(photo_item.id, "1");
        assert_eq!(photo_item.name, "Test Photo");
        assert_eq!(photo_item.description, "A test photo");
        assert_eq!(photo_item.count_views, 100);
        assert_eq!(photo_item.count_faves, 50);
        assert_eq!(photo_item.count_comments, 25);
        assert_eq!(photo_item.date_taken, "2023-01-01");
        assert_eq!(photo_item.count_tags, 10);
        assert_eq!(photo_item.count_notes, 5);
        assert_eq!(photo_item.rotation, 0);
        assert_eq!(photo_item.date_imported, "2023-01-02");
        assert_eq!(photo_item.photopage, "http://example.com/photo/1");
        assert_eq!(photo_item.original, "http://example.com/photo/1/original");
        assert_eq!(photo_item.license, "CC BY-SA");
        assert_eq!(photo_item.geo.len(), 1);
        assert_eq!(photo_item.geo[0].latitude, "40.7128");
        assert_eq!(photo_item.geo[0].longitude, "-74.0060");
        assert_eq!(photo_item.geo[0].accuracy, "16");
        assert_eq!(photo_item.groups, vec!["group1", "group2"]);
        assert_eq!(photo_item.albums, vec!["album1", "album2"]);
        assert_eq!(photo_item.tags.len(), 1);
        assert_eq!(photo_item.tags[0].tag, "tag1");
        assert_eq!(photo_item.tags[0].user, "user1");
        assert_eq!(photo_item.tags[0].date_create, "2023-01-01");
        assert_eq!(photo_item.people, vec!["person1", "person2"]);
        assert_eq!(photo_item.notes, vec!["note1", "note2"]);
        assert_eq!(photo_item.privacy, "public");
        assert_eq!(photo_item.comment_permissions, "everyone");
        assert_eq!(photo_item.tagging_permissions, "friends");
        assert_eq!(photo_item.safety, "safe");
        assert_eq!(photo_item.comments, vec!["comment1", "comment2"]);
    }
}
