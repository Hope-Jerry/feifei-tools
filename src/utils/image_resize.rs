


///
/// 修改图片的尺寸大小
/// 
pub fn _ff_resize(width: u32,height: u32){
    let img = image::open("E:/rust/image/huaixiao.png").expect("open file failed!!");
    let new_img = img.resize(width, height, image::imageops::FilterType::Nearest);
    let _ = new_img.save_with_format("E:/rust/image/new_huaixiao.png", image::ImageFormat::Png);
}


