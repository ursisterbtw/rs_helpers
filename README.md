# rs_helpers

A random assortment of Rust helper scripts.

<img src="docs/nice_kreb.png" width="320" />

## * Work in Progress *

feel free to check out <https://github.com/ursisterbtw/hash_hunter>, an Ethereum address generator that I wrote, while you wait for me to add more scripts :)

## Available Tools

### 1. GitHub Repository Analyzer
Located in `src/`, this tool provides a quick "summary" of a repository.

- Fetches repository information such as name, description, URL, stars, forks, and more
- Retrieves repository statistics including open issues, watchers, network count, and size
- Analyzes the languages used in the repository
- Fetches content of key files like `README.md`, `LICENSE`, `Cargo.toml`, etc
- Supports output in JSON or YAML format

### 2. EVM RPC Health Checker
Located in `crates/evm_rpc_health_checker/`, this tool monitors the health of Ethereum RPC endpoints.

- Continuously checks RPC endpoint availability
- Reports current block number when healthy
- Automatic retry on failures
- Easy to configure for different RPC endpoints

### 3. Solana RPC Health Checker
Located in `crates/sol_rpc_health_checker/`, this tool monitors the health of Solana RPC endpoints.

- Continuously checks RPC endpoint availability
- Reports current Solana version when healthy
- Automatic retry on failures
- Configurable commitment level
- Easy to configure for different RPC endpoints


## Usage

### GitHub Repository Analyzer

```bash
# For a repository like paradigmxyz/solar
cargo run --release -- paradigmxyz/solar

# For local testing with a smaller repo
cargo run --release -- ursisterbtw/pyrs-template

# Verbose output
cargo run --release -- paradigmxyz/solar --token <GITHUB_TOKEN>

# For help
cargo run --release -- --help
```

### EVM RPC Health Checker

```bash
# Navigate to the health checker directory
cd crates/evm_rpc_health_checker

# Run the health checker
cargo run --release
```

### Solana RPC Health Checker

```bash
# Navigate to the health checker directory
cd crates/sol_rpc_health_checker

# Run the health checker
cargo run --release
```

## Directory structure

```
rs_helpers/
├── .github/
│ └── workflows/
│ ├── ci.yml
│ └── release.yml
├── src/
│ └── main.rs
├── crates/
│ ├── evm_rpc_health_checker/
│ │ └── src/
│ │ └── main.rs
│ └── sol_rpc_health_checker/
│ └── src/
│ └── main.rs
└── Cargo.toml
```

## TODO

- [ ] Add more tests
- [ ] Add more documentation
- [ ] Add configuration file support for RPC health checkers
- [ ] Add support for multiple RPC endpoints in health checkers
- [ ] Add customizable polling intervals for health checkers

## Contributing

We welcome contributions to the `rs_helpers` repository! Here are some ways you can get involved:

1. **Report Issues**: If you encounter any bugs or would like to request a new feature, please open an issue on GitHub.
2. **Submit Pull Requests**: If you'd like to contribute code, please fork the repository and submit a pull request. Make sure to follow the coding standards and include tests for any new features or bug fixes.
3. **Improve Documentation**: Help us improve the documentation by editing the README or writing more comprehensive guides.
4. **Review Code**: Participate in code reviews to ensure high-quality code is merged into the main branch.
5. **Suggest Enhancements**: Share your ideas for improving existing tools or adding new ones.

### Getting Started

1. **Fork the repository** to your own GitHub account.
2. **Clone the repository** to your local machine:
   ```bash
   git clone https://github.com/ursisterbtw/rs_helpers.git
   ```
   
## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Acknowledgments

- [reqwest](https://github.com/seanmonstar/reqwest) for HTTP requests
- [clap](https://github.com/clap-rs/clap) for command-line argument parsing
- [indicatif](https://github.com/mitsuhiko/indicatif) for progress bars
- [serde](https://github.com/serde-rs/serde) for serialization/deserialization
- [ethers-rs](https://github.com/gakonst/ethers-rs) for Ethereum RPC interactions
- [solana-sdk](https://github.com/solana-labs/solana) for Solana RPC interactions
