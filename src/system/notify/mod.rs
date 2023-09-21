use notify_rust::Notification;

//发送系统通知
pub fn _notify(summary: &str,body: &str,timeout: i32){
    let _ = Notification::new()
            .summary(summary)
            .body(body)
            .auto_icon()
            .timeout(timeout)
            .show();
}

