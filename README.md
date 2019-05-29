# rust-comp

This project is a compiler for the Rust programming language.

## Goals

* Correctness -- It should never crash and should always output machine code
  performing the same task as the source code.
* Compilation speed -- It must be fast at compiling Rust source code.
* Embeddable -- It should be easy to use the compiler as a library to do static
  analysis and make related tooling.
