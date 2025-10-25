use dashmap::DashSet;
use crate::domain::data_stores::BannedTokenStore;

#[derive(Default)]
pub struct TokenStore {
    users: DashSet<String>,
}

#[async_trait::async_trait]
impl BannedTokenStore for TokenStore {
    async fn ban(&self, token: &str) {
        self.users.insert(token.to_owned());
    }

    async fn is_banned(&self, token: &str) -> bool {
        self.users.contains(token)
    }
}