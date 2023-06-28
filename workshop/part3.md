### Step 3: Create our GRPC server with Tonic

Now that we have played around with basic Rust functions, let's create something more interesting.

Let's start by adding Tonic and Tokio to our `Cargo.toml` dependencies:

Run the following command :

`cargo add prost tonic tokio -F tokio/rt-multi-thread -F tokio/macros && cargo add --build tonic-build` 

You should now see the dependencies section of your `Cargo.toml` looking like this :

```toml
[dependencies]
prost = "0.11.9"
tokio = { version = "1.28.2", features = ["rt-multi-thread", "macros"] }
tonic = "0.9.2"

[build-dependencies]
tonic-build = "0.9.2"
```

We'll be using `build.rs` to generate the code from our protobuf file (`hello_world.proto`).
Create a `protos` directory, and add the following content to the `hello_world.proto` file.


```protobuf
syntax = "proto3";

package hello_world;

service Greeter {
  rpc SayHello (HelloRequest) returns (HelloReply);
}

message HelloRequest {
  string name = 1;
}

message HelloReply {
  string message = 1;
}
```

Create a `build.rs` in your project root (not in `src/`), and include this code to generate Rust from the proto:

```rust
fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("protos/hello_world.proto")?;
    Ok(())
}
```

Now, we can create our server in `main.rs`:

```rust
mod hello_world {
    tonic::include_proto!("hello_world");
}

use hello_world::{greeter_server::{Greeter, GreeterServer}, HelloRequest, HelloReply};
use tonic::{transport::Server, Request, Response, Status};

pub struct MyGreeter;

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello(
        &self,

        request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {
        let reply = hello_world::HelloReply {
            message: format!("Hello {}!", request.into_inner().name),
        };
        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "0.0.0.0:50051".parse()?;
    let greeter = MyGreeter;

    Server::builder()
        .add_service(GreeterServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}
```

Here we introduce Rust's Trait system with `impl Greeter for MyGreeter`. This allows us to define shared behavior; think of them like TypeScript interfaces but on steroids.

You can now run your server with `cargo run` and try it out with the grpc client of your choice.

#### About `build.rs`

In Rust, **`build.rs`** is a special file that's part of the build process. This file is executed by Cargo before your package is built, which makes it a powerful tool to handle any pre-build steps.

In this workshop, we're using **`build.rs`** to compile our Protocol Buffers into Rust code. The **`tonic_build::compile_protos`** function call takes care of this. It reads the **`.proto`** files and generates the corresponding Rust code, which we can then use in our application.

Compared to TypeScript, Rust's **`build.rs`** can be seen as a more integrated and powerful version of the "scripts" section in **`package.json`**. However, it's more than just a place to put scripts. It's a fully-fledged Rust program, meaning you can use all of Rust's features, libraries, and error handling capabilities to define your build process.

Think about how you would set up a TypeScript project to work with Protocol Buffers. You'd have to install a Protocol Buffers compiler, write scripts to call it on your **`.proto`** files, potentially write more scripts to move the generated code to the correct location, and so on. It's a lot of manual work and boilerplate code.

With Rust's **`build.rs`**, all that complexity is abstracted away. You write a few lines of Rust code, and you get a robust, reliable, and efficient build process that integrates seamlessly with the rest of your Rust toolchain.

This is just one example of how Rust's design, focusing on zero-cost abstractions and toolchain integration, can make complex tasks simpler and more enjoyable.

[Now let's make our program a little bit more useful !](./part4.md)