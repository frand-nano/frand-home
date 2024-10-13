use anyhow::anyhow;
use rustls::{pki_types, ServerConfig};
use rustls_pemfile::{certs, pkcs8_private_keys};
use serde::Deserialize;
use std::{env::Args, fs::{read_to_string, File}, io::BufReader};
use hex::decode;

#[derive(Deserialize)]
pub struct Config {
    pub paths: Paths,
    pub uris: Uris,
    pub keys: Keys,
    pub settings: Settings,
}

#[derive(Deserialize)]
pub struct Paths {
    pub cert: String,
    pub privkey: String,
    pub log4rs: String,
}

#[derive(Deserialize)]
pub struct Uris {
    pub oauth_redirect: String,
    pub oauth_root: String,
    pub oauth_token: String,   
    pub oauth_userinfo: String,    
    pub oauth_scope_profile: String,
    pub oauth_scope_email: String,
    pub youtube_playlists: String,
    pub youtube_playlist_items: String,
}

#[derive(Deserialize)]
pub struct Keys {
    pub client_id: String,
    pub client_secret: String,    
    pub session_secret: String,
    pub youtube_api_key: String,
}

#[derive(Deserialize)]
pub struct Settings {
    pub local_mode: bool,
    pub port: u16,
    pub server_whitelists: Vec<String>,
    pub client_whitelists_all: bool,
    pub client_whitelists: Vec<String>,
    pub playlists: Vec<String>,
}

impl Config {
    pub fn read_from(args: Args, default_dir: &str) -> anyhow::Result<Self> {
        let args: Vec<_> = args.collect();
        let dir = args.get(1)
        .map(|args1| args1.to_owned())
        .unwrap_or(default_dir.to_owned());

        let path = format!("./{dir}/Config.toml");

        toml::from_str::<Self>(&read_to_string(&path)?)
        .map_err(|err| anyhow!("Failed to deserialize config file path: {path} err: {err}"))
    }

    pub fn read_tls_server_config(&self) -> anyhow::Result<ServerConfig> {
        let cert = self.paths.cert.as_str();
        let privkey = self.paths.privkey.as_str();

        let mut certs_file = BufReader::new(File::open(cert)?);
        let mut key_file = BufReader::new(File::open(privkey)?);

        let tls_certs = certs(&mut certs_file).collect::<Result<Vec<_>, _>>()?;

        let tls_key = pkcs8_private_keys(&mut key_file).next()
        .ok_or(anyhow!("❗ Found private key file with config, but no TLS private key in that file."))??;
        
        let server_config = ServerConfig::builder()
        .with_no_client_auth()
        .with_single_cert(tls_certs, pki_types::PrivateKeyDer::Pkcs8(tls_key))?;

        Ok(server_config)      
    }

    pub fn session_secret(&self) -> anyhow::Result<Vec<u8>> {
        let session_secret = self.keys.session_secret.as_str();        
        Ok(decode(session_secret)?)
    }
}
