
# Automated Compliance Checker for Code

## Overview
This project is designed to automatically check codebases for compliance with standards like GDPR and HIPAA in real-time. It includes a Rust-based core analysis engine and a Node.js integration for CI/CD workflows.

## Features
- Rust engine for efficient compliance rule checks.
- Node.js API for CI/CD integration.
- CLI support for local usage.

## Installation
### Rust Engine
```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Clone the repo and build
cd automated-compliance-checker
cargo build --release
```

### Node.js API
```bash
# Install Node.js dependencies
cd node-integration
npm install

# Start the server
node server.js
```

## Usage
### CLI
```bash
cargo run -- <directory_path>
```

### API
```bash
POST /check-compliance
Content-Type: application/json
{
    "directory": "/path/to/codebase"
}
```

## Contributing
1. Fork the repo.
2. Create a feature branch.
3. Submit a pull request with detailed notes.

## License
This project is licensed under the MIT License.
