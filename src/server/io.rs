use crate::{commands::Commands, session::UserSession};
use std::fs;

pub(crate) struct IOOperationHandler<'a> {
    session: &'a UserSession,
}

impl<'a> IOOperationHandler<'a> {
    pub(crate) fn new(session: &'a UserSession) -> IOOperationHandler {
        IOOperationHandler { session }
    }

    fn ls(&self, command: Commands) {}
}
