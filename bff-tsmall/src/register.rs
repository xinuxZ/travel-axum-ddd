use std::sync::Arc;

use tracing::info;

use crate::application::{
    argon::ArgonSecurityService, articles::AppArticlesService, comments::AppCommentsService, jwt::JwtService,
    profiles::AppProfilesService, suppliers::AppSupplierService, tags::AppTagsService, users::AppUsersService,
};
use crate::domain::{
    articles::{DynArticlesRepository, DynArticlesService},
    comments::{DynCommentsRepository, DynCommentsService},
    profiles::{DynProfilesRepository, DynProfilesService},
    // suppliers::service::DynSuppliersService,
    suppliers::repository::DynSuppliersRepository,
    tags::{DynTagsRepository, DynTagsService},
    users::{DynUsersRepository, DynUsersService},
    utils::{DynSecurityService, DynTokenService},
};
use crate::infrastructure::dao::supplier_dto::SupplierDAO;
use crate::infrastructure::{
    config::AppConfig,
    mysql_connection_pool::MySqlConnectionPool,
    pg_connection_pool::PgConnectionPool,
    repositories::{
        articles_repository::PostgresArticlesRepository, comments_repository::PostgresCommentsRepository,
        profiles_repository::PostgresProfilesRepository, suppliers_repository::SupplierRepository,
        tags_repository::PostgresTagsRepository, users_repository::PostgresUsersRepository,
    },
};

#[derive(Clone)]
pub struct ServiceRegister {
    pub users_service: DynUsersService,
    pub token_service: DynTokenService,
    pub profiles_service: DynProfilesService,
    pub tags_service: DynTagsService,
    pub articles_service: DynArticlesService,
    pub comments_service: DynCommentsService,
    // pub suppliers_service: DynSuppliersService,
    pub suppliers_service: AppSupplierService,
}

/// A simple service container responsible for managing the various services our API endpoints will pull from through axum extensions.
impl ServiceRegister {
    pub fn new(pg_pool: PgConnectionPool, mysql_pool: MySqlConnectionPool, config: Arc<AppConfig>) -> Self {
        info!("initializing utility services...");
        let security_service = Arc::new(ArgonSecurityService::new(config.clone())) as DynSecurityService;
        let token_service = Arc::new(JwtService::new(config)) as DynTokenService;

        info!("utility services initialized, building feature services...");
        let users_repository = Arc::new(PostgresUsersRepository::new(pg_pool.clone())) as DynUsersRepository;
        let users_service = Arc::new(AppUsersService::new(
            users_repository.clone(),
            security_service,
            token_service.clone(),
        )) as DynUsersService;

        let profiles_repository = Arc::new(PostgresProfilesRepository::new(pg_pool.clone())) as DynProfilesRepository;
        let profiles_service =
            Arc::new(AppProfilesService::new(users_repository.clone(), profiles_repository)) as DynProfilesService;

        let tags_repository = Arc::new(PostgresTagsRepository::new(pg_pool.clone())) as DynTagsRepository;
        let tags_service = Arc::new(AppTagsService::new(tags_repository.clone())) as DynTagsService;

        let articles_repository = Arc::new(PostgresArticlesRepository::new(pg_pool.clone())) as DynArticlesRepository;
        let articles_service =
            Arc::new(AppArticlesService::new(articles_repository.clone(), tags_repository)) as DynArticlesService;

        let comments_repository = Arc::new(PostgresCommentsRepository::new(pg_pool)) as DynCommentsRepository;
        let comments_service =
            Arc::new(AppCommentsService::new(comments_repository, articles_repository)) as DynCommentsService;

        // 供应商依赖注入
        let supplier_dao = SupplierDAO::new(mysql_pool);
        let suppliers_repository = Arc::new(SupplierRepository::new(supplier_dao)) as DynSuppliersRepository;
        let suppliers_service = AppSupplierService::new(suppliers_repository.clone());
        // let suppliers_service = Arc::new(AppSupplierService::new(suppliers_repository.clone())) as DynSuppliersService;

        info!("feature services successfully initialized!");

        ServiceRegister {
            users_service,
            token_service,
            profiles_service,
            tags_service,
            articles_service,
            comments_service,
            suppliers_service,
        }
    }
}
