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
struct Dice {
    pmf: Pmf<u32>,
}

impl Dice {
    pub fn new<I: Iterator<Item = u32>>(hypos: I) -> Dice {
        let mut out = Dice { pmf: Pmf::new() };
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

impl Suite<u32, u32> for Dice {
    fn get_mut_pmf(&mut self) -> &mut Pmf<u32> {
        &mut self.pmf
    }
    /// Computes the likelihood of the data under the hypothesis.
    /// hypo: integer number of sides on the die
    /// data: integer die roll
    fn likelihood(&self, data: &u32, hypo: &u32) -> f64 {
        if hypo < data {
            0.0
        } else {
            1.0 / (*hypo as f64)
        }
    }
}

#[test]
fn suite_dice() {
    let mut suite = Dice::new([4u32, 6, 8, 12, 20].iter().cloned());
    suite.update(&6);
    assert_ulps_eq!{suite.get_pdf().prob(&4, 0.0), 0.0, max_ulps = 4}
    assert_ulps_eq!{suite.get_pdf().prob(&6, 0.0), 0.3921568627450981, max_ulps = 4}
    assert_ulps_eq!{suite.get_pdf().prob(&8, 0.0), 0.29411764705882354, max_ulps = 4}
    assert_ulps_eq!{suite.get_pdf().prob(&12, 0.0), 0.19607843137254904, max_ulps = 4}
    assert_ulps_eq!{suite.get_pdf().prob(&20, 0.0), 0.11764705882352944, max_ulps = 4}

    for roll in [6, 8, 7, 7, 5, 4].into_iter() {
        suite.update(&roll);
    }

    assert_ulps_eq!{suite.get_pdf().prob(&4, 0.0), 0.0, max_ulps = 4}
    assert_ulps_eq!{suite.get_pdf().prob(&6, 0.0), 0.0, max_ulps = 4}
    assert_ulps_eq!{suite.get_pdf().prob(&8, 0.0), 0.9432484536722124, max_ulps = 4}
    assert_ulps_eq!{suite.get_pdf().prob(&12, 0.0), 0.0552061280612909, max_ulps = 4}
    assert_ulps_eq!{suite.get_pdf().prob(&20, 0.0), 0.001545418266496554, max_ulps = 4}

}
