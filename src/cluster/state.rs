use std::{ops::Deref, sync::Arc};

use twilight_http::Client as HttpClient;
use twilight_model::id::{
    marker::{ApplicationMarker, UserMarker},
    Id,
};

#[derive(Clone)]
#[repr(transparent)]
pub struct ShardState(Arc<ShardStateRef>);

impl ShardState {
    pub fn new(
        application_id: Id<ApplicationMarker>,
        current_user_id: Id<UserMarker>,
        http: Arc<HttpClient>,
    ) -> Self {
        Self(Arc::new(ShardStateRef {
            application_id,
            current_user_id,
            http,
        }))
    }
}

impl Deref for ShardState {
    type Target = ShardStateRef;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub struct ShardStateRef {
    pub application_id: Id<ApplicationMarker>,
    pub current_user_id: Id<UserMarker>,
    pub http: Arc<HttpClient>,
}
