


///
/// png转ico
///
pub fn ff_jpg_to_ico(path: &str, out_path: &str) -> Option<bool> {
    // 创建一个新的
    let mut icon_dir = ico::IconDir::new(ico::ResourceType::Icon);
    // 读取到ico
    if let Ok(file) = std::fs::File::open(path){
        if let Ok(image) = ico::IconImage::read_png(file) {
            icon_dir.add_entry(ico::IconDirEntry::encode(&image).unwrap());
            //写入文件
            let save_path = format!("{}.ico",out_path);
            let file = std::fs::File::create(save_path).unwrap();
            icon_dir.write(file).unwrap();
            return Some(true);
        }
    }
    None
}
