# argsparse

[![Crates.io](https://img.shields.io/crates/v/argsparse.svg)](https://crates.io/crates/argsparse)
[![Docs.rs](https://docs.rs/argsparse/badge.svg)](https://docs.rs/argsparse)

A minimal and flexible argument parser for command-line applications, written in Rust.

---

## 📦 Features

- Simple parsing of flags, options, and positional arguments
- Convenient trait-based API for extracting values
- Supports `--`, short `-f` and long `--flag` forms
- Works with slices of `&str` (no global state)

---

## 🚀 Getting Started

Add to your `Cargo.toml`:

```toml
[dependencies]
argsparse = "0.1"
