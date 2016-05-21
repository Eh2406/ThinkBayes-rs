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
use think_bayes::cdf::*;
#[macro_use]
extern crate approx;

// This file contains a partial solution to a problem from
// MacKay, "Information Theory, Inference, and Learning Algorithms."
//
//     Exercise 3.15 (page 50): A statistical statement appeared in
//     "The Guardian" on Friday January 4, 2002:
//
//         When spun on edge 250 times, a Belgian one-euro coin came
//         up heads 140 times and tails 110.  'It looks very suspicious
//         to me,' said Barry Blight, a statistics lecturer at the London
//         School of Economics.  'If the coin were unbiased, the chance of
//         getting a result as extreme as that would be less than 7%.'
//
// MacKay asks, "But do these data give evidence that the coin is biased
// rather than fair?"

struct Euro {
    pmf: Pmf<u8>,
}

impl Euro {
    pub fn new_uniform() -> Euro {
        let mut out = Euro { pmf: Pmf::new() };
        for v in 0..101 {
            out.pmf.set(v, 1.0);
        }

        out.pmf.normalize(1.0);
        out
    }
    pub fn new_triangle() -> Euro {
        let mut out = Euro { pmf: Pmf::new() };
        for v in 0..51 {
            out.pmf.set(v, v.into());
        }
        for v in 51..101 {
            out.pmf.set(v, (100 - v).into());
        }

        out.pmf.normalize(1.0);
        out
    }
    fn get_pdf(&self) -> &Pmf<u8> {
        &self.pmf
    }
}

impl Suite<char, u8> for Euro {
    fn get_mut_pmf(&mut self) -> &mut Pmf<u8> {
        &mut self.pmf
    }
    /// Computes the likelihood of the data under the hypothesis.
    ///
    /// hypo: integer value of x, the probability of heads (0-100)
    /// data: string 'H' or 'T'
    fn likelihood(&self, &data: &char, &hypo: &u8) -> f64 {
        let x: f64 = hypo.into();
        if data == 'H' {
            x / 100.0
        } else {
            1.0 - (x / 100.0)
        }
    }
}

// wow we can reuse the same structue!

impl Suite<(u16, u16), u8> for Euro {
    fn get_mut_pmf(&mut self) -> &mut Pmf<u8> {
        &mut self.pmf
    }
    /// Computes the likelihood of the data under the hypothesis.
    ///
    ///     hypo: integer value of x, the probability of heads (0-100)
    ///     data: tuple of (number of heads, number of tails)
    fn likelihood(&self, &(heads, tails): &(u16, u16), &hypo: &u8) -> f64 {
        let x: f64 = hypo.into();
        let x = x / 100.0;
        x.powi(heads.into()) * (1.0 - x).powi(tails.into())
    }
}

#[test]
fn suite_euro() {
    let mut suite = Euro::new_uniform();
    for data in ['H'].iter().cycle().take(140) {
        suite.update(data);
    }
    for data in ['T'].iter().cycle().take(110) {
        suite.update(data);
    }
    let suite = suite.get_pdf();
    // 4.2  Summarizing the posterior
    assert_eq!(suite.maximum_likelihood(), &56);
    assert_ulps_eq!{suite.mean(), 55.952380952380935, max_ulps = 4};
    assert_eq!(suite.percentile(50.0), &56);
    assert_eq!(suite.credible_interval(90.0), (&51, &61));
    let cdf: Cdf<_> = suite.into();
    assert_eq!(cdf.percentile(50.0), 56);
    assert_eq!(cdf.credible_interval(90.0), (51, 61));
    assert_ulps_eq!{suite.prob(&50, 0.0), 0.02097652612954465, max_ulps = 4};
    // 4.3  Swamping the priors
    let mut suite2 = Euro::new_triangle();
    for data in ['H'].iter().cycle().take(140) {
        suite2.update(data);
    }
    for data in ['T'].iter().cycle().take(110) {
        suite2.update(data);
    }
    let suite2 = suite2.get_pdf();
    assert_eq!(suite2.maximum_likelihood(), &56);
    assert_ulps_eq!{suite2.mean(), 55.74349943859506, max_ulps = 4};
    assert_eq!(suite2.percentile(50.0), &56);
    assert_eq!(suite2.credible_interval(90.0), (&51, &61));
    let cdf: Cdf<_> = suite2.into();
    assert_eq!(cdf.percentile(50.0), 56);
    assert_eq!(cdf.credible_interval(90.0), (51, 61));
    assert_ulps_eq!{suite2.prob(&50, 0.0), 0.023847537214693623, max_ulps = 4};
    // 4.4  Optimization
    let mut suite3 = Euro::new_triangle();
    suite3.update_set(['H'].iter().cycle().take(140));
    suite3.update_set(['T'].iter().cycle().take(110));
    let suite3 = suite3.get_pdf();
    assert_eq!(suite3.maximum_likelihood(), &56);
    assert_ulps_eq!{suite3.mean(), 55.74349943859506, max_ulps = 4};
    assert_eq!(suite3.percentile(50.0), &56);
    assert_eq!(suite3.credible_interval(90.0), (&51, &61));
    let cdf: Cdf<_> = suite3.into();
    assert_eq!(cdf.percentile(50.0), 56);
    assert_eq!(cdf.credible_interval(90.0), (51, 61));
    assert_ulps_eq!{suite3.prob(&50, 0.0), 0.023847537214693623, max_ulps = 4};
    let values = suite2.values();
    for hypo in &values {
        assert_ulps_eq!{suite2.prob(hypo, 0.0), suite3.prob(hypo, 0.0), epsilon = 0.00000001};
    }
    let mut suite4 = Euro::new_triangle();
    suite4.update(&(140, 110));
    let suite4 = suite4.get_pdf();
    assert_eq!(suite4.maximum_likelihood(), &56);
    assert_ulps_eq!{suite4.mean(), 55.74349943859506, max_ulps = 4};
    assert_eq!(suite4.percentile(50.0), &56);
    assert_eq!(suite4.credible_interval(90.0), (&51, &61));
    let cdf: Cdf<_> = suite4.into();
    assert_eq!(cdf.percentile(50.0), 56);
    assert_eq!(cdf.credible_interval(90.0), (51, 61));
    assert_ulps_eq!{suite4.prob(&50, 0.0), 0.023847537214693623, max_ulps = 4};
    let values = suite2.values();
    for hypo in &values {
        assert_ulps_eq!{suite2.prob(hypo, 0.0), suite4.prob(hypo, 0.0), epsilon = 0.00000001};
    }

}
