use fliker_config::photo_item;
use fliker_config::photo_item::PhotoItem;
use little_exif::exif_tag::ExifTag;
use little_exif::metadata::Metadata;
use little_exif::rational::uR64;

use crate::fliker_config::albums;
use std::env;
use std::fs;
use std::io;
use std::path::Path;
use std::path::PathBuf;
mod fliker_config;
struct Config {
    album_file_path: PathBuf,
    image_dir_path: PathBuf,
    out_album_dir_path: PathBuf,
    images_medatada_dir_path: PathBuf,
}

fn extract_args(args: &Vec<String>) -> Config {
    if args.len() < 4 {
        panic!("Not enough arguments");
    }
    let to_path_buf = |path: &String| -> Option<PathBuf> { Some(PathBuf::from(path)) };
    let album_file_path = args
        .get(1)
        .and_then(to_path_buf)
        .expect("No file path provided");

    let images_medatada_dir_path = args
        .get(2)
        .and_then(to_path_buf)
        .expect("No image metadata dir path provided");
    let image_dir_path = args
        .get(3)
        .and_then(to_path_buf)
        .expect("No images dir path provided");
    let out_album_dir_path = args
        .get(4)
        .and_then(to_path_buf)
        .expect("No output album dir path provided");
    Config {
        album_file_path,
        image_dir_path,
        out_album_dir_path,
        images_medatada_dir_path,
    }
}

fn main() {
    let config = extract_args(&env::args().collect());
    let album_file = fs::read_to_string(config.album_file_path).expect("Unable to read file");
    let albums_data: albums::Albums =
        serde_json::from_str(&album_file).expect("Unable to parse JSON");

    let tallinn_album = albums_data
        .albums
        .iter()
        .find(|album| album.title == "Tallin")
        .unwrap();

    move_image_files(
        &tallinn_album,
        &config.image_dir_path,
        &config.images_medatada_dir_path,
        &config.out_album_dir_path,
    );
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

fn move_image_files(
    album: &albums::Album,
    image_dir_path: &Path,
    image_medatada_dir_path: &Path,
    out_album_dir_path: &Path,
) {
    let images = extract_directory_files(image_dir_path).unwrap();
    let images_metadata = extract_directory_files(image_medatada_dir_path).unwrap();
    let out_dir = out_album_dir_path.join(album.title.clone());
    fs::create_dir_all(&out_dir).unwrap();

    album.photos.iter().for_each(|photo| {
        let photo_file = images.find_file_with_substring(&photo);
        let image_metadata = load_image_medatada(&photo, &images_metadata).unwrap();
        match photo_file {
            Some(file) => {
                let new_file_name = compute_new_file_name(&file, &image_metadata);
                let out_file_path = out_dir.join(new_file_name);
                println!(
                    "Copying file: {} to {}",
                    file.to_str().unwrap(),
                    out_file_path.to_str().unwrap()
                );
                fs::copy(&file, &out_file_path).unwrap();
                image_metadata.map(|meta| {
                    add_exif_medata(&out_file_path, &meta).unwrap();
                });
            }
            None => println!("File not found: {}", photo),
        };
    });
}

enum LatitudeRef {
    North,
    South,
}
enum LongitudeRef {
    East,
    West,
}

impl ToString for LongitudeRef {
    fn to_string(&self) -> String {
        match self {
            LongitudeRef::East => "E".to_string(),
            LongitudeRef::West => "W".to_string(),
        }
    }
}

impl ToString for LatitudeRef {
    fn to_string(&self) -> String {
        match self {
            LatitudeRef::North => "N".to_string(),
            LatitudeRef::South => "S".to_string(),
        }
    }
}
struct DegreeCoordinate {
    degrees: i32,
    minutes: u32,
    seconds: f32,
}

impl DegreeCoordinate {
    pub fn to_exif_format(&self) -> Vec<uR64> {
        vec![
            uR64 {
                nominator: self.degrees.abs() as u32,
                denominator: 1,
            },
            uR64 {
                nominator: self.minutes,
                denominator: 1,
            },
            uR64 {
                nominator: (self.seconds * 10.0).abs().trunc() as u32,
                denominator: 10,
            },
        ]
    }
}

fn create_exif_latitude_tag(latitude: f32) -> (LatitudeRef, Vec<uR64>) {
    let dms = decimal_to_dms(latitude);
    if latitude.is_sign_negative() {
        return (LatitudeRef::South, dms.to_exif_format());
    } else {
        return (LatitudeRef::North, dms.to_exif_format());
    }
}

fn create_exif_longitude_tag(latitude: f32) -> (LongitudeRef, Vec<uR64>) {
    let dms = decimal_to_dms(latitude);
    if latitude.is_sign_negative() {
        return (LongitudeRef::West, dms.to_exif_format());
    } else {
        return (LongitudeRef::East, dms.to_exif_format());
    }
}

fn decimal_to_dms(decimal: f32) -> DegreeCoordinate {
    let degrees = decimal.trunc() as i32;
    let minutes_full = (decimal.abs() - degrees.abs() as f32) * 60.0;
    let minutes = minutes_full.trunc() as u32;
    let seconds = (minutes_full - minutes as f32) * 60.0;
    DegreeCoordinate {
        degrees,
        minutes,
        seconds,
    }
}

fn add_exif_medata(
    image_path: &PathBuf,
    image_medatada: &PhotoItem,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut metadata = Metadata::new_from_path(&image_path)?;
    metadata.set_tag(ExifTag::ImageDescription(
        image_medatada.description.clone(),
    ));
    image_medatada.geo.first().map(|geo| {
        let (latitude_ref, latitude) = create_exif_latitude_tag(geo.latitude);
        let (longitude_ref, longitude) = create_exif_longitude_tag(geo.longitude);
        metadata.set_tag(ExifTag::GPSLatitudeRef(latitude_ref.to_string()));
        metadata.set_tag(ExifTag::GPSLatitude(latitude));
        metadata.set_tag(ExifTag::GPSLongitudeRef(longitude_ref.to_string()));
        metadata.set_tag(ExifTag::GPSLongitude(longitude));
    });
    metadata.write_to_file(&image_path)?;
    Ok(())
}

fn compute_new_file_name(
    current_file: &PathBuf,
    image_metadata: &Option<photo_item::PhotoItem>,
) -> String {
    let file_extension = current_file
        .extension()
        .map(|ext| ext.to_str().unwrap().to_string())
        .unwrap_or_else(|| "".to_string());
    image_metadata
        .as_ref()
        .and_then(|meta| Some(format!("{}.{}", meta.name, file_extension)))
        .unwrap_or_else(|| {
            current_file
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string()
        })
}

fn extract_directory_files(dir_path: &Path) -> Result<Vec<PathBuf>, io::Error> {
    fs::read_dir(dir_path)?
        .map(|file| file.map(|f| f.path()))
        .collect()
}

fn load_image_medatada(
    image_id: &str,
    images_metadata: &Vec<PathBuf>,
) -> Result<Option<photo_item::PhotoItem>, io::Error> {
    let metadata_file = images_metadata.find_file_with_substring(image_id);
    match metadata_file {
        Some(file) => {
            let image_metadata_file = fs::read_to_string(file)?;
            let image_metadata: photo_item::PhotoItem = serde_json::from_str(&image_metadata_file)?;
            return Ok(Some(image_metadata));
        }
        None => return Ok(None),
    }
}
