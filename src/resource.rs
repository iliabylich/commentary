use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "resources/"]
struct Asset;

#[derive(Debug, Clone)]
pub(crate) struct Resource(&'static str);

impl Resource {
    pub(crate) fn render(&self) -> String {
        let asset = Asset::get(self.0).unwrap();
        let data = asset.data.as_ref();
        std::str::from_utf8(data).unwrap().to_string()
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub(crate) enum ResourceId {
    IndexHtml,
    IndexMjs,
}

#[derive(Debug, Clone)]
pub(crate) struct Resources {
    pub(crate) map: std::collections::HashMap<ResourceId, Resource>,
}

impl Resources {
    pub(crate) fn new() -> Self {
        let map = std::collections::HashMap::from([
            (ResourceId::IndexHtml, Resource("index.html")),
            (ResourceId::IndexMjs, Resource("index.mjs")),
        ]);
        Self { map }
    }

    pub(crate) fn get(&self, id: ResourceId) -> &Resource {
        self.map.get(&id).expect("Resource not found")
    }
}
