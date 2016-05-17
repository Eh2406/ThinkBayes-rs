use super::pmf::*;
use std::cmp::Eq;
use std::hash::Hash;

/// Represents a suite of hypotheses and their probabilities.
pub trait Suite<D, V: Eq + Hash + Copy> {
    // Computes the likelihood of the data under the hypothesis.
    //     hypo: some representation of the hypothesis
    //     data: some representation of the data
    fn likelihood(&self, data: &D, hypo: &V) -> f64;
    fn get_mut_pmf(&mut self) -> &mut Pmf<V>;
    /// Updates each hypothesis based on the data.
    ///    data: any representation of the data
    ///    returns: the normalizing constant
    fn update(&mut self, data: &D) {
        let values = self.get_mut_pmf().values();
        for hypo in values {
            let like = self.likelihood(data, &hypo);
            self.get_mut_pmf().mult(hypo, like)
        }
        self.get_mut_pmf().normalize(1.0);
    }
}
