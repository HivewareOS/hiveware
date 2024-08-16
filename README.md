![Hive Banner](./assets/banner.png)

# Hiveware CLI Documentation

**Hiveware** is a command-line tool designed for managing Nix packages easily. It provides commands to install, uninstall, and interact with packages in a Nix shell. This document outlines the installation process and usage details for Hiveware.

## Installation Instructions

To install Hiveware, you need to have Rust and Cargo installed on your system, as Hiveware is built using Rust. Follow these steps to get Hiveware up and running:

### Prerequisites

1. **Install Rust**: If you haven't already installed Rust, you can do so using `rustup`. Open your terminal and run:

   ```sh
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

<br>

Follow the on-screen instructions to complete the installation. You might need to restart your terminal or run `source $HOME/.cargo/env` to start using Rust immediately.

2. **Verify Installation**: Ensure Rust and Cargo are installed correctly by running:

   ```sh
   rustc --version
   cargo --version
   ```

### Quickstart

Install `hiveware` via crates.io:

```sh
cargo install hiveware
```

```sh
hiveware install alacritty
```

### Clone and Build Hive

1. **Clone the Repository**: Clone the Hiveware repository from GitHub:

   ```sh
   git clone https://github.com/HivewareOS/hiveware.git
   ```

<br>

2. **Navigate to the Project Directory**: Move into the cloned repository directory:

   ```sh
   cd hiveware
   ```

<br>

3. **Build the Project**: Compile the Hiveware tool using Cargo:

   ```sh
   cargo build --release
   ```

   The compiled binary will be located in the `target/release` directory.

<br>

4. **Install the Binary**: Optionally, you can copy the binary to a directory in your system's PATH for easy execution. For example:

   ```sh
   cp target/release/hiveware /usr/local/bin/
   ```

   Ensure `/usr/local/bin` is in your PATH. You might need `sudo` permissions to copy the binary.

<br>

5. To add the Cargo bin directory to your PATH, append the following line to your .bashrc, .zshrc, or equivalent shell configuration file:

```bash
export PATH="$HOME/.cargo/bin:$PATH"
```

After updating the file, reload your shell configuration with:

```bash
source ~/.bashrc
```

or

```bash
source ~/.zshrc
```

## Usage

Once Hiveware is installed, you can use it to manage Nix packages. Here’s a quick guide to the available commands:

### Commands Overview

#### `install`

Installs a package using `nix-env`.

**Usage:**

```sh
hiveware install <package>
```

- **`<package>`**: The name of the package to install.

**Example:**

```sh
hiveware install rust-analyzer
```

#### `uninstall`

Uninstalls a package using `nix-env`.

**Usage:**

```sh
hiveware uninstall <package>
```

- **`<package>`**: The name of the package to uninstall.

**Example:**

```sh
hiveware uninstall wezterm
```

#### `virtual`

Enters a Nix shell with a specified package installed.

**Usage:**

```sh
hiveware virtual <package>
```

- **`<package>`**: The name of the package to install in the Nix shell.

**Example:**

```sh
hiveware virtual alacritty
```

#### `version`

Displays the current version of Hiveware.

**Usage:**

```sh
hiveware version
```

**Example:**

```sh
hiveware version
```

## Error Handling

If an invalid command or option is provided, Hiveware will display an error message and exit with status code `1`.

## Contact and Support

For support or inquiries, please contact HivewareOS at [hivewareos@protonmail.com](mailto:hivewareos@protonmail.com).

## Additional Information

For more detailed information on usage and options, use:

```sh
hiveware --help
```

---

This documentation aims to provide a comprehensive overview of installing and using Hiveware. If you encounter any issues or have questions, don't hesitate to reach out for support.
