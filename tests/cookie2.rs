// This file contains code for use with "Think Bayes",
// by Allen B. Downey, available from greenteapress.com
//
// Copyright 2012 Allen B. Downey
//
// Rewrite 2016 by Jacob Finkelman
// License: GNU GPLv3 http://www.gnu.org/licenses/gpl.html

extern crate think_bayes;
use think_bayes::pmf::*;
#[macro_use]
extern crate approx;

use std::collections::HashMap;
use std::cmp::Eq;
use std::hash::Hash;
use std::hash::BuildHasherDefault;
extern crate fnv;
use fnv::FnvHasher;

// This file uses composition to emulate classes
struct Cookie<V: Eq + Hash + Copy> {
    pmf: Pmf<V>,
    mixes: HashMap<V,
                   HashMap<V, f64, BuildHasherDefault<FnvHasher>>,
                   BuildHasherDefault<FnvHasher>>,
}

impl<V: Eq + Hash + Copy> Cookie<V> {
    pub fn new(mixes: HashMap<V,
                              HashMap<V, f64, BuildHasherDefault<FnvHasher>>,
                              BuildHasherDefault<FnvHasher>>)
               -> Cookie<V> {
        let mut cookie = Cookie {
            pmf: Pmf::new(),
            mixes: mixes,
        };
        for &v in cookie.mixes.keys() {
            cookie.pmf.set(v, 1.0);
        }

        cookie.pmf.normalize(1.0);
        cookie
    }
    fn get_pdf(&self) -> &Pmf<V> {
        &self.pmf
    }
    fn update(&mut self, data: &V) {
        for hypo in self.pmf.values() {
            let like = self.likelihood(data, hypo);
            self.pmf.mult(hypo, like)
        }
        self.pmf.normalize(1.0);
    }
    fn likelihood(&self, data: &V, hypo: V) -> f64 {
        let mix = self.mixes.get(&hypo).unwrap();
        mix.get(data).unwrap_or(&0.0).clone()
    }
}


#[test]
fn pmf_cookie_composition() {
    let mut bowl1 = HashMap::default();
    bowl1.insert("vanilla", 0.75);
    bowl1.insert("chocolate", 0.25);
    let mut bowl2 = HashMap::default();
    bowl2.insert("vanilla", 0.5);
    bowl2.insert("chocolate", 0.5);
    let mut mixes = HashMap::default();
    mixes.insert("Bowl 1", bowl1);
    mixes.insert("Bowl 2", bowl2);

    let mut pmf = Cookie::new(mixes);
    assert_ulps_eq!{pmf.get_pdf().prob(&"Bowl 1", 0.0), 0.5, max_ulps = 4}
    assert_ulps_eq!{pmf.get_pdf().prob(&"Bowl 2", 0.0), 0.5, max_ulps = 4}

    pmf.update(&"vanilla");
    assert_ulps_eq!{pmf.get_pdf().prob(&"Bowl 1", 0.0), 0.6, max_ulps = 4}
    assert_ulps_eq!{pmf.get_pdf().prob(&"Bowl 2", 0.0), 0.4, max_ulps = 4}

    let dataset = ["vanilla", "chocolate", "vanilla"];
    for data in &dataset {
        pmf.update(data);
    }
    assert_ulps_eq!{pmf.get_pdf().prob(&"Bowl 1", 0.0), 0.627906976744186, max_ulps = 4}
    assert_ulps_eq!{pmf.get_pdf().prob(&"Bowl 2", 0.0), 0.37209302325581395, max_ulps = 4}
}
