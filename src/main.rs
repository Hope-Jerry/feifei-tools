//#![windows_subsystem = "windows"]
mod utils;
mod model;
mod system;
mod user;

use user::image_attr;
use i_slint_backend_winit::WinitWindowAccessor;
use system::{system_window, run_main};

slint::include_modules!();

#[tokio::main]
async fn main() {

    let slint_winit = i_slint_backend_winit::Backend::new();

    let _ = slint::platform::set_platform(Box::new(slint_winit));

    let main = Main::new().expect("Form initialization failed!");

    let main_weak = main.as_weak();
    main_weak
        .unwrap()
        .window()
        .with_winit_window(|winit_window: &winit::window::Window| {
            winit_window.set_decorations(true);
            winit_window.set_transparent(true);
        });

    image_attr(main_weak.clone());
    system_window(main_weak);
    
    run_main(main);
}
