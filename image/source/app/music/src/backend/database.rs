use mysql::{prelude::Queryable, Pool};

pub fn init_database(
    mysql_url: &str,
) -> anyhow::Result<Pool> {
    let name = "music";
    let pool = Pool::new(mysql_url)?;
    let mut conn = pool.get_conn()?;

    conn.query_drop(format!(r#"
        CREATE DATABASE IF NOT EXISTS {name}
    "#))?;

    let pool = Pool::new(format!("{mysql_url}/{name}").as_str())?;
    let mut conn = pool.get_conn()?;

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

    Ok(pool)
}