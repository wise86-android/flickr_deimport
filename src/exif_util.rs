use std::path::PathBuf;

use little_exif::exif_tag::ExifTag;
use little_exif::metadata::Metadata;
use little_exif::rational::uR64;

use crate::fliker_config::photo_item::PhotoItem;

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

pub fn add_exif_medata(
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
