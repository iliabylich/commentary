use crate::database::Database;

#[derive(Clone)]
pub(crate) struct AppState {
    pub(crate) database: Database,
}

impl AppState {
    pub(crate) fn new(database: Database) -> Self {
        Self { database }
    }
}
