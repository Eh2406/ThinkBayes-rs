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
    pub fn new<I: Iterator<Item = u32>>(hypos: I, alpha: f64) -> Train {
        let mut out = Train { pmf: Pmf::new() };
        for v in hypos {
            let hypo: f64 = v.into();
            out.pmf.set(v, hypo.powf(-alpha));
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
fn suite_train_power_law() {
    let mut suite = Train::new(1..1001, 1.0);
    suite.update(&60);
    assert_ulps_eq!{suite.get_pdf().prob(&59, 0.0), 0.0, max_ulps = 4}
    assert_ulps_eq!{suite.get_pdf().prob(&60, 0.0), 0.017573278852195767, max_ulps = 4}
    assert_ulps_eq!{suite.get_pdf().prob(&61, 0.0), 0.017001828505214936, max_ulps = 4}
    assert_ulps_eq!{suite.get_pdf().prob(&100, 0.0), 0.006326380386790478, max_ulps = 4}
    assert_ulps_eq!{suite.get_pdf().prob(&500, 0.0), 0.0002530552154716191, max_ulps = 4}
    assert_ulps_eq!{suite.get_pdf().prob(&1000, 0.0), 0.00006326380386790477, max_ulps = 4}
    assert_ulps_eq!{suite.get_pdf().mean(), 178.54735317971586, max_ulps = 4}
    assert_eq!{*suite.get_pdf().percentile(5.0), 62}
    assert_eq!{*suite.get_pdf().percentile(95.0), 559}
    suite.update(&30);
    suite.update(&90);
    assert_ulps_eq!{suite.get_pdf().mean(), 133.2752313750311, max_ulps = 4}
    assert_eq!{*suite.get_pdf().percentile(5.0), 91}
    assert_eq!{*suite.get_pdf().percentile(95.0), 242}

    let mut suite = Train::new(1..501, 1.0);
    suite.update(&60);
    assert_ulps_eq!{suite.get_pdf().mean(), 143.8123198209791, max_ulps = 4}
    assert_eq!{*suite.get_pdf().percentile(5.0), 62}
    assert_eq!{*suite.get_pdf().percentile(95.0), 365}
    suite.update(&30);
    suite.update(&90);
    assert_ulps_eq!{suite.get_pdf().mean(), 130.70846986255995, max_ulps = 4}
    assert_eq!{*suite.get_pdf().percentile(5.0), 91}
    assert_eq!{*suite.get_pdf().percentile(95.0), 235}

    let mut suite = Train::new(1..2001, 1.0);
    suite.update(&60);
    assert_ulps_eq!{suite.get_pdf().mean(), 215.5689255061552, max_ulps = 4}
    assert_eq!{*suite.get_pdf().percentile(5.0), 63}
    assert_eq!{*suite.get_pdf().percentile(95.0), 760}
    suite.update(&30);
    suite.update(&90);
    assert_ulps_eq!{suite.get_pdf().mean(), 133.9974630807312, max_ulps = 4}
    assert_eq!{*suite.get_pdf().percentile(5.0), 91}
    assert_eq!{*suite.get_pdf().percentile(95.0), 243}
}
