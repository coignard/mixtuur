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

Run the following Cargo command in your project directory:

```bash
cargo add mixtuur
```

Or add the following line to your `Cargo.toml`:

```toml
mixtuur = "0.1.1"
```

## Usage

```bash
mixtuur <note> <scale>
```

## License

© 2026 René Coignard.

All code is licensed under the GPL, v3 or later. See [LICENSE](LICENSE) file for details.
