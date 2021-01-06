use itertools::Itertools;
use std::cmp;
use std::fmt;
use std::str;

use crate::domain::catalog::{
    brands::Brand, categories::Category, rolling_stocks::RollingStock,
    scales::Scale,
};

use super::rolling_stocks::Epoch;

/// It represent a catalog item number.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct ItemNumber(String);

impl ItemNumber {
    /// Creates a new ItemNumber from the string slice, it needs to panic when the
    /// provided string slice is empty.
    pub fn new(value: &str) -> Result<Self, &'static str> {
        if value.is_empty() {
            Err("Item number cannot blank")
        } else {
            Ok(ItemNumber(value.to_owned()))
        }
    }

    /// Returns the item number value, this cannot be blank.
    pub fn value(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for ItemNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

// The power methods for the model.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum PowerMethod {
    /// Direct current.
    DC,

    /// Alternating current (Maerklin).
    AC,
}

impl fmt::Display for PowerMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl str::FromStr for PowerMethod {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "DC" => Ok(PowerMethod::DC),
            "AC" => Ok(PowerMethod::AC),
            _ => Err("Invalid power method value"),
        }
    }
}

/// A catalog item, it can contain one or more rolling stock.
///
/// A catalog item is identified by its catalog item number.
#[derive(Debug)]
pub struct CatalogItem {
    brand: Brand,
    item_number: ItemNumber,
    description: String,
    rolling_stocks: Vec<RollingStock>,
    category: Category,
    scale: Scale,
    power_method: PowerMethod,
    count: u8,
}

impl PartialEq for CatalogItem {
    fn eq(&self, other: &Self) -> bool {
        self.brand == other.brand && self.item_number == other.item_number
    }
}

impl cmp::Eq for CatalogItem {}

impl cmp::Ord for CatalogItem {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        let cmp1 = self.brand().cmp(other.brand());
        if cmp1 == cmp::Ordering::Equal {
            return self.item_number.cmp(&other.item_number);
        }

        cmp1
    }
}

impl cmp::PartialOrd for CatalogItem {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl CatalogItem {
    pub fn new(
        brand: Brand,
        item_number: ItemNumber,
        description: String,
        rolling_stocks: Vec<RollingStock>,
        power_method: PowerMethod,
        scale: Scale,
        count: u8,
    ) -> Self {
        let category = Self::extract_category(&rolling_stocks);
        CatalogItem {
            brand,
            item_number,
            description,
            rolling_stocks,
            category,
            count,
            power_method,
            scale,
        }
    }

    /// Brand for this catalog item.
    pub fn brand(&self) -> &Brand {
        &self.brand
    }

    /// The item number as in the corresponding brand catalog.
    pub fn item_number(&self) -> &ItemNumber {
        &self.item_number
    }

    pub fn rolling_stocks(&self) -> &Vec<RollingStock> {
        &self.rolling_stocks
    }

    pub fn is_locomotive(&self) -> bool {
        self.category() == Category::Locomotives
    }

    pub fn category(&self) -> Category {
        self.category
    }

    pub fn count(&self) -> u8 {
        self.count
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn scale(&self) -> &Scale {
        &self.scale
    }

    pub fn power_method(&self) -> PowerMethod {
        self.power_method
    }

    fn extract_category(rolling_stocks: &Vec<RollingStock>) -> Category {
        let categories = rolling_stocks
            .iter()
            .map(|rs| rs.category())
            .sorted()
            .dedup()
            .collect::<Vec<Category>>();

        if categories.len() == 1 {
            return categories[0];
        }

        Category::Trains
    }

    // fn extract_epoch(rolling_stocks: &Vec<RollingStock>) -> Option<&Epoch> {
    //     let epochs = rolling_stocks
    //         .iter()
    //         .map(|rs| rs.epoch())
    //         .sorted()
    //         .dedup()
    //         .collect::<Vec<Epoch>>();

    //     if epochs.len() == 1 {
    //         return epochs.get(0);
    //     }

    //     None
    // }
}

impl fmt::Display for CatalogItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {} ({})",
            self.brand,
            self.item_number,
            self.category.symbol(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod item_number_tests {
        use super::*;

        #[test]
        fn it_should_create_new_item_numbers() {
            let n = ItemNumber::new("123456");
            assert_eq!(n.unwrap().value(), "123456");
        }

        #[test]
        fn it_should_fail_to_convert_empty_string_slices_as_item_numbers() {
            let item_number = ItemNumber::new("");
            assert!(item_number.is_err());
        }
    }
}
