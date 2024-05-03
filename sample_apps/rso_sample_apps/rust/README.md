# RUST RSO Sample app

#### Description

### Requirements
1. Make sure you have your `config.yml` in the root of the project [(_example_)](config/config.yml): `sample_apps/rso_sample_apps/rust/config.yml`.
1. You need rust 1.77.2 installed on your machine. You can install it from [here](https://www.rust-lang.org/tools/install).

### Makefile Documentation

This Makefile contains several commands that help with building, testing, cleaning, and running the Rust project.

#### Commands

- `make all`: This command first cleans up any previous build artifacts, then builds the project and runs the tests. It's a quick way to ensure everything is up to date and working correctly.

- `make build`: This command builds the Rust project. It compiles the source code into an executable file.

- `make test`: This command runs the tests for the Rust project. It ensures that all the functions in the project are working as expected.

- `make clean`: This command cleans up any build artifacts from previous builds. It's a good practice to clean up before starting a new build.

- `make run`: This command runs the Rust project. It starts the application.

To use these commands, open a terminal in the project's root directory and type the command you want to use. For example, to build the project, type `make build`.