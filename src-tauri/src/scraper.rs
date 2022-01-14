//! ctmpnumis.fr update handler logic

use super::product::Product;

use anyhow::Context;
use futures::lock::Mutex;
use once_cell::sync::OnceCell;
use scraper::{Html, Selector};

/// Website entrypoint URL.
const WEBSITE_URL: &'static str = "https://www.ctmpnumis.fr/en/";
/// The last value of the first listing's URL.
static LAST_VALUE: OnceCell<Mutex<String>> = OnceCell::new();

/// Fetches website updates.
pub async fn get_update(client: &reqwest::Client) -> anyhow::Result<Option<Vec<Product>>> {
    // retrieve an update from the website
    let items = scrape_website(client)
        .await
        .context("failed to scrape the website")?;

    // extract the first item's URL
    // if no items were found, return Ok(None)
    let href = match items.first() {
        Some(item) => item.href.to_owned(),
        None => return Ok(None),
    };

    // extract OnceCell contents and compare them with href
    // if they are the same, nothing new happened
    // if they differ, there was an update
    match LAST_VALUE.get() {
        // OnceCell had not been set before, this is the first website update check
        None => {
            // initialise the OnceCell
            LAST_VALUE
                .set(Mutex::new(href))
                .expect("failed to initialise once_cell");

            // report no new activity
            return Ok(None);
        }
        // OnceCell had been set before
        Some(mutex) => {
            // extract Mutex contents
            let mut value = mutex.lock().await;

            // compare new href value with the previous one
            if href == *value {
                // they're the same, no update will be emitted
                return Ok(None);
            } else {
                // they're different, so the website received an update
                // replace the old Mutex value...
                *value = href;
                // ...and return the Items to be displayed
                Ok(Some(items))
            }
        }
    }
}

/// Scrapes the website.
async fn scrape_website(client: &reqwest::Client) -> anyhow::Result<Vec<Product>> {
    // extract the last href value immediately
    // avoids awaiting after HTML parsing, since scraper::HTML is not Send
    let last_value = match LAST_VALUE.get() {
        Some(mutex) => Some(mutex.lock().await),
        None => None,
    };

    // retrieve the HTML body...
    let body_str = client
        .get(WEBSITE_URL)
        .send()
        .await
        .context("failed to invoke client.get(url).send()")?
        .text()
        .await
        .context("failed to invoke body.text()")?;

    // ...parse it...
    let body = Html::parse_document(&body_str);

    // ...prepare the selectors...
    let product_selector = Selector::parse(".product").unwrap();
    let href_selector = Selector::parse("a[href]").unwrap();
    let title_selector = Selector::parse(".box-text a").unwrap();
    let category_selector = Selector::parse(".category").unwrap();
    let price_selector = Selector::parse("bdi").unwrap();
    let img_selector = Selector::parse("img").unwrap();

    // ...create an empty Vec of Products...
    let mut products = Vec::new();

    // ...and use the selectors to extract what's needed
    for product in body.select(&product_selector) {
        // extract the product's href
        let href = product
            .select(&href_selector)
            .next()
            .map(|x| x.value())
            .and_then(|x| x.attr("href"))
            .context("href_selector failed")?
            .to_owned();

        // extract the product's title
        let title = product
            .select(&title_selector)
            .next()
            .map(|x| x.text())
            .context("title_selector failed")?
            .map(|x| x.trim())
            .collect::<Vec<_>>()
            .join(" ")
            .to_owned();

        // extract the product's category
        let category = product
            .select(&category_selector)
            .next()
            .map(|x| x.text())
            .context("category_selector failed")?
            .map(|x| x.trim())
            .collect::<Vec<_>>()
            .join(" ")
            .to_owned();

        // extract the product's price
        let price = match product.select(&price_selector).next().map(|x| x.text()) {
            Some(text_iter) => text_iter.collect::<Vec<_>>().join(" ").trim().to_owned(),
            None => "OUT OF STOCK".to_string(),
        };

        // extract the product's image hrefs
        let mut imgs = product.select(&img_selector);

        // extract the product's obverse image href
        let obverse_img_href = imgs
            .next()
            .map(|x| x.value())
            .and_then(|x| x.attr("src"))
            .context("img_selector for obverse image failed")?
            .replace("-247x296", "");

        // extract the product's reverse image href
        let reverse_img_href = imgs
            .next()
            .map(|x| x.value())
            .and_then(|x| x.attr("src"))
            .context("img_selector for reverse image failed")?
            .replace("-247x296", "");

        // create a Product from the extracted data
        products.push(Product {
            href,
            title,
            category,
            price,
            obverse_img_href,
            reverse_img_href,
        });
    }

    // filter out products that were already scraped
    match last_value {
        // OnceCell had not been set before, no filtering is required
        None => Ok(products),
        // OnceCell had been set before, filtering is required
        Some(value) => {
            // filter the products
            if let Some(idx) = products.iter().position(|x| x.href == *value) {
                Ok(products.into_iter().take(idx).collect())
            } else {
                Ok(products)
            }
        }
    }
}
