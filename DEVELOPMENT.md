# Development

## Run

```bash
cargo run
```

## Build

```bash
cargo build
```

## Test

```bash
cargo test
```

with coverage

```bash
cargo llvm-cov
```

## Lint

```bash
cargo clippy --fix --allow-dirty
```

## Format

```bash
cargo fmt
```

## Build Production

```bash
cargo build --release
```

## Run Production

```bash
cargo run -- --target ./src
```

## Install globally

```bash
cargo install --path .
```

execute `nuxt-auto-import-replacer` in your terminal.

```bash
nuxt-auto-import-replacer --target ./src
```
