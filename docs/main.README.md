# Repo Summarizer

A command-line tool to generate "summaries" of GitHub repositories.

## For a repository like `paradigmxyz/solar`, run

```bash
cargo run --release -- paradigmxyz/solar
```

## Or for local testing, you might want to try a smaller repo

```bash
cargo run --release -- ursisterbtw/pyrs-template
```

## Verbose output

```rust
cargo run --release -- paradigmxyz/solar --token <GITHUB_TOKEN>
```

## For help

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

See [CONTRIBUTING.md](CONTRIBUTING.md) for details.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
