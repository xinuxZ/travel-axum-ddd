// use anyhow::Context;

use crate::infrastructure::mysql::model::product::PageProductRequest;
use crate::infrastructure::mysql::model::product::ProductDO;
use crate::infrastructure::mysql::model::product::ProductRequest;
use crate::infrastructure::mysql_connection_pool::MySqlConnectionPool;
use crate::infrastructure::paginate::Paginate;

#[derive(Clone)]
pub struct ProductDAO {
    pool: MySqlConnectionPool,
}
impl ProductDAO {
    pub fn new(pool: MySqlConnectionPool) -> Self {
        Self { pool }
    }
}

impl ProductDAO {
    pub async fn list(&self, r: ProductRequest) -> anyhow::Result<Vec<ProductDO>> {
        let mut q = sqlx::QueryBuilder::new(
            r#"SELECT `id`,`tenant_id`,`supplier_id`,`product_title`,`product_category_id`,`cooperation_id`,
            `product_type`,`origin_amount`,`actual_amount`,`settlement_amount`,`commision_amount`,`service_type`,
            `created_at`,`updated_at`,`deleted_at` FROM product WHERE 1=1 "#,
        );

        if let Some(id) = &r.id {
            q.push(" AND id =").push_bind(id);
        }
        if let Some(tenant_id) = &r.tenant_id {
            q.push(" AND tenant_id =").push_bind(tenant_id);
        }

        if let Some(supplier_id) = &r.supplier_id {
            q.push(" AND supplier_id =").push_bind(supplier_id);
        }

        let data = q.build_query_as().fetch_all(&self.pool).await?;
        // .context(" and unexpected error occured list product");

        Ok(data)
    }

    pub async fn page(&self, r: &PageProductRequest) -> anyhow::Result<Paginate<ProductDO>> {
        let mut q = sqlx::QueryBuilder::new(
            r#"SELECT `id`,`tenant_id`,`supplier_id`,`product_title`,`product_category_id`,`cooperation_id`,
            `product_type`,`origin_amount`,`actual_amount`,`settlement_amount`,`commision_amount`,`service_type`,
            `created_at`,`updated_at`,`deleted_at` FROM product WHERE 1=1 "#,
        );

        let mut qc = sqlx::QueryBuilder::new(r#"SELECT COUNT(*) FROM product WHERE 1=1"#);

        if let Some(id) = &r.id {
            q.push(" AND id =").push_bind(id);
            qc.push(" AND id =").push_bind(id);
        }
        if let Some(tenant_id) = &r.tenant_id {
            q.push(" AND tenant_id =").push_bind(tenant_id);
            qc.push(" AND tenant_id =").push_bind(tenant_id);
        }

        if let Some(supplier_id) = &r.supplier_id {
            q.push(" AND supplier_id =").push_bind(supplier_id);
            qc.push(" AND supplier_id =").push_bind(supplier_id);
        }

        q.push(" LIMIT ")
            .push_bind(r.paginate.page_size)
            .push(" OFFSET ")
            .push_bind(r.paginate.offset());

        let count = qc.build_query_as().fetch_one(&self.pool).await?;
        // .context(" product page err");

        let data = q.build_query_as().fetch_all(&self.pool).await?;
        // .context(" and unexpected error occured list product");

        Ok(Paginate::quick(&r.paginate, &count, data))
    }

    pub async fn get(&self, id: i64) -> anyhow::Result<Option<ProductDO>> {
        let mut q = sqlx::QueryBuilder::new(
            r#"SELECT `id`,`tenant_id`,`supplier_id`,`product_title`,`product_category_id`,`cooperation_id`,
            `product_type`,`origin_amount`,`actual_amount`,`settlement_amount`,`commision_amount`,`service_type`,
            `created_at`,`updated_at`,`deleted_at` from product WHERE 1=1 "#,
        );

        q.push(" AND id =").push_bind(id);
        let data = q.build_query_as().fetch_optional(&self.pool).await?;
        // .context(" and unexpected error occured list product");

        Ok(data)
    }

    // pub async fn get(&self, id: i64) -> anyhow::Result<Option<ProductDO>> {
    //     // 选择适用 query_as 宏，因为适用宏可以在编译阶段校验SQL
    //     sqlx::query_as!(
    //         ProductDO,
    //         r#"SELECT `id`,`tenant_id`,`supplier_id`,`product_title`,`product_category_id`,`cooperation_id`,
    //         `product_type`,`origin_amount`,`actual_amount`,`settlement_amount`,`commision_amount`,`service_type`,
    //         `created_at`,`updated_at`,`deleted_at` FROM product where id = ?"#,
    //         id
    //     )
    //     .fetch_optional(&self.pool)
    //     .await
    //     .context("an unexpected error occured query product")
    // }
}
