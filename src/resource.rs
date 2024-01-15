use anyhow::{Context, Result};
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "resources/"]
pub(crate) struct Asset;

impl Asset {
    pub(crate) fn index_html() -> Result<String> {
        Self::render("index.html")
    }

    pub(crate) fn index_mjs() -> Result<String> {
        Self::render("index.mjs")
    }

    pub(crate) fn output_css() -> Result<String> {
        Self::render("output.css")
    }

    fn render(path: &str) -> Result<String> {
        let asset = Self::get(path).context("Failed to get asset")?;
        String::from_utf8(asset.data.to_vec()).context("Failed to convert asset to string")
    }
}
