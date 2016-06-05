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
use think_bayes::utils::*;
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

fn pmf_max(pmf1: &Pmf<u32>, pmf2: &Pmf<u32>) -> Pmf<u32> {
    use std::cmp::max;
    let mut res = Pmf::new();
    let items1 = pmf1.items();
    let items2 = pmf2.items();
    for &(v1, p1) in items1.iter() {
        for &(v2, p2) in items2.iter() {
            res.incr(max(v1, v2), p1 * p2)
        }
    }
    res
}

#[test]
fn suite_dungeons_sum() {
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

#[test]
fn suite_dungeons_max() {
    let d6 = Die::new(6);
    let three_exact = d6.get_pdf() + d6.get_pdf() + d6.get_pdf();

    // compute the distribution of the best attribute the hard way
    let best_attr2 = pmf_max(&three_exact, &three_exact);
    let best_attr4 = pmf_max(&best_attr2, &best_attr2);
    let mut best_attr6 = pmf_max(&best_attr4, &best_attr2);
    best_attr6.normalize(1.0);
    let best_attr6_cdf = best_attr6.make_cdf();

    // and the easy way
    let best_attr_cdf = three_exact.make_cdf().max(6);

    for &(v, _) in best_attr6.items().iter() {
        assert_ulps_eq!{best_attr6_cdf.prob(v), best_attr_cdf.prob(v), max_ulps = 9};
    }
}

#[test]
fn suite_dungeons_mixtures() {
    let dice = [(Die::new(4), 5.0),
                (Die::new(6), 4.0),
                (Die::new(8), 3.0),
                (Die::new(12), 2.0),
                (Die::new(20), 1.0)];
    let mut mix = make_mixture(dice.iter().map(|&(ref d, p)| (d.get_pdf(), p)));
    mix.normalize(1.0);
    assert_ulps_eq!{mix.prob(&1, 0.0), 0.1672222222222222, max_ulps = 4};
    assert_ulps_eq!{mix.prob(&4, 0.0), 0.1672222222222222, max_ulps = 4};
    assert_ulps_eq!{mix.prob(&5, 0.0), 0.08388888888888889, max_ulps = 4};
    assert_ulps_eq!{mix.prob(&6, 0.0), 0.08388888888888889, max_ulps = 4};
    assert_ulps_eq!{mix.prob(&7, 0.0), 0.03944444444444444, max_ulps = 4};
    assert_ulps_eq!{mix.prob(&8, 0.0), 0.03944444444444444, max_ulps = 4};
    assert_ulps_eq!{mix.prob(&9, 0.0), 0.014444444444444446, max_ulps = 4};
    assert_ulps_eq!{mix.prob(&12, 0.0), 0.014444444444444446, max_ulps = 4};
    assert_ulps_eq!{mix.prob(&13, 0.0), 0.0033333333333333335, max_ulps = 4};
    assert_ulps_eq!{mix.prob(&20, 0.0), 0.0033333333333333335, max_ulps = 4};
}
