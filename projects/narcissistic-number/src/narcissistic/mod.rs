use crate::{BASE10, BASE2, BASE3, BASE4, BASE5, BASE6, BASE7, BASE8, BASE9};
use num::BigUint;

mod searcher;

pub use self::searcher::NarcissisticSearcher;

pub fn narcissistic_numbers(base: usize) -> Box<dyn Iterator<Item = BigUint>> {
    match base {
        0 | 1 => panic!("base {} doesn't a valid base", base),
        2 => Box::new(BASE2.iter().map(|u| BigUint::from(*u))),
        3 => Box::new(BASE3.iter().map(|u| BigUint::from(*u))),
        4 => Box::new(BASE4.iter().map(|u| BigUint::from(*u))),
        5 => Box::new(BASE5.iter().map(|u| BigUint::from(*u))),
        6 => Box::new(BASE6.iter().map(|u| BigUint::from(*u))),
        7 => Box::new(BASE7.iter().map(|u| BigUint::from(*u))),
        8 => Box::new(BASE8.iter().map(|u| BigUint::from(*u))),
        9 => Box::new(BASE9.iter().map(|u| BigUint::from(*u))),
        10 => Box::new(BASE10.iter().map(|u| BigUint::from(*u))),
        _ => Box::new(NarcissisticSearcher::new(base)),
    }
}
