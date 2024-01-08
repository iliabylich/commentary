use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "resources/"]
pub(crate) struct Asset;

impl Asset {
    pub(crate) fn index_html() -> String {
        Self::render("index.html")
    }

    pub(crate) fn index_mjs() -> String {
        Self::render("index.mjs")
    }

    pub(crate) fn output_css() -> String {
        Self::render("output.css")
    }

    fn render(path: &str) -> String {
        let asset = Self::get(path).unwrap();
        String::from_utf8(asset.data.to_vec()).unwrap()
    }
}
