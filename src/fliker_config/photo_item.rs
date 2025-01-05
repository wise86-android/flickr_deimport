use crate::fliker_config::utils::string_to_i32;
use serde::{Deserialize, Serialize};

fn geo_coordinate_string_to_f32<'de, D>(deserializer: D) -> Result<f32, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    let value: i32 = s.parse().unwrap();
    return Ok(value as f32 / 1000000.0);
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Geo {
    #[serde(rename = "latitude", deserialize_with = "geo_coordinate_string_to_f32")]
    pub latitude: f32,
    #[serde(
        rename = "longitude",
        deserialize_with = "geo_coordinate_string_to_f32"
    )]
    pub longitude: f32,
    #[serde(rename = "accuracy")]
    pub accuracy: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Tag {
    #[serde(rename = "tag")]
    pub tag: String,
    #[serde(rename = "user")]
    pub user: String,
    #[serde(rename = "date_create")]
    pub date_create: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PhotoItem {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "count_views", deserialize_with = "string_to_i32")]
    pub count_views: i32,
    #[serde(rename = "count_faves", deserialize_with = "string_to_i32")]
    pub count_faves: i32,
    #[serde(rename = "count_comments", deserialize_with = "string_to_i32")]
    pub count_comments: i32,
    #[serde(rename = "date_taken")]
    pub date_taken: String,
    #[serde(rename = "count_tags", deserialize_with = "string_to_i32")]
    pub count_tags: i32,
    #[serde(rename = "count_notes", deserialize_with = "string_to_i32")]
    pub count_notes: i32,
    #[serde(rename = "rotation")]
    pub rotation: i32,
    #[serde(rename = "date_imported")]
    pub date_imported: String,
    #[serde(rename = "photopage")]
    pub photopage: String,
    #[serde(rename = "original")]
    pub original: String,
    #[serde(rename = "license")]
    pub license: String,
    #[serde(rename = "geo")]
    pub geo: Vec<Geo>,
    #[serde(rename = "groups")]
    pub groups: Vec<String>,
    #[serde(rename = "albums")]
    pub albums: Vec<String>,
    #[serde(rename = "tags")]
    pub tags: Vec<Tag>,
    #[serde(rename = "people")]
    pub people: Vec<String>,
    #[serde(rename = "notes")]
    pub notes: Vec<String>,
    #[serde(rename = "privacy")]
    pub privacy: String,
    #[serde(rename = "comment_permissions")]
    pub comment_permissions: String,
    #[serde(rename = "tagging_permissions")]
    pub tagging_permissions: String,
    #[serde(rename = "safety")]
    pub safety: String,
    #[serde(rename = "comments")]
    pub comments: Vec<String>,
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
                    "latitude": "40712812",
                    "longitude": "-740060",
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
        assert_eq!(photo_item.geo[0].latitude, 40.712812);
        assert_eq!(photo_item.geo[0].longitude, -0.740060);
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
