#[derive(clap::Parser)]
pub struct AppConfig {
    #[clap(long, env)]
    pub host: String,

    #[clap(long, env)]
    pub port: u32,

    #[clap(long, env)]
    pub cors_origin: String,

    #[clap(long, env)]
    pub mysql_url: String,

    #[clap(long, env)]
    pub mysql_host: String,
    #[clap(long, env)]
    pub mysql_port: u16,
    #[clap(long, env)]
    pub mysql_username: String,
    #[clap(long, env)]
    pub mysql_password: String,

    #[clap(long, env)]
    pub database_url: String,

    #[clap(long, env)]
    pub rust_log: String,

    #[clap(long, env)]
    pub log_path: String,

    #[clap(long, env)]
    pub argon_salt: String,

    #[clap(long, env)]
    pub token_secret: String,

    #[clap(long, env)]
    pub run_migrations: bool,

    #[clap(long, env)]
    pub seed: bool,
}
