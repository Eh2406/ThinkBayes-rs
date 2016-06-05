
use std::cmp::Eq;
use std::hash::Hash;
use std::cmp::Ord;
use rand::{thread_rng, Rng};

/// Represents a cumulative distribution function.
/// Attributes:
///     xs: sequence of values
///     ps: sequence of probabilities
///     label: string used as a graph label.
pub struct Cdf<V: Eq + Copy + Ord> {
    xs: Vec<V>,
    ps: Vec<f64>,
}

impl<V: Eq + Copy + Ord> Cdf<V> {
    /// Returns InverseCDF(p), the value that corresponds to probability p.
    /// Args:
    ///     p: number in the range [0, 1]
    pub fn value(&self, p: f64) -> V {
        if p < 0.0 || p > 1.0 {
            panic!("Probability p must be in range [0, 1]")
        }
        let index = self.ps
            .binary_search_by(|v| v.partial_cmp(&p).expect("Couldn't compare values"));
        self.xs[index.unwrap_or_else(|x| x)]
    }

    /// Returns the value that corresponds to percentile p.
    /// Args:
    ///     p: number in the range [0, 100]
    /// Returns:
    ///     number value
    pub fn percentile(&self, p: f64) -> V {
        self.value(p / 100.0)
    }

    /// Chooses a random value from this distribution.
    pub fn random(&self) -> V { ;
        // maybe faster with lazy_static(distributions::Range)
        self.value(thread_rng().gen_range(0.0, 1.0))
    }


    /// Generates a random sample from this distribution.
    ///
    /// n: usize length of the sample
    /// returns: Vec<V>
    pub fn sample(&self, n: usize) -> Vec<V> {
        // O(n*log(len(cdf)))
        (0..n).map(|_| self.random()).collect()
    }

    /// Computes the central credible interval of a given Pmf.
    ///     percentage: float 0-100
    ///     returns: value from the Pmf
    pub fn credible_interval(&self, percentage: f64) -> (V, V) {
        let p = (100.0 - percentage) / 2.0;
        (self.percentile(p), self.percentile(100.0 - p))
    }
}

impl<'a, V: Eq + Hash + Copy + Ord> From<&'a super::pmf::Pmf<V>> for Cdf<V> {
    fn from(pmf: &'a super::pmf::Pmf<V>) -> Self {
        let mut items = pmf.items();
        items.sort_by_key(|&(val, _)| val);
        super::cdf::Cdf {
            xs: items.iter().map(|&(val, _)| val).collect(),
            ps: items.iter()
                .scan(0.0, |s, &(_, prb)| {
                    *s += prb;
                    Some(s.clone())
                })
                .collect(),
        }
    }
}