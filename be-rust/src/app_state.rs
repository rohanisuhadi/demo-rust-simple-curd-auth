use std::sync::Arc;

pub struct AppState<A, U> {
    pub auth_service: Arc<A>,
    pub user_service: Arc<U>,
}

impl<A, U> AppState<A, U> {
    pub fn new(auth: A, user: U) -> Self {
        Self {
            auth_service: Arc::new(auth),
            user_service: Arc::new(user),
        }
    }
}

impl<A, U> Clone for AppState<A, U> {
    fn clone(&self) -> Self {
        Self {
            auth_service: self.auth_service.clone(),
            user_service: self.user_service.clone(),
        }
    }
}
