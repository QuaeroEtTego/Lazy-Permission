use std::{ops::Deref, sync::Arc};

use twilight_http::client::{Client as HttpClient, InteractionClient};
use twilight_model::id::{
    marker::{ApplicationMarker, UserMarker},
    Id,
};

#[derive(Clone)]
#[repr(transparent)]
pub struct BotState(Arc<BotStateRef>);

impl BotState {
    pub fn new(
        application_id: Id<ApplicationMarker>,
        current_user_id: Id<UserMarker>,
        http: HttpClient,
    ) -> Self {
        Self(Arc::new(BotStateRef {
            application_id,
            current_user_id,
            http,
        }))
    }
}

impl Deref for BotState {
    type Target = BotStateRef;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub struct BotStateRef {
    pub application_id: Id<ApplicationMarker>,
    pub current_user_id: Id<UserMarker>,
    pub http: HttpClient,
}

impl BotStateRef {
    pub fn interaction(&self) -> InteractionClient {
        self.http.interaction(self.application_id)
    }
}
