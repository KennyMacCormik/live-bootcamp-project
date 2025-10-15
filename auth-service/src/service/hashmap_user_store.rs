use std::collections::HashMap;
use std::collections::hash_map::Entry;
use crate::domain::{User, data_stores::UserStoreError};
use crate::domain::data_stores::UserStore;

#[derive(Default)]
pub struct HashmapUserStore {
    users: HashMap<String, User>,
}

#[async_trait::async_trait]
impl UserStore for HashmapUserStore {
    async fn add_user(&mut self, user: User) -> Result<(), UserStoreError> {
        match self.users.entry(user.get_email().to_owned()) {
            Entry::Occupied(_) => Err(UserStoreError::UserAlreadyExists),
            Entry::Vacant(v) => {
                v.insert(user);
                Ok(())
            }
        }
    }

    // TODO: Implement a public method called `get_user`, which takes an
    // immutable reference to self and an email string slice as arguments.
    // This function should return a `Result` type containing either a
    // `User` object or a `UserStoreError`.
    // Return `UserStoreError::UserNotFound` if the user can not be found.

    async fn validate_user(&self, email: &str, password: &str) -> Result<(), UserStoreError> {
        if let Some(user) = self.users.get(email) {
            if user.get_password() == password {
                return Ok(());
            }

            return Err(UserStoreError::InvalidCredentials)
        }

        Err(UserStoreError::UserNotFound)
    }
}

// TODO: Add unit tests for your `HashmapUserStore` implementation
#[cfg(test)]
mod tests {
    use super::*;

    // #[tokio::test]
    // fn test_add_user() {
    //     todo!()
    // }
    //
    // #[tokio::test]
    // fn test_get_user() {
    //     todo!()
    // }
    //
    // #[tokio::test]
    // fn test_validate_user() {
    //     todo!()
    // }
}