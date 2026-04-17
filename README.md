# Rust Port of caarlos0/env

This repository was created to port the popular Golang project **[caarlos0/env](https://github.com/caarlos0/env)** into Rust.

The main goal of this project is to use it as a **learning opportunity** to improve my understanding of Rust, its ecosystem, ownership model, traits, error handling, and idiomatic patterns.

## Original Project
- **Go Repository**: [caarlos0/env](https://github.com/caarlos0/env)
- A simple, zero-dependency library for loading environment variables into structs.

## Project Goals
- Learn Rust by re-implementing a real-world, useful library
- Understand how to translate Go concepts (structs, tags, reflection) into idiomatic Rust
- Explore Rust's type system, derive macros, and error handling
- Compare developer experience and performance between Go and Rust

## Current Status
- In active development / learning phase
- Rust library (`--lib`) project

## Getting Started

### Prerequisites
- Rust toolchain (install via [rustup.rs](https://rustup.rs/))

### Build
```bash
cargo build