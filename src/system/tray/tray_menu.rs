use slint::{Weak,ComponentHandle};
use i_slint_backend_winit::WinitWindowAccessor;
use crate::Main;
use std::time::Duration;
use tray_icon::
    menu::MenuEvent;

pub fn show_tray_menu(weak: Weak<Main>){
    let event_handle_weak = weak.clone();
    let _event_handle = tokio::spawn(async move {
        let weak = event_handle_weak;
        loop {
            if let Ok(event) = MenuEvent::receiver().recv_timeout(Duration::from_millis(200)) {
                println!("event:{:?}", event);
                if event.id.0 == "1000" {
                    break;
                } else if event.id.0 == "1001" {
                    let event_handle_weak = weak.clone();
                    let _ = slint::invoke_from_event_loop(move || {
                        event_handle_weak.unwrap().window()
                            .with_winit_window(|winit_window: &winit::window::Window| {
                                winit_window.set_visible(true);
                            });
                    });
                }
            }
        }

        let event_handle_weak = weak.clone();
        let _ = slint::invoke_from_event_loop(move || {
            let weak = event_handle_weak.unwrap();
            let _ = weak.hide();
        });

        println!("Program exit!")
    });
}


