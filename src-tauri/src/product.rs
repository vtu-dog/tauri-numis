//! `Product` type definition

use serde::Serialize;

/// Product struct.
#[derive(Serialize)]
pub struct Product {
    /// Product store page URL.
    pub href: String,
    /// Product title.
    pub title: String,
    /// Product category.
    pub category: String,
    /// Product price.
    pub price: String,
    /// Product image URL (obverse side).
    pub obverse_img_href: String,
    /// Product image URL (reverse side).
    pub reverse_img_href: String,
}
