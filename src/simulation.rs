// This file contains code for use with "Think Stats" and
// "Think Bayes", both by Allen B. Downey, available from greenteapress.com
// Copyright 2014 Allen B. Downey
//
// Rewrite 2016 by Jacob Finkelman
// License: GNU GPLv3 http://www.gnu.org/licenses/gpl.html
use std::cmp::Eq;
use std::hash::Hash;
use std::cmp::Ord;
use std::ops::Add;
use itertools::Itertools;
use super::{Cdf, Pmf};

/// Chooses a random value from each dist and returns the sum.
///
/// dists: sequence of Cdf objects if V can be converted into f64.
///
/// returns: numerical sum
pub fn random_sum<'a, V, I>(dists: I) -> V
    where V: 'a + Eq + Copy + Ord + Hash + Add<Output = V>,
          I: Iterator<Item = &'a Cdf<V>>
{
    dists.map(|dist| dist.random()).fold1(|s, r| s + r).unwrap()
}

/// Draws a sample of sums from a list of distributions.
///
/// dists: iter of Pmf or Cdf objects
/// n: sample size
///
/// returns: new Pmf of sums
pub fn sample_sum<V, C, I>(dists: I, n: usize) -> Pmf<V>
    where V: Eq + Copy + Ord + Hash + Add<Output = V>,
          C: Into<Cdf<V>>,
          I: Iterator<Item = C>
{
    let dists: Vec<_> = dists.map(|i| i.into()).collect();
    let mut pdf = Pmf::new();
    for _ in 0..n {
        pdf.incr(random_sum(dists.iter()), 1.0);
    }
    pdf
}
