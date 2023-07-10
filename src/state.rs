use crate::{database::Database, mailer::Gmail, resource::Resources};

#[derive(Clone)]
pub(crate) struct AppState {
    pub(crate) database: Database,
    pub(crate) resources: Resources,
    pub(crate) mailer: Gmail,
}

impl AppState {
    pub(crate) fn new(database: Database, resources: Resources, mailer: Gmail) -> Self {
        Self {
            database,
            resources,
            mailer,
        }
    }
}
