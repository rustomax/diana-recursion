# diana-recursion
Rather contrived example of needless recursion in Rust

[![Build Status](https://travis-ci.org/rustomax/diana-recursion.svg?branch=master)](https://travis-ci.org/rustomax/diana-recursion)

## Usage

```sh
git clone https://github.com/rustomax/diana-recursion.git
cd diana-recursion
cargo run

// Output:
// 1 1 1 0 0 0 0 0 0
// 1 1 1 0 0 0 0 0 0
// 1 1 1 0 0 0 0 0 0
// 0 0 0 1 1 0 0 0 0
// 0 0 0 1 1 0 0 0 0
// 0 0 0 0 0 1 1 1 1
// 0 0 0 0 0 1 1 1 1
// 0 0 0 0 0 1 1 1 1
// 0 0 0 0 0 1 1 1 1
```
