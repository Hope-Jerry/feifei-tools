
use std::path::PathBuf;

use image::{ImageFormat,io::Reader as ImageReader};
use rfd::FileDialog;
use crate::{ImageAttribute,ImageConvertFormat,HandleStatus};

pub fn open_folder() -> String {
    let files = FileDialog::new()
    .set_directory("/")
    .pick_folder();
    if let Some(path) = files {
        return path.as_os_str().to_str().unwrap().to_string();
    }
    "".to_string()
}

pub fn open_file() -> Option<PathBuf> {
    let files = FileDialog::new()
    .add_filter("image", &["png","jpeg"])
    .set_directory("/")
    .pick_file();

    files
}

pub fn image_arrtibute_init(path: &str,image_attribute: &mut ImageAttribute){
    if let Ok(img) = ImageReader::open(path){
        image_attribute.handle_result = HandleStatus::Wait;
        if let Some(format) = img.format(){
            match format {
                ImageFormat::Png => {
                    image_attribute.image_type = ImageConvertFormat::Png;
                }
                ImageFormat::Jpeg => {
                    image_attribute.image_type = ImageConvertFormat::Jpeg;
                }
                ImageFormat::Ico => {
                    image_attribute.image_type = ImageConvertFormat::Ico;
                }
                ImageFormat::Bmp => {
                    image_attribute.image_type = ImageConvertFormat::Bmp;
                }
                _ => {
                    image_attribute.handle_result = HandleStatus::Error;
                }
            }
        }
        if let Ok(p) = img.decode(){
            image_attribute.width = p.width() as i32;
            image_attribute.height = p.height() as i32;
            image_attribute.n_width = p.width() as i32;
            image_attribute.n_height = p.height() as i32;
        }else{
            image_attribute.handle_result = HandleStatus::Error;
        }
        
    }
}

