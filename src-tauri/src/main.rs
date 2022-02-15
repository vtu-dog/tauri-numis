//! ctmpnumis.fr update notifier

#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod commands;
mod product;
mod scraper;

use tauri::{CustomMenuItem, SystemTray, SystemTrayEvent, SystemTrayMenu, WindowBuilder};

/// Starts the application.
fn main() {
    // create a tray item...
    let tray = SystemTray::new();

    // ...and add a Quit menu item
    let quit = CustomMenuItem::new("quit".to_string(), "Wyjdź");
    let tray_menu = SystemTrayMenu::new().add_item(quit);

    // initialise Tauri
    let app = tauri::Builder::default()
        .system_tray(tray.with_menu(tray_menu))
        .setup(|app| {
            // create an app handle
            let handle = app.handle();

            // spawn a new thread to handle the event loop
            tauri::async_runtime::spawn(async {
                event_loop(handle).await;
            });

            // we don't care about the join handle
            Ok(())
        })
        .on_system_tray_event(|_, event| match event {
            // if the Quit menu item is clicked, quit the app
            SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
                "quit" => {
                    std::process::exit(0);
                }
                _ => {}
            },
            // ignore any other events
            _ => {}
        })
        .invoke_handler(tauri::generate_handler![commands::retrieve_products])
        .build(tauri::generate_context!())
        .expect("error while building tauri application");

    // run the app
    app.run(|_, e| match e {
        // prevent app exit when the last window is closed
        tauri::RunEvent::ExitRequested { api, .. } => {
            api.prevent_exit();
        }
        _ => {}
    });
}

/// Handles the event loop.
async fn event_loop(app_handle: tauri::AppHandle) {
    // create a new reqwest::Client
    let client = reqwest::Client::new();

    // set website polling interval and set it off immediately
    let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(300));
    interval.tick().await;

    // repeat forever:
    loop {
        // 1. check if the website received an update
        match scraper::get_update(&client).await {
            // 2. if it didn't, do nothing
            Ok(None) => { /* pass */ }
            // 3. if it did, create a new window
            Ok(Some(products)) => {
                // store the products in an internal variable to be retrieved via the invoke API
                commands::store_products(&products).await;

                // create a new window and display the products
                app_handle
                    .create_window(
                        &uuid::Uuid::new_v4().to_string(),
                        tauri::WindowUrl::App("".into()),
                        |_wb, attrs| {
                            let wb = _wb
                                .title(format!("CTMP Numis: nowe oferty ({})", products.len()))
                                .inner_size(500.0, 730.0)
                                .resizable(false)
                                .always_on_top(true)
                                .skip_taskbar(true);

                            (wb, attrs)
                        },
                    )
                    .expect("could not create a window");
            }
            // 4. if an unexpected error occurred, print it to stderr
            Err(e) => eprintln!("{:#?}", e),
        }

        // 5. wait for a set interval
        interval.tick().await;
    }
}
