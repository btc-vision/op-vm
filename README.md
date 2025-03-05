# OP_VM

![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)
![Bitcoin](https://img.shields.io/badge/Bitcoin-000?style=for-the-badge&logo=bitcoin&logoColor=white)
![AssemblyScript](https://img.shields.io/badge/assembly%20script-%23000000.svg?style=for-the-badge&logo=assemblyscript&logoColor=white)
![TypeScript](https://img.shields.io/badge/TypeScript-007ACC?style=for-the-badge&logo=typescript&logoColor=white)
![NodeJS](https://img.shields.io/badge/Node%20js-339933?style=for-the-badge&logo=nodedotjs&logoColor=white)
![NPM](https://img.shields.io/badge/npm-CB3837?style=for-the-badge&logo=npm&logoColor=white)
![Gulp](https://img.shields.io/badge/GULP-%23CF4647.svg?style=for-the-badge&logo=gulp&logoColor=white)
![ESLint](https://img.shields.io/badge/ESLint-4B3263?style=for-the-badge&logo=eslint&logoColor=white)

[![code style: prettier](https://img.shields.io/badge/code_style-prettier-ff69b4.svg?style=flat-square)](https://github.com/prettier/prettier)

**Package**: `@btc-vision/op-vm`

## Overview

Welcome to the OP_NET Rust VM, an experimental virtual machine designed for evaluating smart contracts on the Bitcoin
network using Rust and Wasmer. This VM is part of the broader OP_NET initiative, which aims to bring smart contract
functionality to Bitcoin. However, please note that this code is currently in an experimental stage and is subject to
frequent breaking changes. Nothing is final, and you should expect ongoing development and instability.

## Key Features

- **Wasmer Integration**: The VM leverages Wasmer, a leading WebAssembly runtime, to execute smart contracts.
- **N-API Integration**: This project uses N-API to interface the Rust VM with Node.js, enabling seamless interaction
  between JavaScript/TypeScript code and Rust.
- **Multi-platform Support**: The VM is built with support for a wide range of platforms, including various Linux
  distributions, macOS, Windows, and even Android.

## Project Structure

The project is organized into the following directories:

- **`src/`**: Contains the Rust source code for the VM.
    - **`application/`**: (Currently empty, might be reserved for application-level logic in the future)
    - **`domain/`**: Core domain logic of the VM.
        - **`runner/`**: Contains the code that runs and manages contracts, including setting up environments, managing
          instances, and handling custom imports.
        - **`vm/`**: Core virtual machine components, including gas cost management, logging, and other critical VM
          operations.
    - **`interfaces/`**: Manages external function calls and N-API bindings.
        - **`napi/`**: Implements external functions that interface with Node.js.
- **`target/`**: Build artifacts, including the compiled VM binaries, are stored here.

## Getting Started

### Prerequisites

- **Node.js**: Version 16 or higher is required.
- **Rust**: You must have Rust installed to compile and develop this project.
- **wasm-pack**: To work with Wasmer, ensure that `wasm-pack` is installed.
- **@napi-rs/cli**: This is a required dependency for managing N-API bindings.

### Installation

1. **Clone the repository**:

   ```bash
   git clone git://github.com/btc-vision/op-vm.git
   cd op-vm
   ```

2. **Install dependencies**:

   ```bash
   npm install
   ```

3. **Build the project**:

   To build the project for your platform, run:

   ```bash
   npm run build
   ```

   To build in debug mode:

   ```bash
   npm run build:debug
   ```

### Usage

After building the project, you can use the VM to evaluate Bitcoin smart contracts by integrating it into your Node.js
applications. Since this is an experimental module, documentation on specific APIs and usage examples is still under
development. Please refer to the source code for detailed insights on how the VM operates.

### Scripts

- **`npm run build`**: Compiles the Rust code into a platform-specific binary.
- **`npm run build:debug`**: Compiles the Rust code with debug information.
- **`npm run artifacts`**: Generates artifacts for all supported platforms.
- **`npm run universal`**: Builds a universal binary.
- **`npm run version`**: Updates the package version.
- **`npm run postinstall`**: Automatically runs the build script after installation.

## Supported Platforms

This project aims to support a wide range of platforms. The N-API bindings and Wasmer runtime are configured to work on
various architectures, including:

- **Linux**: x86_64, aarch64, armv7
- **macOS**: x86_64, arm64
- **Windows**: x86_64, i686
- **Android**: aarch64, armv7
- **FreeBSD**: x86_64
- **Other**: RISC-V, Musl variants, etc.

## Contributing

Given the experimental nature of this project, contributions are welcome, but please be aware that the codebase is
evolving rapidly. If you encounter any issues or have suggestions for improvement, feel free to open an issue or submit
a pull request. Signed commits are required, and please adhere to the project's code of conduct.

## Warning: Experimental Code

This project is in the early stages of development and is considered experimental. Expect breaking changes, incomplete
features, and potential bugs. Use this code at your own risk, and avoid relying on it for production purposes until it
reaches a more stable state.

## License

This project is licensed under a custom license. For more information, please refer to the [LICENSE.md](LICENSE.md)
file.

## Contact

For more information, visit the [OP_NET homepage](https://opnet.org/) or reach out via the repository's GitHub page.
