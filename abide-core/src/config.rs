use bon::Builder;

#[derive(Builder, Debug, Clone)]
pub struct Config {
    pub identity: Option<String>,
    pub store_config: ObjectStoreConfig,
    pub wal_config: WALConfig,
    pub lsm_tree_config: LSMTreeConfig,
}

#[derive(Builder, Debug, Clone)]
pub struct WALConfig {
    #[builder(default = "wal".to_string())]
    pub relative_path: String,

    #[builder(default = "abide_data/wal".to_string())]
    pub local_path: String,
}

#[derive(Builder, Debug, Clone)]
pub struct LSMTreeConfig {
    #[builder(default = "lsm_tree".to_string())]
    pub relative_path: String,

    #[builder(default = "abide_data/lsm_tree".to_string())]
    pub local_cache_path: String,

    #[builder(default = 16)]
    pub local_cache_size_gb: usize,
}

#[derive(Debug, Clone)]
pub enum ObjectStoreConfig {
    S3 {
        key: Option<String>,
        secret: Option<String>,
    },
    AzureBlobStorage {
        key: Option<String>,
        secret: Option<String>,
    },
}
