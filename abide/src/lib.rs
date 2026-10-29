pub mod internode {
    include!(concat!(env!("OUT_DIR"), "/internode.rs"));
}

use internode::internode_server::{Internode, InternodeServer};
use internode::{PingRequest, PingResponse};
use tonic::{Request, Response, Status};

pub struct InternodeService {
    pub node_id: String,
}

#[tonic::async_trait]
impl Internode for InternodeService {
    async fn ping(&self, _request: Request<PingRequest>) -> Result<Response<PingResponse>, Status> {
        Ok(Response::new(PingResponse {
            from: self.node_id.clone(),
        }))
    }
}

pub async fn serve(node_id: String, addr: std::net::SocketAddr) -> Result<(), tonic::transport::Error> {
    tonic::transport::Server::builder()
        .add_service(InternodeServer::new(InternodeService { node_id }))
        .serve(addr)
        .await
}
