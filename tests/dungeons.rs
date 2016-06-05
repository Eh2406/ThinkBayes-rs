// This file contains code for use with "Think Bayes",
// by Allen B. Downey, available from greenteapress.com
//
// Copyright 2012 Allen B. Downey
//
// Rewrite 2016 by Jacob Finkelman
// License: GNU GPLv3 http://www.gnu.org/licenses/gpl.html

extern crate think_bayes;
use think_bayes::pmf::*;
use think_bayes::simulation::*;
#[macro_use]
extern crate approx;

// This file uses composition to emulate classes
struct Die {
    pmf: Pmf<u32>,
}

impl Die {
    pub fn new(sides: u32) -> Die {
        let mut out = Die { pmf: Pmf::new() };
        for v in 1..(sides + 1) {
            out.pmf.set(v, 1.0);
        }

        out.pmf.normalize(1.0);
        out
    }
    fn get_pdf(&self) -> &Pmf<u32> {
        &self.pmf
    }
}

#[test]
fn suite_dungeons() {
    let d6 = Die::new(6);
    let dice = [&d6; 3];
    let sample_n = 100_000;
    let delta_sample_n = 2.58 / ((sample_n as f64).sqrt());
    let mut three = sample_sum(dice.iter().map(|i| i.get_pdf()), sample_n);
    three.normalize(1.0);
    let three_exact = d6.get_pdf() + d6.get_pdf() + d6.get_pdf();
    for &(v, p) in three_exact.items().iter() {
        // sudo statistical test should fail approx len(three_exact) * .99 of the time
        assert_relative_eq!{p, three.prob(&v, 0.0), epsilon = p.sqrt() * delta_sample_n};
    }
}
