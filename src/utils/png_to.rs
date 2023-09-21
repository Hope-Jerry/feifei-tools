
use image::DynamicImage;
use image::GenericImage;
use image::GenericImageView;
use image::ImageFormat;
use std::io::Write;
///
/// png转ico
///
pub fn ff_png_to_ico(path: &str, out_path: &str) -> Option<bool> {
    // 创建一个新的
    let mut icon_dir = ico::IconDir::new(ico::ResourceType::Icon);
    // 读取到ico
    let file = std::fs::File::open(path).unwrap();
    let image = ico::IconImage::read_png(file).unwrap();
    icon_dir.add_entry(ico::IconDirEntry::encode(&image).unwrap());
    //写入文件
    let file = std::fs::File::create(format!("{}{}",out_path,".ico")).unwrap();
    icon_dir.write(file).unwrap();
    Some(true)
}

///
/// png 转 jpeg
///
pub fn ff_png_to_jpeg(path: &str, out_path: &str) -> Option<bool> {
    if let Ok(img) = image::open(path) {
        if img.save(out_path).is_ok() {
            return Some(true);
        }
    }
    None
}

///
/// png 转 bmp
///
pub fn ff_png_to_bmp(path: &str, out_path: &str) -> Option<bool> {
    // 打开PNG图像文件
    let img = image::open(path).expect("Failed to open PNG image");

    // 获取图像的宽度和高度
    let (width, height) = img.dimensions();

    // 创建一个新的灰度图像，用于表示深度图
    let mut dmp_img = DynamicImage::new_luma8(width, height);

    // 遍历每个像素，并将灰度值设置为深度值（可以根据需要进行转换）
    for y in 0..height {
        for x in 0..width {
            let pixel = img.get_pixel(x, y);
            //let gray_value =
            //    (0.299 * pixel[0] as f32 + 0.587 * pixel[1] as f32 + 0.114 * pixel[2] as f32) as u8;
            dmp_img.put_pixel(x, y, pixel);
        }
    }

    // 保存深度图为PNG格式（你也可以保存为其他格式，或将数据导出为其他格式）
    dmp_img
        .save_with_format(format!("{}{}",out_path,".png"), ImageFormat::Png)
        .expect("Failed to save as DMP");

    Some(true)
}


///
/// png 转 svg
/// 
pub fn ff_png_to_svg(path: &str, out_path: &str) -> Option<bool> {
    // 打开 PNG 图像文件
    let img = image::open(path).expect("Failed to open PNG image");

    // 获取图像的宽度和高度
    let (width, height) = img.dimensions();

    // 创建 SVG 头部
    let svg_header = format!(
        r#"<svg xmlns="http://www.w3.org/2000/svg" width="{}" height="{}">"#,
        width, height
    );

    // 创建 SVG 图形内容
    let mut svg_content = String::new();
    for y in 0..height {
        for x in 0..width {
            let pixel = img.get_pixel(x, y);
            let r = pixel.0[0];
            let g = pixel.0[1];
            let b = pixel.0[2];
            let a = pixel.0[3];
            //let (r, g, b, a) = pixel.channels4();
            // 创建 SVG 矩形，颜色和位置与 PNG 像素对应
            svg_content.push_str(&format!(
                r#"<rect x="{}" y="{}" width="1" height="1" fill="rgba({}, {}, {}, {})" />"#,
                x, y, r, g, b, a as f32 / 255.0
            ));
        }
    }

    // 创建 SVG 尾部
    let svg_footer = "</svg>";

    // 将 SVG 写入输出文件
    let mut output_file = std::fs::File::create(format!("{}{}",out_path,".svg")).expect("Failed to create SVG file");
    writeln!(output_file, "{}", svg_header).expect("Failed to write SVG header");
    writeln!(output_file, "{}", svg_content).expect("Failed to write SVG content");
    writeln!(output_file, "{}", svg_footer).expect("Failed to write SVG footer");
    Some(true)
}
