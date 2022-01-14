//! Tauri invoke API commands & helpers

use futures::lock::Mutex;
use once_cell::sync::OnceCell;

use super::product::Product;

static PRODUCTS: OnceCell<Mutex<String>> = OnceCell::new();

/// Retrieves the latest products as a JSON string.
#[tauri::command] // <- this is a command handler for Tauri API
pub async fn retrieve_products() -> String {
    PRODUCTS
        .get()
        .expect("failed to get product")
        .lock()
        .await
        .to_string()
}

/// Serialises given products to a JSON string and stores them for the next API call.
pub async fn store_products(products: &[Product]) {
    match PRODUCTS.get() {
        // once_cell has not been initialised yet
        None => {
            PRODUCTS
                .set(Mutex::new(
                    serde_json::to_string(products)
                        .expect("failed to serialise product (once_cell init)"),
                ))
                .expect("failed to initialise once_cell");
        }
        // once_cell has already been initialised
        Some(mutex) => {
            let mut value = mutex.lock().await;
            *value = serde_json::to_string(products).expect("failed to serialise product");
        }
    }
}
