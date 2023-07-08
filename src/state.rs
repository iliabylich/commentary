use crate::{database::Database, resource::Resources};

#[derive(Clone)]
pub(crate) struct AppState {
    pub(crate) database: Database,
    pub(crate) resources: Resources,
}

impl AppState {
    pub(crate) fn new(database: Database, resources: Resources) -> Self {
        Self {
            database,
            resources,
        }
    }
}
