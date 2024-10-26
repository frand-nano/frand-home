use mysql::{prelude::Queryable, Pool, PooledConn};

use crate::CONFIG;

pub struct Database {
    pool: Pool,
}

impl Database {
    pub fn new(host: &str) -> anyhow::Result<Self> {
        let mysql_user = CONFIG.keys.mysql_user();
        let mysql_pass = CONFIG.keys.mysql_pass();
        let mysql_database = CONFIG.keys.mysql_database();
        let url = format!("mysql://{mysql_user}:{mysql_pass}@{host}:3306/{mysql_database}");

        let result = Self {
            pool: Pool::new(url.as_str())?,
        };

        let mut conn = result.get_connection()?;
        
        conn.query_drop(format!(r#"
            CREATE TABLE IF NOT EXISTS music (
                music_id CHAR(12) PRIMARY KEY,
                youtube_title NVARCHAR(100),
                title NVARCHAR(100),
                artist NVARCHAR(100),
                upload_by NVARCHAR(100),
                lyrics_by NVARCHAR(100),
                volume INT
            )
        "#))?;

        conn.query_drop(format!(r#"
            CREATE TABLE IF NOT EXISTS music_list (
                id INT PRIMARY KEY,
                list_id CHAR(40) NOT NULL,
                music_id CHAR(12) NOT NULL,
	            FOREIGN KEY (music_id) references music(music_id)
            )
        "#))?;

        Ok(result)
    }

    pub fn get_connection(&self) -> anyhow::Result<PooledConn> {
        Ok(self.pool.get_conn()?)
    }
}