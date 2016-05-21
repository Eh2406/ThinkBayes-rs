// This file contains code for use with "Think Stats" and
// "Think Bayes", both by Allen B. Downey, available from greenteapress.com
// Copyright 2014 Allen B. Downey
//
// Rewrite 2016 by Jacob Finkelman
// License: GNU GPLv3 http://www.gnu.org/licenses/gpl.html

use std::collections::HashMap;
use std::cmp::Eq;
use std::hash::Hash;
use std::hash::BuildHasherDefault;
use fnv::FnvHasher;
use std::borrow::Borrow;
use std::cmp::Ord;

/// Represents a probability mass function.
///
/// Values can be any hashable type; probabilities are floating-point.
/// Pmfs are not necessarily normalized.
pub struct Pmf<V: Eq + Hash + Copy> {
    d: HashMap<V, f64, BuildHasherDefault<FnvHasher>>,
}

impl<V: Eq + Hash + Copy> Pmf<V> {
    pub fn new() -> Pmf<V> {
        Pmf { d: HashMap::default() }
    }

    /// Gets an unsorted sequence of values.
    /// Note: one source of confusion is that the keys of this
    /// dictionary are the values of the Hist/Pmf, and the
    /// values of the dictionary are frequencies/probabilities.
    pub fn values(&self) -> Vec<V> {
        self.d.keys().cloned().collect()
    }

    /// Gets an unsorted sequence of (value, freq/prob) pairs.
    pub fn items(&self) -> Vec<(V, f64)> {
        self.d.iter().map(|(&val, &prb)| (val, prb)).collect()
    }

    /// Returns the total of the frequencies/probabilities in the map.
    pub fn total(&self) -> f64 {
        self.d.values().fold(0.0, |s, p| s + p)
    }

    /// Gets the probability associated with the value x.
    /// Args:
    ///     x: number value
    ///     default: value to return if the key is not there
    /// Returns:
    ///     float probability
    pub fn prob<Q>(&self, x: &Q, default: f64) -> f64
        where V: Borrow<Q>,
              Q: Hash + Eq
    {
        self.d.get(x).unwrap_or(&default).clone()
    }


    /// Sets the freq/prob associated with the value x.
    /// Args:
    ///     x: number value
    ///     y: number freq or prob
    pub fn set(&mut self, x: V, y: f64) {
        self.d.insert(x, y);
    }

    /// Increments the freq/prob associated with the value x.
    /// Args:
    ///     x: number value
    ///     term: how much to increment by
    pub fn incr(&mut self, x: V, term: f64) {
        *self.d.entry(x).or_insert(0.0) += term;
    }

    /// Scales the freq/prob associated with the value x.
    /// Args:
    ///     x: number value
    ///     factor: how much to multiply by
    pub fn mult(&mut self, x: V, factor: f64) {
        *self.d.entry(x).or_insert(0.0) *= factor;
    }

    /// Normalizes this PMF so the sum of all probs is fraction.
    /// Args:
    ///     fraction: what the total should be after normalization
    /// Returns: the total probability before normalizing
    /// panics if total probability is zero.
    pub fn normalize(&mut self, fraction: f64) -> f64 {
        let total = self.total();
        if total == 0.0 {
            panic!{"Normalize: total probability is zero."};

        }
        let factor = fraction / total;
        for (_, x) in self.d.iter_mut() {
            *x *= factor;
        }

        total
    }

    // Returns the value with the highest probability.
    //     Returns: float probability
    pub fn maximum_likelihood(&self) -> &V {
        use std::f64::NEG_INFINITY;
        let mut max = NEG_INFINITY;
        let mut max_v = None;
        for (val, &prb) in self.d.iter() {
            if prb >= max {
                max_v = Some(val);
                max = prb;
            }
        }
        max_v.expect("maximum_likelihood on empty pdf")
    }
}

impl<V: Eq + Hash + Copy + Into<f64>> Pmf<V> {
    /// Computes the mean of a PMF if V can be converted into f64.
    /// Returns:
    ///     float mean
    pub fn mean(&self) -> f64 {
        self.d.iter().fold(0.0, |s, (&x, &p)| s + x.into() * p)
    }
}

impl<V: Eq + Hash + Copy + Ord> Pmf<V> {
    /// Computes a percentile of a given Pmf.
    ///     Note: this is not super efficient.  If you are planning
    ///     to compute more than a few percentiles, compute the Cdf.
    ///     percentage: float 0-100
    ///     returns: value from the Pmf
    pub fn percentile(&self, percentage: f64) -> &V {
        let p = percentage / 100.0;
        let mut total = 0.0;
        let mut items: Vec<(&V, &f64)> = self.d.iter().collect();
        items.sort_by_key(|&(&val, _)| val);
        for &(val, &prob) in &items {
            total += prob;
            if total >= p {
                return val;
            }
        }
        items.last().expect("percentile of empty Pmf").0
    }
    /// Computes the central credible interval of a given Pmf.
    ///     Note: this is not super efficient.  If you are planning
    ///     to compute more than a few credible interval, compute the Cdf.
    ///     percentage: float 0-100
    ///     returns: value from the Pmf
    pub fn credible_interval(&self, percentage: f64) -> (&V, &V) {
        let p = (100.0 - percentage) / 2.0;
        (self.percentile(p), self.percentile(100.0 - p))
    }

    pub fn make_cdf(&self) -> super::cdf::Cdf<V> {
        self.into()
    }
}

#[cfg(test)]
mod tests_pmf {
    use super::*;
    #[test]
    fn pmf_new_set() {
        // from 2.1 #1
        let mut pmf = Pmf::new();
        for &x in &[1, 2, 3, 4, 5, 6] {
            pmf.set(x, 1.0 / 6.0)
        }
    }
    #[test]
    fn pmf_new_incr_prob() {
        // from 2.1 #2
        let mut pmf = Pmf::new();
        for &x in &["the", "the", "and", "me", "foo", "bar", "bar", "bar"] {
            pmf.incr(x, 1.0)
        }
        assert_ulps_eq!{pmf.prob(&"the", 0.0), 2.0, max_ulps = 4}
        assert_ulps_eq!{pmf.prob(&"foo", 0.0), 1.0, max_ulps = 4}
        assert_ulps_eq!{pmf.normalize(1.0), 8.0, max_ulps = 4}
        assert_ulps_eq!{pmf.prob(&"the", 0.0), 0.25, max_ulps = 4}
        assert_ulps_eq!{pmf.prob(&"foo", 0.0), 0.125, max_ulps = 4}
    }
}
