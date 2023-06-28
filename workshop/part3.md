### Step 3: Adding some tests

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

Now let's [add our GRPC server](./part4.md).