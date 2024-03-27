use crate::domain::profiles::MockProfilesRepository;
use crate::domain::users::MockUsersRepository;
use crate::domain::utils::MockSecurityService;
use crate::domain::utils::MockTokenService;

pub struct UsersServiceTestFixture {
    pub mock_repository: MockUsersRepository,
    pub mock_token_service: MockTokenService,
    pub mock_security_service: MockSecurityService,
}

pub struct ProfilesServiceTestFixture {
    pub mock_profiles_repository: MockProfilesRepository,
    pub mock_users_repository: MockUsersRepository,
}

impl Default for UsersServiceTestFixture {
    fn default() -> Self {
        UsersServiceTestFixture::new()
    }
}

impl UsersServiceTestFixture {
    pub fn new() -> Self {
        Self {
            mock_repository: MockUsersRepository::new(),
            mock_token_service: MockTokenService::new(),
            mock_security_service: MockSecurityService::new(),
        }
    }
}

impl ProfilesServiceTestFixture {
    pub fn new() -> Self {
        Self {
            mock_profiles_repository: MockProfilesRepository::new(),
            mock_users_repository: MockUsersRepository::new(),
        }
    }
}

impl Default for ProfilesServiceTestFixture {
    fn default() -> Self {
        ProfilesServiceTestFixture::new()
    }
}
