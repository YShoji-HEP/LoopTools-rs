# Rust Wrapper for LoopTools

This project provides a Rust wrapper for LoopTools-2.16, a library designed for the calculation of Passino-Veltman integrals. The wrapper allows users to leverage the powerful capabilities of LoopTools within Rust applications, facilitating efficient and accurate computations in high-energy physics.

## Features

- Seamless integration with LoopTools for integral calculations.
- Easy-to-use API for Rust developers.

## Usage

Add the following dependency to your `Cargo.toml`:

```toml
[dependencies]
looptools-rs = { git = "https://github.com/YShoji-HEP/LoopTools-rs.git" }
```
The following example demonstrates basic usage of the LoopTools wrapper:
```rust
fn main() {
    looptools_rs::ltini();
    let result = looptools_rs::bget(1000., 50., 80.);
    println!("{:?}", result);
    looptools_rs::ltexi();
}
```

## References

LoopTools was developed by G. J. van Oldenborgh (NIKHEF-H, Amsterdam).

For the algorithms used, G. J. van Oldenborgh and J. A. M. Vermaseren, "New Algorithms for One-loop Integrals," *Zeitschrift für Physik* C46 (1990) 425. NIKHEF-H 89/17.
