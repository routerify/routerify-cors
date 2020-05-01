[![Github Actions Status](https://github.com/routerify/routerify-cors/workflows/Test/badge.svg)](https://github.com/routerify/routerify-cors/actions)
[![crates.io](https://img.shields.io/crates/v/routerify-cors.svg)](https://crates.io/crates/routerify-cors)
[![Documentation](https://docs.rs/routerify-cors/badge.svg)](https://docs.rs/routerify-cors)
[![MIT](https://img.shields.io/crates/l/routerify-cors.svg)](./LICENSE)

# routerify-cors

A [`Routerify`](https://github.com/routerify/routerify) middleware which enables [`CORS`](https://en.wikipedia.org/wiki/Cross-origin_resource_sharing).

[Docs](https://docs.rs/routerify-cors)

## Usage

First add this to your `Cargo.toml`:

```toml
[dependencies]
routerify-cors = "1.0"
```

An example:
```rust
use routerify_cors;

fn main() {
  println!("{}", routerify_cors::add(2, 3));
}
```

## Contributing

Your PRs and suggestions are always welcome.