# Rust Experiment: Derive Procedural Macro #

This repository contains some simple experimentation with Rust trait derivation
using procedural macros.

Procedural macros are a fast-changing area in Rust.
However, as of Nov 2018, this represents a current standard approach for implementing
procedural macros.

## Resources ##

Further guidance and examples can be found in the following:

* [Syn crate examples](https://github.com/dtolnay/syn/tree/master/examples)
* [serde-derive crate](https://github.com/serde-rs/serde/tree/master/serde_derive)
* [derive-new crate](https://github.com/nrc/derive-new)
* [failure-derive crate](https://github.com/withoutboats/failure_derive)

## TODO ##

* More complex derives
* Try derive on a generic struct.
* Testing
  * Tests for `example_lib`
  * Compilation tests for `example_derive`
  * Expansion tests for `example_derive`
