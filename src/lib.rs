// This file contains code for use with "Think Stats" and
// "Think Bayes", both by Allen B. Downey, available from greenteapress.com
// Copyright 2014 Allen B. Downey
//
// Rewrite 2016 by Jacob Finkelman
// License: GNU GPLv3 http://www.gnu.org/licenses/gpl.html

// thinkbayes.py has all the code in one big file. So it cane be shared and imported in python.
// Cargo means that it is straightforward to share split up, so I will do that.
extern crate fnv;

#[cfg(test)]
#[macro_use]
extern crate approx;
#[cfg(test)]
extern crate quickcheck;

pub mod utils;
pub use utils::*;

pub mod pmf;
pub use pmf::*;

pub mod suite;
pub use suite::*;

pub mod cdf;
pub use cdf::*;
