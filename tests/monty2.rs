// This file contains code for use with "Think Bayes",
// by Allen B. Downey, available from greenteapress.com
//
// Copyright 2012 Allen B. Downey
//
// Rewrite 2016 by Jacob Finkelman
// License: GNU GPLv3 http://www.gnu.org/licenses/gpl.html

extern crate think_bayes;
use think_bayes::pmf::*;
use think_bayes::suite::*;
#[macro_use]
extern crate approx;

// This file uses composition to emulate classes
struct Monty {
    pmf: Pmf<char>,
}

impl Monty {
    pub fn new<I: Iterator<Item = char>>(hypos: I) -> Monty {
        let mut out = Monty { pmf: Pmf::new() };
        for v in hypos {
            out.pmf.set(v, 1.0);
        }

        out.pmf.normalize(1.0);
        out
    }
    fn get_pdf(&self) -> &Pmf<char> {
        &self.pmf
    }
}

impl Suite<char, char> for Monty {
    fn get_mut_pmf(&mut self) -> &mut Pmf<char> {
        &mut self.pmf
    }
    fn likelihood(&self, data: &char, hypo: &char) -> f64 {
        if data == hypo {
            0.0
        } else if *hypo == 'A' {
            0.5
        } else {
            1.0
        }
    }
}

#[test]
fn suite_monty_composition() {
    let mut pmf = Monty::new("ABC".chars());
    assert_ulps_eq!{pmf.get_pdf().prob(&'A', 0.0), 1.0 / 3.0, max_ulps = 4}
    assert_ulps_eq!{pmf.get_pdf().prob(&'B', 0.0), 1.0 / 3.0, max_ulps = 4}
    assert_ulps_eq!{pmf.get_pdf().prob(&'C', 0.0), 1.0 / 3.0, max_ulps = 4}

    pmf.update(&'B');
    assert_ulps_eq!{pmf.get_pdf().prob(&'A', 0.0), 1.0 / 3.0, max_ulps = 4}
    assert_ulps_eq!{pmf.get_pdf().prob(&'B', 0.0), 0.0, max_ulps = 4}
    assert_ulps_eq!{pmf.get_pdf().prob(&'C', 0.0), 2.0 / 3.0, max_ulps = 4}
}
