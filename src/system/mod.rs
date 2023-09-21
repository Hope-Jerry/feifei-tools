mod bar_system;
mod tray;
mod notify;
use std::env;

pub use notify::_notify;
pub use bar_system::system_window;
use slint::ComponentHandle;
pub use tray::show_tray_menu;

use crate::Main;
use tray_icon::{
    menu::{Menu, MenuItem},
    Icon, TrayIconBuilder,
};


pub fn run_main(main: Main){

    let object_dir = env!("CARGO_MANIFEST_DIR");

    let img = image::open(object_dir.to_string() + "/ui/assets/tray/theme.ico").unwrap();
    let rgba = img.as_rgba8().unwrap();
    let to_vec = rgba.to_vec();
    let icon = Icon::from_rgba(to_vec, 64,64).unwrap();

    /*let icon = Icon::from_path(
        object_dir.to_string() + "/ui/assets/tray/theme.ico",
        Some((64, 64)),
    )
    .unwrap();*/

    let quit = MenuItem::new("退出", true, None);
    let show = MenuItem::new("主窗体", true, None);
    //增加带有图标的菜单
    //let theme_tray_menu = Submenu::new("多级菜单",true);
    

    //let icon_light = tray_icon::menu::Icon::from_path(object_dir.to_string() + "/ui/assets/tray/light.ico", Some((64,64))).expect("reload icon failed!!!");
    //let icon_dart = tray_icon::menu::Icon::from_path(object_dir.to_string() + "/ui/assets/tray/dart.ico", Some((64,64))).expect("reload icon failed!!!");
    //let icon_transparent = tray_icon::menu::Icon::from_path(object_dir.to_string() + "/ui/assets/tray/transparent.ico", Some((64,64))).expect("reload icon failed!!!");
    //let light_theme_menu = IconMenuItem::new("一", true, Some(icon_dart), None);
    //let dart_theme_menu = IconMenuItem::new("二", true, Some(icon_light), None);
    //let transparent_theme_menu = IconMenuItem::new("三", true, Some(icon_transparent), None);
    //let _ = theme_tray_menu.append(&light_theme_menu);
    //let _ = theme_tray_menu.append(&dart_theme_menu);
    //let _ = theme_tray_menu.append(&transparent_theme_menu);

    let tray_menu = Menu::new();
    //let _ = tray_menu.append(&theme_tray_menu);
    let _ = tray_menu.append(&show);
    let _ = tray_menu.append(&quit);
    
    

    let _tray_icon = TrayIconBuilder::new()
        .with_menu(Box::new(tray_menu))
        .with_tooltip("工具箱")
        .with_icon(icon)
        .build()
        .unwrap();

    _tray_icon.set_title(Some("title"));

    show_tray_menu(main.as_weak());

    main.run().expect("run mian failed!!!");
}