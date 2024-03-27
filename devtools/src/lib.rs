#[macro_use]
extern crate rbatis;
pub mod core;
pub use crate::core::*;
use handlebars::Handlebars;
use rbatis::{crud::CRUD, rbatis::Rbatis};

// use handlebars::to_json;
use serde_json::value::{Map, Value as Json};
use std::fs::File;

const TEMP: [(&str, &str, &str); 4] = [
    (
        "service_template",
        "service",
        "/Users/zxl/Sites/rust/rust-axum-sqlx/template/service_template.hbs",
    ),
    (
        "resource_template",
        "resource",
        "/Users/zxl/Sites/rust/rust-axum-sqlx/template/resources_template.hbs",
    ),
    (
        "dto_template",
        "dto",
        "/Users/zxl/Sites/rust/rust-axum-sqlx/template/dto_template.hbs",
    ),
    (
        "entity_template",
        "entity",
        "/Users/zxl/Sites/rust/rust-axum-sqlx/template/entity_template.hbs",
    ),
];

pub async fn init_rbatis(database_url: &str) -> Rbatis {
    let rbatis = Rbatis::new();
    rbatis.link(database_url).await.expect("rbatis link database fail!");
    return rbatis;
}

pub async fn get_columns(table_name: &str) -> Vec<Field> {
    let rb = init_rbatis("mysql://root:xinuxZ@localhost:3306/saas_srm").await;
    let w = rb
        .new_wrapper()
        .eq("table_name", table_name)
        .eq("table_schema", "saas_srm");
    let columns = rb.fetch_list_by_wrapper::<TableColumns>(w).await.unwrap();
    columns.into_iter().map(|c| c.into()).collect()
}

pub fn render_template(table_name: &str, data: &Map<String, Json>) {
    let mut handlebars = Handlebars::new();
    for (temp_name, file_path, temple_path) in TEMP {
        println!("{}", file_path);

        handlebars.register_template_file(temp_name, temple_path).unwrap();
        let mut output_dtofile = File::create(format!(
            "/Users/zxl/Sites/rust/rust-axum-sqlx/target/gen_rust/{}_{}.rs",
            table_name,
            file_path.to_string()
        ))
        .unwrap();
        handlebars
            .render_to_write(temp_name, &data, &mut output_dtofile)
            .unwrap();
    }
}
