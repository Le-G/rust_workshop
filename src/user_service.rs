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

        match repo
            .signup(
                payload.username.clone(),
                payload.email.clone(),
                payload.password.clone(),
            )
            .await
        {
            Ok(user_id) => Ok(Response::new(SignupResponse { user_id })),
            Err(_) => Err(Status::internal("Could not create account")),
        }
    }

    async fn login(
        &self,
        request: Request<LoginRequest>,
    ) -> Result<Response<LoginResponse>, Status> {
        let payload = request.get_ref();
        let repo = UserRepository::new(self.db.clone());

        match repo
            .login(payload.username.clone(), payload.password.clone())
            .await
        {
            Ok(Some(token)) => Ok(Response::new(LoginResponse { token })),
            Ok(None) => Err(Status::unauthenticated("Wrong login or passowrd")),
            Err(_) => Err(Status::internal("Could not login")),
        }
    }
}
