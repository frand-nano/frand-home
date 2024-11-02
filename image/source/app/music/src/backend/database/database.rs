use mysql::{params, prelude::Queryable, Params, Pool, PooledConn};
use mysql_common::chrono::NaiveDateTime;

use crate::{backend::config::Config, state::{client::musiclist::MusiclistItem, server::playlist::{PlaylistId, PlaylistPage}}};

pub fn init_database(
    mysql_url: &str,
) -> anyhow::Result<Pool> {
    let name = "music";
    let pool = Pool::new(format!("{mysql_url}").as_str())?;
    let mut conn = pool.get_conn()?;

    conn.query_drop(format!(r#"
        CREATE DATABASE IF NOT EXISTS {name}
    "#))?;

    let pool = Pool::new(format!("{mysql_url}/{name}").as_str())?;
    let mut conn = pool.get_conn()?;

    conn.query_drop(format!(r#"
        CREATE TABLE IF NOT EXISTS music (
            music_id CHAR(12) PRIMARY KEY,
            datetime DATETIME DEFAULT CURRENT_TIMESTAMP,
            youtube_title VARCHAR(200),
            title VARCHAR(100),
            artist VARCHAR(100),
            upload_by VARCHAR(100),
            lyrics_by VARCHAR(100),
            info VARCHAR(100),
            tag VARCHAR(100),
            volume INT
        ) 
    "#))?;

    conn.query_drop(format!(r#"
        CREATE TABLE IF NOT EXISTS musiclist (
            id CHAR(52) PRIMARY KEY,
            datetime DATETIME DEFAULT CURRENT_TIMESTAMP,
            playlist_id CHAR(40) NOT NULL,
            music_id CHAR(12) NOT NULL,
            FOREIGN KEY (music_id) references music(music_id)
        )
    "#))?;

    Ok(pool)
}

impl From<MusiclistItem::State> for Params {
    fn from(music: MusiclistItem::State) -> Self {
        params! {
            "music_id" => &music.music_id,
            "youtube_title" => &music.youtube_title,
            "title" => &music.title,
            "artist" => &music.artist,
            "upload_by" => &music.upload_by,
            "lyrics_by" => &music.lyrics_by,
            "info" => &music.tag,
            "tag" => &music.tag,
            "volume" => &music.volume,
        }
    }
}

type MusiclistItemData = (String, NaiveDateTime, String, String, String, String, String, String, String, i32);
impl From<MusiclistItemData> for MusiclistItem::State {
    fn from((
        music_id, 
        datetime,
        youtube_title, 
        title, 
        artist, 
        upload_by, 
        lyrics_by, 
        info, 
        tag, 
        volume,             
    ): MusiclistItemData) -> Self {
        Self { 
            music_id, 
            datetime: datetime.to_string(),
            youtube_title, 
            title, 
            artist, 
            upload_by, 
            lyrics_by, 
            info, 
            tag, 
            volume, 
        }
    }
}

pub fn insert_update_music(
    conn: &mut PooledConn,
    playlist_id: &PlaylistId,
    music: &MusiclistItem::State,
) -> anyhow::Result<()> {
    let params: Params = music.into(); 
    
    conn.exec_drop(
        format!(r#"
            INSERT INTO music 
                ( music_id,  youtube_title,  title,  artist,  upload_by,  lyrics_by,  info,  tag,  volume)
            VALUES
                (:music_id, :youtube_title, :title, :artist, :upload_by, :lyrics_by, :info, :tag, :volume)
            ON DUPLICATE KEY UPDATE
                music_id = :music_id,
                youtube_title = :youtube_title,
                title = :title,
                artist = :artist,
                upload_by = :upload_by,
                lyrics_by = :lyrics_by,
                info = :info,
                tag = :tag,
                volume = :volume
        "#), 
        params,
    )?;

    let params: Params = params! {
        "id" => &format!("{playlist_id}{}", music.music_id),
        "playlist_id" => playlist_id.as_str(),
        "music_id" => &music.music_id,
    }; 

    conn.exec_drop(
        format!(r#"
            INSERT IGNORE INTO musiclist 
            ( id,  playlist_id,  music_id)
            VALUES
            (:id, :playlist_id, :music_id)
        "#), 
        params,
    )?;

    Ok(())
}

pub fn select_playlist_pages(
    config: &Config,
    conn: &mut PooledConn,
    playlist_id: &PlaylistId,
) -> anyhow::Result<Vec<PlaylistPage::State>> {   
    let params: Params = params! {
        "playlist_id" => playlist_id.as_str(),
    }; 

    let count: usize = conn.exec(
        format!(r#"
            SELECT COUNT(CASE WHEN playlist_id = :playlist_id THEN 1 END) FROM musiclist
        "#),
        params,
    )?.pop().unwrap_or_default();

    let stride = config.playlist_items_max_results;
    let mut result: Vec<_> = Vec::default();

    let mut last_end = 0;
    for index in 0 .. count / stride {
        let (start, end) = (index*stride, (index+1)*stride);
        result.push(PlaylistPage::State { 
            id: playlist_id.clone(), 
            range: start..end, 
        });
        last_end = end;
    }

    if last_end < count {
        result.push(PlaylistPage::State { 
            id: playlist_id.clone(), 
            range: last_end..count, 
        });
    }

    Ok(result)
}

pub fn select_musics(
    conn: &mut PooledConn,
    page: &PlaylistPage::State,
) -> anyhow::Result<Vec<MusiclistItem::State>> {    
    let params: Params = params! {
        "playlist_id" => page.id.as_str(),
    }; 

    let (min, max) = (page.range.start.min(page.range.end), page.range.start.max(page.range.end));
    let (min, length) = (page.range.start, max - min);

    let musics: Vec<MusiclistItem::State> = conn.exec(
        format!(r#"
            SELECT music.*
            FROM musiclist
            RIGHT JOIN music
            ON musiclist.music_id = music.music_id
            WHERE musiclist.playlist_id = :playlist_id
            ORDER BY datetime, music_id
            LIMIT {min}, {length}
        "#),
        params,
    )?.into_iter()
    .map(|value: MusiclistItemData| value.into())
    .collect();

    Ok(musics)
}