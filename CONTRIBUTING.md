# Contributing
Contributors are very welcome! **No contribution is too small and all contributions are valued.**

## License and Attribution

#### _If you plan on contributing to the base project, no attribution is needed!_ Feel free to proceed to the [next steps](CONTRIBUTING.md#Rust).

Otherwise, below are key points to understand from the [Managarr License, Version 1.0](LICENSE):
1. **Non-Commercial Use**:
   - Managarr is licensed solely for non-commercial purposes. Any commercial use of Managarr (e.g., selling or offering as a paid service) requires separate permission.

2. **Attribution when Forking and Redistributing Without Contributing back to Main Project**:
   - **If you fork the project and distribute it separately** (e.g., publish or _publicly_ host it independently from the original project), you are required to provide attribution. 
   - You may credit the original author by using any of the following phrasing:
     - "Thanks to Alexander J. Clarke (Dark-Alex-17) for creating the original Managarr project!"
     - "Forked from the Managarr project, created by Alexander J. Clarke (Dark-Alex-17)"
     - "This software is based on the original Managarr project by Alexander J. Clarke (Dark-Alex-17)"
     - "Inspired by Alexander J. Clarke (Dark-Alex-17)'s Managarr project"
   - If changes are made to the base Managarr project, please note those modifications and provide the new attribution accordingly.

## Rust
You'll need to have the stable Rust toolchain installed in order to develop Managarr.

The Rust toolchain (stable) can be installed via rustup using the following command:

```shell
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

This will install `rustup`, `rustc` and `cargo`. For more information, refer to the [official Rust installation documentation](https://www.rust-lang.org/tools/install). 

## Commitizen
[Commitizen](https://github.com/commitizen-tools/commitizen?tab=readme-ov-file) is a nifty tool that helps us write better commit messages. It ensures that our 
commits have a consistent style and makes it easier to generate CHANGELOGS. Additionally, 
Commitizen is used to run pre-commit checks to enforce style constraints. 

To install `commitizen` and the `pre-commit` prerequisite, run the following command:

```shell
python3 -m pip install commitizen pre-commit
```

### Commitizen Quick Guide
To see an example commit to get an idea for the Commitizen style, run:

```shell
cz example
```

To see the allowed types of commits and their descriptions, run:

```shell
cz info
```

If you'd like to create a commit using Commitizen with an interactive prompt to help you get
comfortable with the style, use:

```shell
cz commit
```

## Setup workspace

1. Clone this repo
2. Run `cargo test` to set up hooks
3. Make changes
4. Run the application using `make run` or `cargo run`
5. Commit changes. This will trigger pre-commit hooks that will run format, test and lint. If there are errors or warnings from Clippy, please fix them.
6. Push your code to a new branch named after the feature/bug/etc. you're adding. This will trigger pre-push hooks that will run lint and test.
7. Create a PR

### CI/CD Testing with Act
If you also are planning on testing out your changes before pushing them with [Act](https://github.com/nektos/act), you will need to set up `act`,
`docker`, and configure your local system to run different architectures:

1. Install `docker` by following the instructions on the [official Docker installation page](https://docs.docker.com/get-docker/).
2. Install `act` by following the instructions on the [official Act installation page](https://nektosact.com/installation/index.html).
3. Install `binfmt` on your system once so that `act` can run the correct architecture for the CI/CD workflows. 
   You can do this by running:
   ```shell
   sudo docker run --rm --privileged tonistiigi/binfmt --install all
   ```
   
Then, you can run workflows locally without having to commit and see if the GitHub action passes or fails. 

**For example**: To test the [release.yml](.github/workflows/release.yml) workflow locally, you can run:

```shell
act -W .github/workflows/release.yml --input_type bump=minor
```

## Questions? Reach out to me!
If you encounter any questions while developing Managarr, please don't hesitate to reach out to me at alex.j.tusa@gmail.com. I'm happy to help contributors, new and experienced in any way I can!
