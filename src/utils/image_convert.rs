use crate::utils::jpeg_to::ff_jpg_to_ico;

use super::png_to;
use crate::{ImageAttribute,ImageConvertFormat,HandleStatus};

impl ImageAttribute {

    pub fn convert(&mut self) -> Option<bool> {
        //判断图片的类型
        if let Some(_res) = ff_convert(
            &self.path,
            &self.out_path,
            self.image_type,
            self.out_image_type,
        ) {
            self.handle_result = HandleStatus::Complete;
            Some(true)
        } else {
            self.handle_result = HandleStatus::Error;
            None
        }
    }
}

fn ff_convert(
    path: &str,
    out_path: &str,
    image_type: ImageConvertFormat,
    out_image_type: ImageConvertFormat,
) -> Option<bool> {
    println!("{:?},{:?}",image_type,out_image_type);
    match image_type {
        ImageConvertFormat::Png => match out_image_type {
            ImageConvertFormat::Ico => {
                return png_to::ff_png_to_ico(path, out_path);
            }
            ImageConvertFormat::Jpeg => {
                return png_to::ff_png_to_jpeg(path, out_path);
            }
            ImageConvertFormat::Bmp => {
                return png_to::ff_png_to_bmp(path, out_path);
            }
            ImageConvertFormat::Svg => {
                return png_to::ff_png_to_svg(path, out_path);
            }
            _ => {
                println!("未知");
            }
        },
        ImageConvertFormat::Jpeg => {
            match out_image_type {
                ImageConvertFormat::Ico => {
                    return ff_jpg_to_ico(path, out_path);
                }
                _ => {
                    println!("未知");
                }
            }
        }
        _ => {
            return None;
        }
    }
    None
}
