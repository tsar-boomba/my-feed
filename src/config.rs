use std::{fs, net::SocketAddr, path::Path, sync::Arc};

use http::Uri;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    #[cfg_attr(debug_assertions, serde(default = "dev_domain"))]
    #[serde(with = "http_serde::uri")]
    pub domain: Uri,

    #[serde(default = "default_listen_addr")]
    pub listen_addr: SocketAddr,

    #[serde(default)]
    pub multi_thread_runtime: bool,

    #[serde(default)]
    pub runtime_workers: Option<usize>,

    #[serde(default = "default_env_filter")]
    pub tracing_filter: Arc<str>,

    #[serde(default = "default_web_dir")]
    pub web_dir: Arc<Path>,

    #[serde(default = "default_data_dir")]
    pub data_dir: Arc<Path>,

    #[serde(default = "default_password")]
    pub password: Arc<str>,
}

impl Config {
    pub fn from_env() -> color_eyre::Result<Self> {
        Ok(serde_env::from_env()?)
    }

    pub fn from_json(json: &[u8]) -> color_eyre::Result<Self> {
        Ok(serde_json::from_slice(json)?)
    }
}

fn dev_domain() -> Uri {
    "localhost".parse().unwrap()
}

fn default_env_filter() -> Arc<str> {
    if cfg!(debug_assertions) {
        Arc::from("info,my_feed=debug,axum=debug,axum-core=debug,tower_http=debug")
    } else {
        Arc::from("info,my_feed=debug")
    }
}

/// 0.0.0.0:8080
fn default_listen_addr() -> SocketAddr {
    if cfg!(debug_assertions) {
        "127.0.0.1:8013".parse().unwrap()
    } else {
        "0.0.0.0:8013".parse().unwrap()
    }
}

fn default_web_dir() -> Arc<Path> {
    if cfg!(debug_assertions) {
        Arc::from(Path::new("./web/dist").canonicalize().unwrap())
    } else {
        Arc::from(Path::new("/my-feed-web").canonicalize().unwrap())
    }
}

fn default_data_dir() -> Arc<Path> {
    if cfg!(debug_assertions) {
        fs::create_dir("./my-feed-data").ok();
        Arc::from(Path::new("./my-feed-data").canonicalize().unwrap())
    } else {
        Arc::from(Path::new("/my-feed-data").canonicalize().unwrap())
    }
}

fn default_password() -> Arc<str> {
    if cfg!(debug_assertions) {
        Arc::from("password")
    } else {
        panic!("Must set PASSWORD env variable for production.")
    }
}
