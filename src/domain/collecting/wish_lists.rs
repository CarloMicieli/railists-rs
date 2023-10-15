use collections::HashMap;
use rust_decimal::prelude::*;
use std::cmp;
use std::collections;
use std::default;
use std::fmt;
use std::str;

use crate::domain::catalog::catalog_items::CatalogItem;

use super::Price;

#[derive(Debug)]
pub struct WishList {
    name: String,
    version: u8,
    items: Vec<WishListItem>,
}

impl WishList {
    pub fn new(name: &str, version: u8) -> Self {
        WishList {
            name: name.to_owned(),
            version,
            items: Vec::new(),
        }
    }

    pub fn add_item(
        &mut self,
        catalog_item: CatalogItem,
        priority: Priority,
        prices: Vec<PriceInfo>,
    ) {
        let item = WishListItem {
            catalog_item,
            priority,
            prices,
        };
        self.items.push(item);
    }

    pub fn get_items(&self) -> &Vec<WishListItem> {
        &self.items
    }

    pub fn sort_items(&mut self) {
        self.items.sort();
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct WishListItem {
    catalog_item: CatalogItem,
    priority: Priority,
    prices: Vec<PriceInfo>,
}

impl WishListItem {
    pub fn catalog_item(&self) -> &CatalogItem {
        &self.catalog_item
    }

    pub fn priority(&self) -> Priority {
        self.priority
    }

    pub fn prices(&self) -> &Vec<PriceInfo> {
        &self.prices
    }

    pub fn price_range(&self) -> Option<(&PriceInfo, &PriceInfo)> {
        if self.prices.is_empty() {
            None
        } else {
            Some((
                self.prices().iter().min().unwrap(),
                self.prices().iter().max().unwrap(),
            ))
        }
    }
}

impl cmp::PartialOrd for WishListItem {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl cmp::Ord for WishListItem {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.catalog_item.cmp(&other.catalog_item)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, PartialOrd, Ord, Hash, Default)]
pub enum Priority {
    High,
    #[default]
    Normal,
    Low,
}

impl str::FromStr for Priority {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "HIGH" => Ok(Priority::High),
            "NORMAL" => Ok(Priority::Normal),
            "LOW" => Ok(Priority::Low),
            _ => Err(anyhow!(
                "Invalid value for priority ['high', 'normal', 'low']"
            )),
        }
    }
}

impl fmt::Display for Priority {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct PriceInfo {
    shop: String,
    price: Price,
}

impl PriceInfo {
    pub fn new(shop: &str, price: Price) -> Self {
        PriceInfo {
            shop: shop.to_owned(),
            price,
        }
    }

    pub fn shop(&self) -> &str {
        &self.shop
    }

    pub fn price(&self) -> &Price {
        &self.price
    }
}

impl cmp::PartialOrd for PriceInfo {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl cmp::Ord for PriceInfo {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.price.cmp(&other.price)
    }
}

#[derive(Debug)]
pub struct WishListBudget {
    budget: Decimal,
    by_priority: HashMap<Priority, Decimal>,
}

impl WishListBudget {
    pub fn from_wish_list(wishlist: &WishList) -> Self {
        let mut map: HashMap<Priority, Decimal> = HashMap::new();

        for it in wishlist.get_items() {
            let amount = if let Some((_, max)) = it.price_range() {
                max.price.amount
            } else {
                Decimal::new(0, 0)
            };

            let en = map.entry(it.priority()).or_insert(amount);
            *en += amount;
        }

        WishListBudget {
            budget: Decimal::new(0, 0),
            by_priority: map,
        }
    }

    pub fn by_priority(&self, priority: Priority) -> Decimal {
        *self
            .by_priority
            .get(&priority)
            .unwrap_or(&Decimal::new(0, 0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod priority_tests {
        use super::*;

        #[test]
        fn it_should_parse_string_as_priority() {
            let p = "NORMAL".parse::<Priority>();

            assert!(p.is_ok());
            assert_eq!(Priority::Normal, p.unwrap());
        }

        #[test]
        fn it_should_produce_string_representations_for_priority() {
            let p = "NORMAL".parse::<Priority>().unwrap();
            assert_eq!("Normal", p.to_string());
        }

        #[test]
        fn it_should_default_to_normal_priority() {
            let p: Priority = Default::default();
            assert_eq!(Priority::Normal, p);
        }
    }

    mod price_info_tests {
        use super::*;

        #[test]
        fn it_shold_create_new_price_info_values() {
            let price = Price::euro(Decimal::new(195, 0));
            let pi = PriceInfo::new("Treni&Treni", price.clone());

            assert_eq!("Treni&Treni", pi.shop());
            assert_eq!(&price, pi.price());
        }
    }
}
