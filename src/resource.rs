#[derive(Debug, Clone)]
pub(crate) struct Resource {
    pub(crate) path: String,
    pub(crate) embedded_src: &'static str,
}

impl Resource {
    pub(crate) fn new(path: String, embedded_src: &'static str) -> Self {
        Self { path, embedded_src }
    }

    async fn render_debug(&self) -> String {
        tokio::fs::read_to_string(&self.path)
            .await
            .expect("Failed to read file")
    }

    async fn render_release(&self) -> String {
        self.embedded_src.to_string()
    }

    pub(crate) async fn render(&self) -> String {
        if cfg!(debug_assertions) {
            self.render_debug().await
        } else {
            self.render_release().await
        }
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub(crate) enum ResourceId {
    IndexHtml,
    IndexJs,
    IndexCss,
}

#[derive(Debug, Clone)]
pub(crate) struct Resources {
    pub(crate) map: std::collections::HashMap<ResourceId, Resource>,
}

impl Resources {
    pub(crate) fn new() -> Self {
        let map = std::collections::HashMap::from([
            (
                ResourceId::IndexHtml,
                Resource::new(
                    String::from("resources/index.html"),
                    include_str!("../resources/index.html"),
                ),
            ),
            (
                ResourceId::IndexJs,
                Resource::new(
                    String::from("resources/index.js"),
                    include_str!("../resources/index.js"),
                ),
            ),
            (
                ResourceId::IndexCss,
                Resource::new(
                    String::from("resources/index.css"),
                    include_str!("../resources/index.css"),
                ),
            ),
        ]);
        Self { map }
    }

    pub(crate) fn get(&self, id: ResourceId) -> &Resource {
        self.map.get(&id).expect("Resource not found")
    }
}
