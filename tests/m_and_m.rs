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

use std::collections::HashMap;
use std::cmp::Eq;
use std::hash::Hash;
use std::hash::BuildHasherDefault;
extern crate fnv;
use fnv::FnvHasher;

// This file uses composition to emulate classes
#[allow(non_camel_case_types)]
struct M_and_M<V: Eq + Hash + Copy> {
    pmf: Pmf<V>,
    hypotheses: HashMap<V,
                        HashMap<V,
                                HashMap<V, f64, BuildHasherDefault<FnvHasher>>,
                                BuildHasherDefault<FnvHasher>>,
                        BuildHasherDefault<FnvHasher>>,
}

impl<V: Eq + Hash + Copy> M_and_M<V> {
    pub fn new(hypotheses: HashMap<V,
                                   HashMap<V,
                                           HashMap<V, f64, BuildHasherDefault<FnvHasher>>,
                                           BuildHasherDefault<FnvHasher>>,
                                   BuildHasherDefault<FnvHasher>>)
               -> M_and_M<V> {
        let mut out = M_and_M {
            pmf: Pmf::new(),
            hypotheses: hypotheses,
        };
        for &v in out.hypotheses.keys() {
            out.pmf.set(v, 1.0);
        }

        out.pmf.normalize(1.0);
        out
    }
    fn get_pdf(&self) -> &Pmf<V> {
        &self.pmf
    }
}

impl<V: Eq + Hash + Copy> Suite<(V, V), V> for M_and_M<V> {
    fn get_mut_pmf(&mut self) -> &mut Pmf<V> {
        &mut self.pmf
    }
    fn likelihood(&self, data: &(V, V), hypo: &V) -> f64 {
        let &(bag, color) = data;
        self.hypotheses[hypo][&bag][&color]
    }
}

#[test]
fn suite_m_and_m() {
    let mut mix94 = HashMap::default();
    mix94.insert("brown", 30.0);
    mix94.insert("yellow", 20.0);
    mix94.insert("red", 20.0);
    mix94.insert("green", 10.0);
    mix94.insert("orange", 10.0);
    mix94.insert("tan", 10.0);
    let mut mix96 = HashMap::default();
    mix96.insert("blue", 24.0);
    mix96.insert("green", 20.0);
    mix96.insert("orange", 16.0);
    mix96.insert("yellow", 14.0);
    mix96.insert("red", 13.0);
    mix96.insert("brown", 13.0);

    let mut hypo_a = HashMap::default();
    hypo_a.insert("bag1", mix94.clone());
    hypo_a.insert("bag2", mix96.clone());

    let mut hypo_b = HashMap::default();
    hypo_b.insert("bag1", mix96.clone());
    hypo_b.insert("bag2", mix94.clone());

    let mut hypotheses = HashMap::default();
    hypotheses.insert("A", hypo_a);
    hypotheses.insert("B", hypo_b);

    let mut suite = M_and_M::new(hypotheses);
    assert_ulps_eq!{suite.get_pdf().prob(&"A", 0.0), 0.5, max_ulps = 4}
    assert_ulps_eq!{suite.get_pdf().prob(&"B", 0.0), 0.5, max_ulps = 4}

    suite.update(&("bag1", "yellow"));
    assert_ulps_eq!{suite.get_pdf().prob(&"A", 0.0), 0.5882352941176471, max_ulps = 4}
    assert_ulps_eq!{suite.get_pdf().prob(&"B", 0.0), 0.4117647058823529, max_ulps = 4}

    suite.update(&("bag2", "green"));
    assert_ulps_eq!{suite.get_pdf().prob(&"A", 0.0), 0.7407407407407407, max_ulps = 4}
    assert_ulps_eq!{suite.get_pdf().prob(&"B", 0.0), 0.2592592592592592, max_ulps = 4}
}
