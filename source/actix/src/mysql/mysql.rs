use mysql::{Pool, PooledConn};

use crate::CONFIG;

pub struct Database {
    pool: Pool,
}

impl Database {
    pub fn new(host: &str) -> anyhow::Result<Self> {
        let mysql_user = CONFIG.keys.mysql_user();
        let mysql_pass = CONFIG.keys.mysql_pass();
        let url = format!("mysql://{mysql_user}:{mysql_pass}@{host}:3306");

        Ok(Self {
            pool: Pool::new(url.as_str())?,
        })
    }

    pub fn get_connection(&self) -> anyhow::Result<PooledConn> {
        Ok(self.pool.get_conn()?)
    }
}