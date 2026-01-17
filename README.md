# OP_VM

![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)
![Bitcoin](https://img.shields.io/badge/Bitcoin-000?style=for-the-badge&logo=bitcoin&logoColor=white)
![AssemblyScript](https://img.shields.io/badge/assembly%20script-%23000000.svg?style=for-the-badge&logo=assemblyscript&logoColor=white)
![TypeScript](https://img.shields.io/badge/TypeScript-007ACC?style=for-the-badge&logo=typescript&logoColor=white)
![NodeJS](https://img.shields.io/badge/Node%20js-339933?style=for-the-badge&logo=nodedotjs&logoColor=white)
![NPM](https://img.shields.io/badge/npm-CB3837?style=for-the-badge&logo=npm&logoColor=white)
![ESLint](https://img.shields.io/badge/ESLint-4B3263?style=for-the-badge&logo=eslint&logoColor=white)

[![code style: prettier](https://img.shields.io/badge/code_style-prettier-ff69b4.svg?style=flat-square)](https://github.com/prettier/prettier)

**Package**: `@btc-vision/op-vm`

## Overview

OP_NET Rust VM, a production-ready virtual machine designed for evaluating smart contracts on the Bitcoin
network using Rust and Wasmer. This VM is part of the broader OP_NET initiative, which brings smart contract
functionality to Bitcoin.

## Security Audit

<p align="center">
  <a href="https://verichains.io/">
    <picture>
      <source media="(prefers-color-scheme: dark)" srcset="./AUDIT/verichains-logo.svg">
      <source media="(prefers-color-scheme: light)" srcset="./AUDIT/verichains-logo-dark.svg">
      <img alt="Verichains" src="./AUDIT/verichains-logo.svg" height="32">
    </picture>
  </a>
</p>

OP-VM has been professionally audited by **[Verichains](https://verichains.io/)**, a leading blockchain security firm.
The audit confirms that the codebase is secure and ready for mainnet deployment.

For audit reports and details, see the [AUDIT](./AUDIT) directory.

## Getting Started

### Prerequisites

- **Node.js**: Version 24/25+ or higher is required.
- **Rust**: You must have Rust installed to compile and develop this project.

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
   npm run debug
   ```

### Usage

Please refer to the source code for detailed insights on how the VM operates.
See [the unit test framework](https://github.com/btc-vision/unit-test-framework).

### Scripts

- **`npm run build`**: Compiles the Rust code into a release binary.
- **`npm run debug`**: Compiles the Rust code with debug information.
- **`npm run cross`**: Cross-compiles for different platforms.
- **`npm test`**: Runs the Rust test suite.
- **`npm run coverage`**: Generates code coverage report.
- **`npm run postinstall`**: Automatically runs after installation to set up the binary.

## Contributing

Contributions are welcome! If you encounter any issues or have suggestions for improvement, feel free to open an issue
or submit
a pull request. Signed commits are required, and please adhere to the project's code of conduct.

## License

This project is licensed under a custom license. For more information, please refer to the [LICENSE.md](LICENSE.md)
file.

## Contact

For more information, visit the [OP_NET homepage](https://opnet.org/) or reach out via the repository's GitHub page.
