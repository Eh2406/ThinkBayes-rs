// This file contains code for use with "Think Stats" and
// "Think Bayes", both by Allen B. Downey, available from greenteapress.com
// Copyright 2014 Allen B. Downey
//
// Rewrite 2016 by Jacob Finkelman
// License: GNU GPLv3 http://www.gnu.org/licenses/gpl.html

// thinkbayes.py has all the code in one big file. So it cane be shared and imported in python.
// Cargo means that it is straightforward to share split up, so I will do that.

#[cfg(test)]
#[macro_use]
extern crate approx;

mod utils;
pub use utils::*;
