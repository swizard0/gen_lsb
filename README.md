# gen_lsb

## Summary

`gen_lsb` stands for "Genetic Algorithm like a serious business". It is a high-performance generic framework with various evolution strategies support.

Being written in [Rust](https://www.rust-lang.org/) `gen_lsb` framework uses the best language features, and it's developing is focused primarily on following goals:

* Performance:
 * Static polymorphism and zero-cost abstractions wherever possible.
 * Minimum memory allocations and/or reallocations.
 * Active usage of parallelism using map-reduce style jobs and thread pool.
* Maximum abstraction:
 * Flexible architecture based on [IoC](https://en.wikipedia.org/wiki/Inversion_of_control) design using [Policy Pattern](https://en.wikipedia.org/wiki/Strategy_pattern).
 * Convinient user-defined error types support with accurate processing.
 * Large collection of evolution strategies with relatively small codebase.
* Higher reliability:
 * Very accurate errors handling with almost no panics, only using return values and static typing.
 * No [unsafe](https://doc.rust-lang.org/book/unsafe.html) code at all.
 * Maximum test coverage.

## Current status

Proof-of-concept (in active development). Not recommended for use.

## Using gen_lsb

Add to `Cargo.toml`:

```toml
[dependencies]
gen_lsb = "0"
```

To `src/main.rs`:

```rust
extern crate gen_lsb;

// TODO: add a small and cute example
```
