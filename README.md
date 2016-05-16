ThinkBayes-rs
===========

This is a unofficial rewrite of [Allen B. Downey's ThinkBayes](https://github.com/AllenDowney/ThinkBayes) in to [Rust](rust-lang.org).

This project has two goals:

1. To be a clean port of ThinkBayes.py; so that the book can easily be followed in Rust code.
2. To be Rusty, demonstrating and using good Rust practise.

Issues, Pull Requests, comments, spelling corrections, and questions all welcome.

The Progress So Far:
-----

These goals do not always align, but we will jump off that bridge when we come to it. So here goes:
##### Chapter 2  Computational Statistics #####
###### 2.1  Distributions ######
Code from this section ended up as tests in src/pmf.rs
###### 2.2  The cookie problem ######
Code from this section (cookie.py) ended up in tests/cookie.rs
###### 2.3  The Bayesian framework ######
Rust dose not (yet 1.8.0) have class inheritance. Rust has traits or composition for code reuse. tests/cookie2.rs demonstrates using composition to extend Pmf.
