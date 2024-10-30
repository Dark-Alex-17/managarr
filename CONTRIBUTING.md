# Contributing
Contributors are very welcome! **No contribution is too small and all contributions are valued.**

You'll need to have the stable Rust toolchain installed in order to develop Managarr.

The Rust toolchain (stable) can be installed via rustup using the following command:

```shell
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

This will install `rustup`, `rustc` and `cargo`. For more information, refer to the [official Rust installation documentation](https://www.rust-lang.org/tools/install). 

## Setup workspace

1. Clone this repo
2. Run `cargo test` to setup hooks
3. Make changes
4. Run the application using `make run` or `cargo run`
5. Commit changes. This will trigger pre-commit hooks that will run format, test and lint. If there are errors or warnings from Clippy, please fix them.
6. Push your code to a new branch named after the feature/bug/etc. you're adding. This will trigger pre-push hooks that will run lint and test.
7. Create a PR

## Questions? Reach out to me!
If you encounter any questions while developing Managarr, please don't hesitate to reach out to me at alex.j.tusa@gmail.com. I'm happy to help contributors, new and experienced in any way I can!
