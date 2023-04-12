use std::{ops::Deref, sync::Arc};

use twilight_http::Client as HttpClient;
use twilight_model::id::{
    marker::{ApplicationMarker, UserMarker},
    Id,
};

#[derive(Clone)]
#[repr(transparent)]
pub struct ClusterState(Arc<ClusterStateRef>);

impl ClusterState {
    pub fn new(
        application_id: Id<ApplicationMarker>,
        current_user_id: Id<UserMarker>,
        http: HttpClient,
    ) -> Self {
        Self(Arc::new(ClusterStateRef {
            application_id,
            current_user_id,
            http,
        }))
    }
}

impl Deref for ClusterState {
    type Target = ClusterStateRef;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub struct ClusterStateRef {
    pub application_id: Id<ApplicationMarker>,
    pub current_user_id: Id<UserMarker>,
    pub http: HttpClient,
}
