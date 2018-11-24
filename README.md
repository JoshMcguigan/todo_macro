# Todo Macro

Todo macro ensures you never forget about your in-code todos.  

## Usage

```rust
// todo fix this
let x = 5 - 4;
```

The todo above can simply be forgotten or ignored. Using Todo macro looks like this:

```rust
todo!("Convert this to let x = 1; by the end of the year", "2018-12-31");
let x = 5 - 4;
``` 

The todo macro ensures todos are fixed by their deadline, or the code will not compile (and the developer will see a helpful error message). If the deadline has not passed, the check is stripped out and there is no trace of it in the compiled code.  

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
