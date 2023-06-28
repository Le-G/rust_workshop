use crate::user::User;
use firestore::{errors::FirestoreError, path_camel_case, struct_path, FirestoreDb};

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
        let user = User {
            id: None,
            username,
            email,
            password,
        };

        let result: Result<User, FirestoreError> = self
            .db
            .fluent()
            .insert()
            .into(COLLECTION_NAME)
            .generate_document_id()
            .object(&user)
            .execute()
            .await;

        match result {
            Ok(user) => Ok(user.id.unwrap()),
            Err(error) => Err(error),
        }
    }

    pub async fn login(
        &self,
        username: String,
        password: String,
    ) -> Result<Option<String>, FirestoreError> {
        // Add your Firestore operations here

        let user_result: Result<Vec<User>, FirestoreError> = self
            .db
            .fluent()
            .select()
            .from(COLLECTION_NAME)
            .filter(|q| {
                q.for_all({
                    [
                        q.field(path_camel_case!(User::username)).equal(&username),
                        q.field(path_camel_case!(User::password)).equal(&password),
                    ]
                })
            })
            .obj()
            .query()
            .await;

        match user_result {
            Ok(users) if users.len() == 1 => Ok(Some("my_token".to_string())),
            Err(error) => Err(error),
            _ => Ok(None),
        }
    }
}
