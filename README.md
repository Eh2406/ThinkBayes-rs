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
###### 2.4  The Monty Hall problem ######
test/monty.rs also uses composition.
###### 2.5  Encapsulating the framework ######
The "Template method pattern" and "abstract type" are perfect for trait inheritance.
So we have a trait in std/suite.rs that need:
- fn likelihood(&self, data: &D, hypo: &V) -> f64;
- fn get_mut_pmf(&mut self) -> &mut Pmf<V>;

And uses them to provide default implementation of:
- fn update(&mut self, data: &D);

Code from this section (monty2.py) ended up in tests/monty2.rs

###### 2.6  The M&M problem ######
Code from this section (m_and_m.py) ended up in tests/m_and_m.rs

##### Chapter 3  Estimation #####
###### 3.1  The dice problem ######
Code from this section (dice.py) ended up in tests/dice.rs
###### 3.2  The locomotive problem and 3.3  What about that prior? ######
Code from this section (train.py) ended up in tests/train.rs
###### 3.4  An alternative prior and 3.5  Credible intervals and 3.6  Cumulative distribution functions ######
Code from this section tests/train2.rs. This is the code from the book, not from train3.py.
