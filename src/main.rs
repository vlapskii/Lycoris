use slint::{ComponentHandle, PlatformError, SharedString, ToSharedString};

slint::include_modules!();

fn is_logged() -> bool {
    false
}

fn main() {
    let main_window = MainWindow::new().unwrap();
    let main_weak = main_window.as_weak();

    if is_logged() {
        main_weak.upgrade().unwrap().set_title_window("Lycoris".to_shared_string());
        main_weak.upgrade().unwrap().set_is_logged(true);
    } else {
        main_weak.upgrade().unwrap().set_title_window("Авторизация в ИС".to_shared_string());
    }

    main_window.on_login(move |login_text: SharedString, password_text: SharedString| {
        let main_window = main_weak.upgrade().unwrap();
        main_window.set_is_logged(true);
        main_window.set_title_window("Lycoris".to_shared_string());
    });

    main_window.run().unwrap();
}
