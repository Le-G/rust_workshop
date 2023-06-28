mod user_grpc_service {
    tonic::include_proto!("user");
}

use std::env;

use firestore::FirestoreDb;
use tonic::transport::Server;
use user_grpc_service::user_service_server::UserServiceServer;
use user_service::MyUserService;

mod user;
mod user_repository;
mod user_service;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Create an instance

    let db = FirestoreDb::new(&env::var("PROJECT_ID").unwrap()).await?;

    let addr = "0.0.0.0:50051".parse().unwrap();
    let user_service = UserServiceServer::new(MyUserService::new(db.clone()));

    Server::builder()
        .add_service(user_service)
        .serve(addr)
        .await?;

    Ok(())
}
