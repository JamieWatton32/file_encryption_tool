# Rust Development Environment Setup

This document details the essential procedures for installing the **Rust programming language** and initializing a project using its official toolchain. Deeper language specifics are delegated to *The Rust Programming Language Book (The Book)*.

---

## 1. Installation via `rustup`

Rust is installed and managed through the official toolchain installer, **`rustup`**. This utility manages the Rust compiler (`rustc`), the build system/package manager (`cargo`), and the standard documentation.

### 1.1. Execution

Execute the appropriate command for your operating system:

* **Linux/macOS:**
    ```bash
    curl --proto '=https' --tlsv1.2 -sSf [https://sh.rustup.rs](https://sh.rustup.rs) | sh
    ```
* **Windows:**
    Download and execute the installer from the official [Rust website](https://www.rust-lang.org/tools/install).

### 1.2. Environment Configuration

Upon installation, the installer attempts to configure the shell to include the **`$HOME/.cargo/bin`** directory in the system's `$PATH` variable.

If manual configuration is required, execute the following command in the current terminal session or add it to the shell's configuration file (e.g., `.bashrc`, `.zshrc`):

```bash
source $HOME/.cargo/env
```

1.3. Verification

Confirm the successful installation by checking the installed versions of the core components:

```bash
rustc --version
cargo --version
```

## 2. Project Initialization with cargo

Cargo is the package manager and build tool for Rust projects. It manages dependencies, compilation, and testing.

### 2.1. Project Creation

Use the cargo new command to initialize a new project directory and structure:

```bash
cargo new project_name
cd project_name
```

This command generates the following required structure:

- The project root directory (project_name).

- A src directory containing the primary source file (main.rs).

- The project manifest file, Cargo.toml.

### 2.2. Project Manifest (Cargo.toml)

This manifest file is used to define project metadata and manage external dependencies (crates).

    [package]: Defines the project name, version, and author.

    [dependencies]: Lists required third-party libraries.

### 2.3. Build and Execution

To compile and execute the project's binary:

Compile and Run:

```bash
cargo run
```

Compile Only: The resulting binary is placed in the target/debug directory.

```bash
cargo build
```

## 3. Reference and Further Documentation

The primary resource for learning the Rust language is the official documentation:

    The Rust Programming Language Book: https://doc.rust-lang.org/book/

    Rust Documentation: https://doc.rust-lang.org/

### 3.1. Recommended Tools

For improved development workflow:

    Language Server: Install the Rust Analyzer extension for your code editor (e.g., VS Code, Vim, RustRover) to enable advanced features like type inference, autocompletion, and diagnostics.

    Linter: rust-clippy (generally included with rustup) provides a collection of lints to adhere to better Rust coding practices.