### Step 2: Writing and Using a Module

#### Create the module

Rust's modules allow for code organization and encapsulation. Create a new file in the `src` directory, let's call it `calc.rs`. This will be our module file.

In `calc.rs`, create a function that performs some computation and returns a `Result`:

```rust
// Don't focus on the static keyword for now, see below
pub fn divide(a: i32, b: i32) -> Result<i32, &'static str> {
    if b == 0 {
        return Err("Cannot divide by zero");
    }
    
    Ok(a / b) // Notice that we don't need to specify `return`
}
```

Here, we're introducing Rust's powerful `Result` type, which is used for error handling. This function can either succeed (return `Ok`) with an integer value or fail (return `Err`) with a static string.

In your `main.rs`, include the new module and use the function:

```rust
mod calc;

fn main() {
    let result = calc::divide(10, 0);

    match result {
        Ok(val) => println!("Result: {}", val),
        Err(e) => eprintln!("Error: {}", e),
    }
}
```

Here we encounter `match`, a fundamental control flow operator in Rust. It allows us to handle the `Result` returned by our computation.
Match requires us to handle all possible flows for a given type. You can try to remove one of the two options and see what happens when you compile the program !

Rust's macros (like `println!` and `eprintln!` here) are a way to define reusable chunks of code. Their exclamation mark is a hint at their power!

#### Explaining `Result<i32, &'static str>`

In the `divide` function, you'll notice this return type `Result<i32, &'static str>`.
As explained before, the `Result` type is a way to encapsulate any operation that can fail, like a write to a database, a network call, or anything. Because of this type, you need to **explicitly** handle every possible outcome of your operation. This forces you to handle every possible failure scenario, and makes the code very easy to understand. No more uncaught exceptions !

On the left side of the `Result` you will see the success type (which is in this case a 32bit integer).

The `Err` variant of this `Result` is a `&'static str`. The `'static` here is a lifetime. Lifetimes are Rust's way of handling memory safety without needing a garbage collector. They denote the span of time that a reference is valid.

The `'static` lifetime is a special lifetime that represents the entire duration of the program. A string literal, like the one we're returning in `Err("Cannot divide by zero")`, has a `'static` lifetime because it is embedded in the binary and is therefore always available.

Lifetimes can get complex when dealing with references to objects that can go out of scope, as Rust needs to ensure that it never keeps a reference to an object that has been deallocated. This is a fundamental part of Rust's safety guarantees, and it's a concept that doesn't have a direct equivalent in TypeScript, which uses garbage collection to automatically clean up unused memory.

However, given the focus of this workshop, we won't delve deeper into lifetimes. For now, just understand that `'static` means "this exists for the entire program duration", and that lifetimes as a whole are a powerful tool Rust provides for memory safety.

Now that we have created this function, we can proceed to [test it](./part3.md).
