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

#### Adding some tests

Rust provides first-class support for testing through its built-in test framework. This is a significant advantage over TypeScript, where you would have to decide on a testing library yourself. In Rust, tests are written in the same files as the code and are run using **`cargo test`**.

Let's add some tests for our **`calc::divide`** function. In Rust, tests are simply functions annotated with **`#[test]`**, and they go inside a **`#[cfg(test)] mod tests { ... }`** block in the same file. The **`#[cfg(test)]`** attribute tells Rust to compile and run the test code only when you run **`cargo test`**, not when you run **`cargo build`**.

Go back to **`calc.rs`** and add the following code at the end of the file:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_divide() {
        assert_eq!(divide(10, 2).unwrap(), 5);
    }

    #[test]
    fn test_divide_by_zero() {
        assert!(divide(10, 0).is_err());
    }
}
```

Here, we're using **`assert_eq!`** to check that our function returns the correct result when dividing two numbers, and **`assert!`** to check that it correctly returns an error when trying to divide by zero. These are two of the most basic assertion macros that Rust provides for testing.

You can run these tests with **`cargo test`**. Cargo automatically finds these tests and runs them, captures the output, and reports whether they passed or not.

Rust's testing framework also supports more advanced features, like setup functions (**`#[test_case]`**), ignored tests (**`#[ignore]`**), and custom assertion messages, but these basics should be enough to get you started.

Testing is an integral part of Rust's philosophy of reliability. It complements the type system and ownership model by catching logic errors at development time, before they can turn into runtime errors. And as we've seen, it's convenient and straightforward to use. You can write tests as soon as you've written your functions, run them at any time with a single command, and trust that if they pass, your code is correct.

Now let's [add our GRPC server](./part3.md).