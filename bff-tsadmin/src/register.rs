use std::sync::Arc;

use tracing::info;

use crate::application::argon::ArgonSecurityService;
use crate::application::jwt::JwtService;
use crate::application::product::ProductAppService;
use crate::application::suppliers::SupplierAppService;
use crate::domain::product::repository::DynProductRepository;
use crate::domain::suppliers::repository::DynSuppliersRepository;
use crate::domain::{DynSecurityService, DynTokenService};
use crate::infrastructure::mysql::{product::ProductDAO, supplier::SupplierDAO};
use crate::infrastructure::mysql_connection_pool::MySqlConnectionManager;
// use crate::infrastructure::mysql_connection_pool;
use crate::infrastructure::config::AppConfig;
use crate::infrastructure::repositories::{product::MySqlProductRepository, supplier::MySqlSupplierRepository};

#[derive(Clone)]
pub struct ServiceRegister {
    pub token_service: DynTokenService,
    pub security_service: DynSecurityService,
    pub suppliers_service: SupplierAppService,
    pub product_service: ProductAppService,
}

/// A simple service container responsible for managing the various services our API endpoints will
/// pull from through axum extensions.
impl ServiceRegister {
    pub async fn new(mysql_manager: &MySqlConnectionManager, config: Arc<AppConfig>) -> Self {
        // let mut mysql_connection_options = mysql_pool.connect_options();

        info!("initializing utility services...");
        let security_service = Arc::new(ArgonSecurityService::new(config.clone())) as DynSecurityService;
        let token_service = Arc::new(JwtService::new(config)) as DynTokenService;

        let mysql_pool_srm = mysql_manager
            .clone()
            .pool_with_db("saas_srm")
            .await
            .expect("mysql error");
        info!("utility services initialized, building feature services...");
        let supplier_dao = SupplierDAO::new(mysql_pool_srm);
        let suppliers_repository = Arc::new(MySqlSupplierRepository::new(supplier_dao)) as DynSuppliersRepository;
        let suppliers_service = SupplierAppService::new(suppliers_repository.clone());

        info!("utility services initialized, building feature services...");
        let mysql_pool_pms = mysql_manager
            .clone()
            .pool_with_db("saas_pms")
            .await
            .expect("mysql error");
        let product_dao = ProductDAO::new(mysql_pool_pms.clone());
        let product_repository = Arc::new(MySqlProductRepository::new(product_dao)) as DynProductRepository;
        let product_service = ProductAppService::new(product_repository.clone());

        info!("feature services successfully initialized!");

        ServiceRegister {
            token_service,
            security_service,
            suppliers_service,
            product_service,
        }
    }
}
