// This file contains code for use with "Think Bayes"
// by Allen B. Downey, available from greenteapress.com
// Copyright 2014 Allen B. Downey
//
// Rewrite 2016 by Jacob Finkelman
// License: GNU GPLv3 http://www.gnu.org/licenses/gpl.html

extern crate think_bayes;
use think_bayes::Pmf;
#[macro_use]
extern crate approx;

#[test]
fn pmf_cookie_problem() {
    // from 2.2
    let mut pmf = Pmf::new();
    pmf.set("Bowl 1", 0.5);
    pmf.set("Bowl 2", 0.5);
    pmf.mult("Bowl 1", 0.75);
    pmf.mult("Bowl 2", 0.5);
    assert_ulps_eq!{pmf.prob(&"Bowl 1", 0.0), 0.375, max_ulps = 4}
    assert_ulps_eq!{pmf.prob(&"Bowl 2", 0.0), 0.25, max_ulps = 4}
    assert_ulps_eq!{pmf.normalize(1.0), 0.625, max_ulps = 4}
    assert_ulps_eq!{pmf.prob(&"Bowl 1", 0.0), 0.6, max_ulps = 4}
    assert_ulps_eq!{pmf.prob(&"Bowl 2", 0.0), 0.4, max_ulps = 4}

}
