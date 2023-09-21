use std::fs::{metadata, read_dir};

use crate::utils::{image_arrtibute_init, open_file, open_folder};
use crate::{HandleStatus, ImageAttr, ImageAttribute, Main};
use slint::{ComponentHandle, Model, ModelRc, SharedString, VecModel, Weak};

pub fn image_attr(weak: Weak<Main>) {
    let main_weak = weak.clone().unwrap();
    let open_folder_weak = weak.clone();
    main_weak.global::<ImageAttr>().on_open_folder(move || {
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
                            let mut name = file_name.split(".");
                            image_arrtibute.out_name =
                                SharedString::from(String::from("out_") + name.next().unwrap());
                            image_arrtibute.out_path =
                                SharedString::from(path.parent().unwrap().to_str().unwrap());
                            image_attrs.push(image_arrtibute);
                        }
                    }
                }
            }
        }
        open_folder_weak
            .unwrap()
            .global::<ImageAttr>()
            .set_imageAttr(ModelRc::new(VecModel::from(image_attrs)));
    });

    let open_file_weak = weak.clone();
    main_weak.global::<ImageAttr>().on_open_file(move || {
        //获取增加到后面
        if let Some(f) = open_file() {
            let mut image_arrtibute = ImageAttribute::default();
            let file_name = f.file_name().unwrap().to_str().unwrap();
            image_arrtibute_init(f.to_str().unwrap(), &mut image_arrtibute);
            image_arrtibute.name = SharedString::from(file_name);
            image_arrtibute.path = SharedString::from(f.parent().unwrap().to_str().unwrap());
            let mut name = file_name.split(".");
            image_arrtibute.out_name =
                SharedString::from(String::from("out_") + name.next().unwrap());
            image_arrtibute.out_path = SharedString::from(f.parent().unwrap().to_str().unwrap());
            let image_attr = open_file_weak.unwrap().global::<ImageAttr>().get_imageAttr();
            let mut vec_model = Vec::new();
            for m in image_attr.iter() {
                vec_model.push(m);
            }
            vec_model.push(image_arrtibute);
            open_file_weak.unwrap().global::<ImageAttr>().set_imageAttr(ModelRc::new(VecModel::from(vec_model)));
        }
    });

    let switch_out_type_weak = weak.clone();
    main_weak.global::<ImageAttr>().on_switch_out_type(move |image_format|{
        let image_attr = switch_out_type_weak.unwrap().global::<ImageAttr>().get_imageAttr();
        let mut vec_model = Vec::new();
            for m in image_attr.iter() {
                let mut _m = m.clone();
                _m.out_image_type = image_format;
                vec_model.push(_m);
            }
            switch_out_type_weak.unwrap().global::<ImageAttr>().set_imageAttr(ModelRc::new(VecModel::from(vec_model)));
    });

    let handle_image_weak = weak.clone();
    main_weak.global::<ImageAttr>().on_handle_image({
        let image_weak = handle_image_weak.clone();
        move || {
            let image_weak = image_weak.clone();
            let image_attr_model = image_weak.unwrap().global::<ImageAttr>().get_imageAttr();
            let mut vec_model = Vec::new();
            for m in image_attr_model.iter() {
                vec_model.push(m);
            }
            tokio::spawn(async move {
                let mut i = 0;
                for m in vec_model.iter() {
                    if m.handle_result != HandleStatus::Complete {
                        let mut image_attribute = m.clone();

                        let _ = image_attribute.convert();
                        //更新界面
                        let _ = image_weak.upgrade_in_event_loop(move |main| {
                            //获取更新ui
                            let image_main_weak = main.global::<ImageAttr>().get_imageAttr();
                            image_main_weak.set_row_data(i, image_attribute);
                        });
                    }
                    i += 1;
                }
            });
        }
    });
    let clear_image_list_weak = weak.clone();
    main_weak
        .global::<ImageAttr>()
        .on_clear_image_list(move || {
            let image_attrs = Vec::new();
            clear_image_list_weak
                .unwrap()
                .global::<ImageAttr>()
                .set_imageAttr(ModelRc::new(VecModel::from(image_attrs)));
        });
}
