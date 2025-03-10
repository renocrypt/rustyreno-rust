# Setting Up Rust Development Environment -- GitHub Codespaces

This guide walks you through installing Rust and Cargo, configuring Visual Studio Code, and setting up GitHub Codespaces for Rust development.

## Installing Rust and Cargo

The recommended way to install Rust is through `rustup`, the Rust toolchain installer. It installs Rust, Cargo (Rust's package manager), and other essential tools.

### On Linux and macOS

Run the following command in your terminal:

```bash
curl https://sh.rustup.rs -sSf | sh
```

This downloads a script and starts the installation. If successful, you'll see:

```
Rust is installed now. Great!
```

Remember to restart your shell or run the command shown in the installation output to update your PATH variables.

### On Windows

1. Download the rustup-init.exe file from [rustup.rs](https://rustup.rs)
2. Run the installer
3. Follow the on-screen instructions

After installation, the console will display a success message.

### Verifying Installation

To verify that Rust and Cargo are properly installed:

```bash
rustc --version
cargo --version
```

You should see version information for both tools.

## Setting Up Visual Studio Code for Rust Development

Visual Studio Code provides excellent support for Rust development.

### Essential VS Code Extensions

Install these extensions for an optimal Rust development experience:

1. **Rust Analyzer** - Provides intelligent code completion, error checking, and more
2. **Even Better TOML** - For editing Cargo.toml files
3. **CodeLLDB** - For debugging Rust applications[4][6]

## Setting Up GitHub Codespaces for Rust

GitHub Codespaces provides a cloud-based development environment that you can configure specifically for Rust.

### Creating a Codespace for Your Repository

1. Navigate to your GitHub repository
2. Click the "Code" button 
3. Select the "Codespaces" tab
4. Click "Create codespace on main"[8]

### Configuring Codespaces for Rust

To ensure your Codespace has Rust installed and configured correctly, create a `.devcontainer` directory in your repository with these files:

#### devcontainer.json

```json
{
    "name": "Rust Development",
    "image": "mcr.microsoft.com/devcontainers/rust:latest",
    "customizations": {
        "vscode": {
            "extensions": [
                "rust-lang.rust-analyzer",
                "tamasfe.even-better-toml",
                "vadimcn.vscode-lldb"
            ]
        }
    },
    "postCreateCommand": "rustup component add clippy rustfmt"
}
```

This configuration:
- Uses Microsoft's official Rust development container image
- Installs essential VS Code extensions for Rust development
- Adds the clippy linter and rustfmt formatter[4][6]

## Creating a Rust Project

Once your environment is set up, you can create your first Rust project:

```bash
# Create a new binary application
cargo new hello_rust
cd hello_rust

# Build and run your project
cargo build
cargo run
```

This will:
1. Create a new Rust project called "hello_rust"
2. Change to that directory
3. Compile and run the default "Hello, world!" program[9]

## Project Structure

A typical Rust project includes:

- `Cargo.toml`: The manifest file containing metadata and dependencies
- `src/main.rs`: The entry point for binary applications
- `src/lib.rs`: The entry point for library crates[9][19]
