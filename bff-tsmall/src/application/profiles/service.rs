use async_trait::async_trait;
use tracing::info;

use crate::domain::profiles::DynProfilesRepository;
use crate::domain::profiles::ProfilesService;
use crate::domain::users::DynUsersRepository;
use crate::infrastructure::errors::{AppError, AppResult};

use super::dto::ProfileDto;
#[derive(Clone)]
pub struct AppProfilesService {
    users_repository: DynUsersRepository,
    profiles_repository: DynProfilesRepository,
}

impl AppProfilesService {
    pub fn new(users_repository: DynUsersRepository, profiles_repository: DynProfilesRepository) -> Self {
        Self {
            users_repository,
            profiles_repository,
        }
    }
}

#[async_trait]
impl ProfilesService for AppProfilesService {
    async fn get_profile(&self, username: &str, current_user_id: Option<i64>) -> AppResult<ProfileDto> {
        info!("retrieving profile for user {:?}", username);
        let user = self.users_repository.get_user_by_username(username).await?;

        if user.is_none() {
            return Err(AppError::NotFound(String::from("profile was not found")));
        }

        // in the case a token is passed and validly extracted, pull the list of users they're following to see if the profile is included
        if let Some(user_id) = current_user_id {
            info!("retrieving followee list for user {:?}", username);
            let users_following_list = self.profiles_repository.get_user_followees(user_id).await?;

            if users_following_list.is_empty() {
                return Ok(user.unwrap().into_profile(false));
            }

            let is_following = users_following_list
                .into_iter()
                .any(|followee| followee.follower_id == user_id);

            return Ok(user.unwrap().into_profile(is_following));
        }

        Ok(user.unwrap().into_profile(false))
    }

    async fn add_user_follow(&self, username: &str, current_user_id: i64) -> AppResult<ProfileDto> {
        info!(
            "add profile follow to user {:?} from user ID {:?}",
            username, current_user_id
        );
        let user = self.users_repository.get_user_by_username(username).await?;

        if user.is_none() {
            return Err(AppError::NotFound(String::from("profile to follow was not found")));
        }

        let followed_user = user.unwrap();

        // verify the user is not already following
        let is_following = self
            .profiles_repository
            .get_user_followees(current_user_id)
            .await?
            .into_iter()
            .any(|followee| followee.follower_id == current_user_id);

        if !is_following {
            self.profiles_repository
                .add_user_follow(current_user_id, followed_user.id)
                .await?;
        }

        Ok(followed_user.into_profile(true))
    }

    async fn remove_user_follow(&self, username: &str, current_user_id: i64) -> AppResult<ProfileDto> {
        info!(
            "removing profile follow to user {:?} from user ID {:?}",
            username, current_user_id
        );
        let user = self.users_repository.get_user_by_username(username).await?;

        if user.is_none() {
            return Err(AppError::NotFound(String::from("profile to follow was not found")));
        }

        let followed_user = user.unwrap();

        // verify the user is following
        let is_following = self
            .profiles_repository
            .get_user_followees(current_user_id)
            .await?
            .into_iter()
            .any(|followee| followee.follower_id == current_user_id);

        if is_following {
            self.profiles_repository
                .remove_user_follow(current_user_id, followed_user.id)
                .await?;
        }

        Ok(followed_user.into_profile(false))
    }
}
