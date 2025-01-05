use crate::fliker_config::albums;
use std::env;
use std::fs;
use std::io;
use std::path::Path;
use std::path::PathBuf;
mod fliker_config;

fn main() {
    let album_file_path = env::args().nth(1).expect("No file path provided");
    let image_dir_path = env::args()
        .nth(2)
        .and_then(|path| Some(PathBuf::from(path)))
        .expect("No file path provided");
    let out_album_dir_path = env::args()
        .nth(3)
        .and_then(|path| Some(PathBuf::from(path)))
        .expect("No file path provided");
    let album_file = fs::read_to_string(album_file_path).expect("Unable to read file");
    let albums_data: albums::Albums =
        serde_json::from_str(&album_file).expect("Unable to parse JSON");

    let tallinn_album = albums_data
        .albums
        .iter()
        .find(|album| album.title == "Tallin")
        .unwrap();
    move_image_files(&tallinn_album, &image_dir_path, &out_album_dir_path);
}

trait PathBufVecExt {
    fn find_file_with_substring(&self, substring: &str) -> Option<&PathBuf>;
}

impl PathBufVecExt for Vec<PathBuf> {
    fn find_file_with_substring(&self, substring: &str) -> Option<&PathBuf> {
        self.iter().find(|file| {
            file.to_str()
                .and_then(|s| Some(s.contains(substring)))
                .unwrap_or(false)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use std::sync::LazyLock;

    static FILES: LazyLock<Vec<PathBuf>> = LazyLock::new(|| {
        return vec![
            PathBuf::from("document.txt"),
            PathBuf::from("image1.jpg"),
            PathBuf::from("image2.jpg"),
        ];
    });

    #[test]
    fn find_file_with_substring_return_the_some_value_if_match() {
        assert_eq!(
            FILES.find_file_with_substring("image1"),
            Some(&PathBuf::from("image1.jpg"))
        );
    }

    #[test]
    fn find_file_with_substring_return_the_first_match() {
        assert_eq!(
            FILES.find_file_with_substring("image"),
            Some(&PathBuf::from("image1.jpg"))
        );
    }

    #[test]
    fn find_file_with_substring_return_none_if_not_find_a_match() {
        assert_eq!(FILES.find_file_with_substring("notExisting"), None);
    }
}

fn move_image_files(album: &albums::Album, image_dir_path: &Path, out_album_dir_path: &Path) {
    let images = extract_directory_files(image_dir_path).unwrap();
    let out_dir = out_album_dir_path.join(album.title.clone());
    fs::create_dir_all(&out_dir).unwrap();

    album.photos.iter().for_each(|photo| {
        let photo_file = images.find_file_with_substring(&photo);
        match photo_file {
            Some(file) => {
                let out_file_path = out_dir.join(file.file_name().unwrap());
                println!(
                    "Copying file: {} to {}",
                    file.to_str().unwrap(),
                    out_file_path.to_str().unwrap()
                );
                fs::copy(&file, out_file_path).unwrap();
            }
            None => println!("File not found: {}", photo),
        };
    });
}

fn extract_directory_files(dir_path: &Path) -> Result<Vec<PathBuf>, io::Error> {
    fs::read_dir(dir_path)?
        .map(|file| file.map(|f| f.path()))
        .collect()
}
