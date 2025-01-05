mod exif_util;
mod fliker_config;
mod vec_extension;

use fliker_config::albums;
use fliker_config::photo_item;
use vec_extension::PathBufVecExt;

use std::env;
use std::fs;
use std::io;
use std::path::Path;
use std::path::PathBuf;

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
                    exif_util::add_exif_medata(&out_file_path, &meta).unwrap();
                });
            }
            None => println!("File not found: {}", photo),
        };
    });
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
