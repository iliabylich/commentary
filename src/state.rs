use crate::{database::Database, mailer::Gmail};

#[derive(Clone)]
pub(crate) struct AppState {
    pub(crate) database: Database,
    pub(crate) mailer: Gmail,
}

impl AppState {
    pub(crate) fn new(database: Database, mailer: Gmail) -> Self {
        Self { database, mailer }
    }
}
