use crate::domain::catalog::catalog_items::CatalogItem;
use std::default;
use std::fmt;
use std::str;

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

    pub fn add_item(&mut self, catalog_item: CatalogItem, priority: Priority) {
        let item = WishListItem {
            catalog_item,
            priority,
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

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct WishListItem {
    catalog_item: CatalogItem,
    priority: Priority,
}

impl WishListItem {
    pub fn catalog_item(&self) -> &CatalogItem {
        &self.catalog_item
    }

    pub fn priority(&self) -> Priority {
        self.priority
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
pub enum Priority {
    High,
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

impl default::Default for Priority {
    fn default() -> Self {
        Priority::Normal
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
}
