use std::sync::Arc;

use crate::domain::profiles::ProfilesService;
use mockall::predicate::*;

use crate::domain::profiles::{DynProfilesRepository, UserFollowEntity};
use crate::domain::users::{DynUsersRepository, UserEntity};
use crate::infrastructure::mocks::ProfilesServiceTestFixture;
use crate::infrastructure::services::profiles_service::AppProfilesService;

#[tokio::test]
async fn return_success_when_downstream_services_succeed_and_user_exists() {
    // arrange
    let mut fixture = ProfilesServiceTestFixture::default();

    fixture
        .mock_users_repository
        .expect_get_user_by_username()
        .with(eq("stub username"))
        .times(1)
        .return_once(move |_| Ok(Some(UserEntity::default())));

    fixture
        .mock_profiles_repository
        .expect_get_user_followees()
        .times(1)
        .return_once(move |_| Ok(vec![]));

    fixture
        .mock_profiles_repository
        .expect_add_user_follow()
        .with(eq(2_i64), eq(1_i64))
        .times(1)
        .return_once(move |_, _| Ok(UserFollowEntity::default()));

    let profiles_service = AppProfilesService::new(
        Arc::new(fixture.mock_users_repository) as DynUsersRepository,
        Arc::new(fixture.mock_profiles_repository) as DynProfilesRepository,
    );

    // act
    let response = profiles_service.add_user_follow("stub username", 2_i64).await;

    // assert
    assert!(response.is_ok());
    assert!(response.unwrap().following);
}
