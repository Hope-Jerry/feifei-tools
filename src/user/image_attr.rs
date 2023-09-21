use std::fs::{metadata, read_dir};
use std::thread::{Thread, self};
use std::time::Duration;

use crate::utils::{image_arrtibute_init, open_folder};
use crate::{ImageAttr, ImageAttribute, Main,HandleStatus};
use slint::{ComponentHandle, ModelRc, SharedString, VecModel, Weak, Model};

pub fn image_attr(weak: Weak<Main>) {
    let main_weak = weak.clone().unwrap();
    let select_folder_weak = weak.clone();
    main_weak.global::<ImageAttr>().on_select_folder(move || {
        let root_path = open_folder();
        //获取所有的符合要求的文件
        let root_dir = read_dir(root_path);
        //创建一个类型
        let mut image_attrs = Vec::new();
        if let Ok(dirs) = root_dir {
            for entiy in dirs {
                if let Ok(dir_entry) = entiy {
                    let path = dir_entry.path();
                    if let Ok(f) = metadata(&path) {
                        if f.is_file() {
                            let file_name = path.file_name().unwrap().to_str().unwrap();
                            let mut image_arrtibute = ImageAttribute::default();
                            image_arrtibute_init(path.to_str().unwrap(), &mut image_arrtibute);
                            image_arrtibute.name = SharedString::from(file_name);
                            image_arrtibute.path =
                                SharedString::from(path.parent().unwrap().to_str().unwrap());
                            image_arrtibute.out_name =
                                SharedString::from(String::from("out_") + file_name);
                            image_arrtibute.out_path =
                                SharedString::from(path.parent().unwrap().to_str().unwrap());
                            image_attrs.push(image_arrtibute);
                        }
                    }
                }
            }
        }
        select_folder_weak
            .unwrap()
            .global::<ImageAttr>()
            .set_imageAttr(ModelRc::new(VecModel::from(image_attrs)));
    });
    let handle_image_weak = weak.clone();
    main_weak.global::<ImageAttr>().on_handle_image({
        let image_weak = handle_image_weak.clone();
        move || {
            let image_weak = image_weak.clone();
            let image_attr_model = image_weak.unwrap().global::<ImageAttr>().get_imageAttr();
            let mut vec_model = Vec::new();
            for m in image_attr_model.iter(){
                vec_model.push(m);
            }
            tokio::spawn(async move {
                let mut i = 0;
                for m in vec_model.iter(){
                    let mut image_attribute = m.clone();
                    //处理文件
                    thread::sleep(Duration::from_millis(1000));
                    image_attribute.handle_result = HandleStatus::Complete;
                    //更新界面
                    let _ = image_weak.upgrade_in_event_loop(move |main|{
                        //获取更新ui
                        let image_main_weak = main.global::<ImageAttr>().get_imageAttr();
                        image_main_weak.set_row_data(i, image_attribute);
                    });
                    i += 1;
                }
            });
        }
    });
}
