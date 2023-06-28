### Adding the firestore logic

#### Signup

Let's now replace our dummy repository logic by actual queries.
To run your program with the emulator, make sure to have `FIRESTORE_EMULATOR_HOST` defined like so : `FIRESTORE_EMULATOR_HOST="localhost:4510" PROJECT_ID=omelettedufromage cargo run`.

A naive version of our `signup` function would look a little bit like this :

```rust
    pub async fn signup(
        &self,
        username: String,
        email: String,
        password: String,
    ) -> Result<String, FirestoreError> {
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
            Ok(user) => Ok(user.id.expect("This should never happen")),
            Err(error) => Err(error),
        }
    }
```

Note that thanks to the firestore library, and the serde annotation we used earlier, the `id` field of our struct is automatically filled when we insert the document.

#### Login

You may have noticed that the return type of login is a little bit more complex that signup. This is because it handles both technical errors, but also possible user mistakes.

A possible implementation could be the following :

```rust
    pub async fn login(
        &self,
        username: String,
        password: String,
    ) -> Result<Option<String>, FirestoreError> {
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
            // No error, but we did not find a user.
            _ => Ok(None),
        }
    }
```

Now run your program and try it out !