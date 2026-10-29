mod wal;
use std::sync::Arc;

use crate::{
    client::{FileSystemClient, InterNodeClient, ObjectStoreClient, StateMachineClient},
    config::Config,
};

pub struct Node {
    cfg: Arc<Config>,
    fs_client: Arc<dyn FileSystemClient>,
    object_store: Arc<dyn ObjectStoreClient>,
    state_machine: Arc<dyn StateMachineClient>,
    inter_node: Arc<dyn InterNodeClient>,
    wal: Arc<wal::WAL>,
}
