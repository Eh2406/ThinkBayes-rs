// This file contains code for use with "Think Stats" and
// "Think Bayes", both by Allen B. Downey, available from greenteapress.com
// Copyright 2014 Allen B. Downey
//
// Rewrite 2016 by Jacob Finkelman
// License: GNU GPLv3 http://www.gnu.org/licenses/gpl.html

use std::f64::{INFINITY, NAN};

/// Computes odds for a given probability.
/// Example: p=0.75 means 75 for and 25 against, or 3:1 odds in favor.
/// Note: when p=1, the formula for odds divides by zero, which is
/// normally undefined.  But I think it is reasonable to define Odds(1)
/// to be infinity, so that's what this function does.
/// p: float 0-1 else returns NAN
pub fn odds(p: f64) -> f64 {
    if p < 0.0 || p > 1.0 {
        NAN
    } else if p == 1.0 {
        INFINITY
    } else {
        p / (1.0 - p)
    }
}

#[cfg(test)]
mod tests_odds {
    use super::*;
    #[test]
    fn odds_to_small() {
        assert!{odds(-1.0).is_nan()}
    }
    #[test]
    fn odds_to_big() {
        assert!{odds(2.0).is_nan()}
    }
    #[test]
    fn odds_to_just_big() {
        assert!{odds(1.0).is_infinite()}
        assert!{odds(1.0) > 0.0}
    }
    #[test]
    fn odds_in_the_mid() {
        assert_ulps_eq!{odds(0.5), 1.0, max_ulps = 4}
    }
    #[test]
    fn odds_in_the_mid2() {
        assert_ulps_eq!{odds(0.25), 1.0 / 3.0, max_ulps = 4}
    }
    #[test]
    fn odds_probability_quickcheck() {
        use quickcheck::{quickcheck, TestResult};
        fn test(p: f64) -> TestResult {
            if p < 0.0 || p >= 1.0 {
                TestResult::discard()
            } else {
                TestResult::from_bool(ulps_eq!{probability(odds(p)), p, max_ulps = 4})
            }
        }
        quickcheck(test as fn(f64) -> TestResult);
    }
}


/// Computes the probability corresponding to given odds.
/// Example: o=2 means 2:1 odds in favor, or 2/3 probability
/// o: float odds, strictly positive else returns NAN
/// Returns: float probability
pub fn probability(o: f64) -> f64 {
    if o < 0.0 {
        NAN
    } else {
        o / (o + 1.0)
    }
}

#[cfg(test)]
mod tests_probability {
    use super::*;
    #[test]
    fn probability_to_small() {
        assert!{probability(-1.0).is_nan()}
    }
    #[test]
    fn probability_in_the_mid() {
        assert_ulps_eq!{probability(0.5), 1.0 / 3.0, max_ulps = 4}
    }
    #[test]
    fn probability_in_the_mid2() {
        assert_ulps_eq!{probability(0.25), 0.20, max_ulps = 4}
    }
}

/// Computes the probability corresponding to given odds.
/// Example: yes=2, no=1 means 2:1 odds in favor, or 2/3 probability.
pub fn probability2(yes: u64, no: u64) -> f64 {
    yes as f64 / ((yes + no) as f64)
}

#[cfg(test)]
mod tests_probability2 {
    use super::*;
    #[test]
    fn probability2_in_the_mid() {
        assert_ulps_eq!{probability2(1, 2), 1.0 / 3.0, max_ulps = 4}
    }
    #[test]
    fn probability2_in_the_mid2() {
        assert_ulps_eq!{probability2(1, 4), 0.20, max_ulps = 4}
    }
}
