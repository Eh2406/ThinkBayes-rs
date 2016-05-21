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

    /// Updates each hypothesis based on the dataset.
    ///     This is more efficient than calling Update repeatedly because
    ///     it waits until the end to Normalize.
    ///     Modifies the suite directly; if you want to keep the original, make
    ///     a copy.
    ///     dataset: a sequence of data
    ///     returns: the normalizing constant
    fn update_set<'a, I: Iterator<Item = &'a D>>(&mut self, dataset: I)
        where D: 'a
    {
        let values = self.get_mut_pmf().values();
        for data in dataset {
            for &hypo in &values {
                let like = self.likelihood(data, &hypo);
                self.get_mut_pmf().mult(hypo, like)
            }
        }
        self.get_mut_pmf().normalize(1.0);
    }
}
