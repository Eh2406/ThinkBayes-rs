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
struct Train {
    pmf: Pmf<u32>,
}

/// Represents hypotheses about how many trains the company has.
/// The likelihood function for the train problem is the same as
/// for the Dice problem.
impl Train {
    pub fn new<I: Iterator<Item = u32>>(hypos: I) -> Train {
        let mut out = Train { pmf: Pmf::new() };
        for v in hypos {
            out.pmf.set(v, 1.0);
        }

        out.pmf.normalize(1.0);
        out
    }
    fn get_pdf(&self) -> &Pmf<u32> {
        &self.pmf
    }
}

impl Suite<u32, u32> for Train {
    fn get_mut_pmf(&mut self) -> &mut Pmf<u32> {
        &mut self.pmf
    }
    fn likelihood(&self, data: &u32, hypo: &u32) -> f64 {
        if hypo < data {
            0.0
        } else {
            1.0 / (*hypo as f64)
        }
    }
}

#[test]
fn suite_train() {
    let mut suite = Train::new(1..1001);
    suite.update(&60);
    assert_ulps_eq!{suite.get_pdf().prob(&59, 0.0), 0.0, max_ulps = 4}
    assert_ulps_eq!{suite.get_pdf().prob(&60, 0.0), 0.005905417875729859, max_ulps = 4}
    assert_ulps_eq!{suite.get_pdf().prob(&61, 0.0), 0.005808607746619534, max_ulps = 4}
    assert_ulps_eq!{suite.get_pdf().prob(&100, 0.0), 0.0035432507254379154, max_ulps = 4}
    assert_ulps_eq!{suite.get_pdf().prob(&500, 0.0), 0.000708650145087583, max_ulps = 4}
    assert_ulps_eq!{suite.get_pdf().prob(&1000, 0.0), 0.0003543250725437915, max_ulps = 4}
    assert_ulps_eq!{suite.get_pdf().mean(), 333.41989326371095, max_ulps = 4}
    suite.update(&30);
    suite.update(&90);
    assert_ulps_eq!{suite.get_pdf().mean(), 164.3055864227335, max_ulps = 4}

    let mut suite = Train::new(1..501);
    suite.update(&60);
    assert_ulps_eq!{suite.get_pdf().mean(), 207.07922798340903, max_ulps = 4}
    suite.update(&30);
    suite.update(&90);
    assert_ulps_eq!{suite.get_pdf().mean(), 151.84958795903847, max_ulps = 4}

    let mut suite = Train::new(1..2001);
    suite.update(&60);
    assert_ulps_eq!{suite.get_pdf().mean(), 552.179017164631, max_ulps = 4}
    suite.update(&30);
    suite.update(&90);
    assert_ulps_eq!{suite.get_pdf().mean(), 171.33818109150948, max_ulps = 4}
}
