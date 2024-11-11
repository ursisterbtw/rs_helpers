# rs_helpers

A random assortment of Rust helper scripts.

<img src="docs/nice_kreb.png" width="320" />

## Work in Progress

### `src` currently contains a GitHub repository analyzer that can be used to get a quick "summary" of a repository.

This Rust application fetches and analyzes data from a specified GitHub repository. It provides a summary of the repository's information, statistics, languages used, and key files.

## Features

- Fetches repository information such as name, description, URL, stars, forks, and more.
- Retrieves repository statistics including open issues, watchers, network count, and size.
- Analyzes the languages used in the repository.
- Fetches content of key files like `README.md`, `LICENSE`, `Cargo.toml`, etc.
- Supports output in JSON or YAML format.

## Usage

### For a repository like `paradigmxyz/solar`, run

```bash
cargo run --release -- paradigmxyz/solar
```

### Or for local testing, you might want to try a smaller repo

```bash
cargo run --release -- ursisterbtw/pyrs-template
```

### Verbose output

```rust
cargo run --release -- paradigmxyz/solar --token <GITHUB_TOKEN>
```

### For help

```bash
cargo run --release -- --help
```

## Directory structure

```rust
repo-summarizer/
├── .github/
│   └── workflows/
│       ├── ci.yml
│       └── release.yml
├── src/
│   └── main.rs
└── Cargo.toml
```

## TODO

- [ ] Add more tests
- [ ] Add more documentation

## Contributing

<!-- See [CONTRIBUTING.md](CONTRIBUTING.md) for details. -->

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
