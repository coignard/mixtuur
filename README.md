# mixtuur

A harmonic pitch colour generator for Cubase.

## Install

```bash
cargo install --locked --git https://github.com/coignard/mixtuur
```

Or build manually:

```bash
git clone https://github.com/coignard/mixtuur
cd mixtuur
cargo build --release
sudo cp target/release/mixtuur /usr/local/bin/
```

## Install as library

Add to your `Cargo.toml`:

```toml
mixtuur = "0.1.0"
```

## Usage

```bash
mixtuur <note> <scale>
```

## License

© 2026 René Coignard.

All code is licensed under the GPL, v3 or later. See [LICENSE](LICENSE) file for details.
