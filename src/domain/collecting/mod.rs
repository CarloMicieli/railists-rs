pub mod collections;
pub mod wish_lists;

use rust_decimal::prelude::*;
use std::fmt;
use std::str;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct Price {
    amount: Decimal,
    currency: String,
}

impl Price {
    pub fn euro(amount: Decimal) -> Self {
        Price {
            amount,
            currency: "EUR".to_owned(),
        }
    }
}

impl str::FromStr for Price {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err("Invalid price: cannot be empty".to_owned());
        }

        let mut it = s.split_ascii_whitespace();
        let amount = it
            .next()
            .map(|s| s.replace(',', "."))
            .map(|amount| Decimal::from_str(&amount))
            .unwrap();

        Ok(Price {
            amount: amount.unwrap(),
            currency: String::from("EUR"),
        })
    }
}

impl core::iter::Sum for Price {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        let total_amount = iter.map(|it| it.amount).sum();
        Price {
            amount: total_amount,
            currency: String::from("EUR"), //TODO: fixme
        }
    }
}

impl fmt::Display for Price {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.amount, self.currency)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_me() {
        assert_eq!(1, 1);
    }
}
