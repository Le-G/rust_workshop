### Implement UserService and UserRepository

#### Add the new proto

Now that you have a running gRPC server, it's time to add more functionality to
it.

First, let's create the `user.proto` file in the `protos` directory with the
following content to reflect the new UserService.

```protobuf
syntax = "proto3";

package user;

service UserService {
  rpc Signup(SignupRequest) returns (SignupResponse) {}
  rpc Login(LoginRequest) returns (LoginResponse) {}
}

message SignupRequest {
  string username = 1;
  string email = 2;
  string password = 3;
}

message SignupResponse {
  string user_id = 1;
}

message LoginRequest {
  string username = 1;
  string password = 2;
}

message LoginResponse {
  string token = 1;
}
```

This defines two RPCs, `Signup` and `Login`. `Signup` will take a
`SignupRequest` containing a username, email, and password, and will return a
`SignupResponse` containing the new user's id. `Login` will take a
`LoginRequest` with a username and password and will return a `LoginResponse`
with a token.

In build.rs, replace `hello_world.proto` by `user.proto`.

#### Add dependencies

Now it’s time to add a firestore client and a serialization library with the
following command :

`cargo add firestore serde`

#### Add the user entity

Create a new `user.rs` file with the following content :

```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    #[serde(alias = "_firestore_id")]
    pub id: Option<String>,
    pub username: String,
    pub email: String,
    pub password: String,
}
```

#### Add the user repository

Now create a `user_repository.rs` file.

```rust
use firestore::{errors:FirestoreError, FirestoreDb};

static COLLECTION_NAME: &str = "users";

// The UserRepository struct will serve as the repository layer for accessing Firestore
pub struct UserRepository {
    db: FirestoreDb,
}

impl UserRepository {
    pub fn new(db: FirestoreDb) -> Self {
        Self { db }
    }

    // Define the methods for accessing Firestore here
    pub async fn signup(
        &self,
        username: String,
        email: String,
        password: String,
    ) -> Result<String, FirestoreError> {
        // Add your Firestore operations here
        Ok("ok".to_string())
    }

    pub async fn login(
        &self,
        username: String,
        password: String,
    ) -> Result<Option<String>, FirestoreError> {
        // Add your Firestore operations here
        Ok(Some("my_token".to_string()))
    }
}
```

Here we are defining a `UserRepository` struct which will be used for Firestore
operations, and two methods `signup` and `login` for the respective operations.

#### Creating the service

We can now create our GRPC service in a `user_service.rs` file.

```rust
use crate::{
    user_grpc_service::{
        user_service_server::UserService, LoginRequest, LoginResponse, SignupRequest,
        SignupResponse,
    },
    user_repository::UserRepository,
};
use firestore::FirestoreDb;
use tonic::{Request, Response, Status};

pub struct MyUserService {
    db: FirestoreDb,
}

impl MyUserService {
    pub fn new(db: FirestoreDb) -> Self {
        Self { db }
    }
}

#[tonic::async_trait]
impl UserService for MyUserService {
    async fn signup(
        &self,
        request: Request<SignupRequest>,
    ) -> Result<Response<SignupResponse>, Status> {
        let payload = request.get_ref();
        let repo = UserRepository::new(self.db.clone());
        let signup_result = repo
            .signup(
                payload.username.clone(),
                payload.email.clone(),
                payload.password.clone(),
            )
            .await;

        // Use match on signup_result to return the appropriate grpc response.
    }

    async fn login(
        &self,
        request: Request<LoginRequest>,
    ) -> Result<Response<LoginResponse>, Status> {
        let payload = request.get_ref();
        let repo = UserRepository::new(self.db.clone());
        let login_result = repo
            .login(payload.username.clone(), payload.password.clone())
            .await;

        //Use match on login_result to return the appropriate grpc response.
    }
}
```

Now, update the main function to include the Firestore client and pass it
through to the UserService.

```rust
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
```

As you may have noticed, the results coming from the repository are not properly
handled in the service. Use `match` to handle all the cases to return the
expected grpc response.

When this is done and that your code compiles, you can move to the
[final part](./part6.md) !
