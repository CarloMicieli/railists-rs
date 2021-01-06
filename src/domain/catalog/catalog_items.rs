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
            _ => Err("Invalid value for power methods [allowed: 'AC' or 'DC']"),
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

    mod power_method_tests {
        use super::*;

        #[test]
        fn it_should_parse_string_as_power_methods() {
            let pm = "AC".parse::<PowerMethod>();
            assert!(pm.is_ok());
            assert_eq!("AC", pm.unwrap().to_string());
        }
    }

    mod catalog_item_tests {
        use crate::domain::catalog::{
            categories::{FreightCarType, LocomotiveType, PassengerCarType},
            railways::Railway,
            rolling_stocks::{
                Control, DccInterface, LengthOverBuffer, ServiceLevel,
            },
        };

        use super::*;

        fn new_locomotive() -> RollingStock {
            RollingStock::new_locomotive(
                String::from("E.656"),
                String::from("E.656 210"),
                Some(String::from("1a serie")),
                Railway::new("FS"),
                Epoch::IV,
                LocomotiveType::ElectricLocomotive,
                Some(String::from("Milano Centrale")),
                Some(String::from("blu/grigio")),
                Some(LengthOverBuffer::new(210)),
                Some(Control::DccReady),
                Some(DccInterface::Nem652),
            )
        }

        fn new_passenger_car() -> RollingStock {
            RollingStock::new_passenger_car(
                String::from("UIC-Z"),
                None,
                Railway::new("FS"),
                Epoch::IV,
                Some(PassengerCarType::OpenCoach),
                Some(ServiceLevel::FirstClass),
                None,
                Some(String::from("bandiera")),
                Some(LengthOverBuffer::new(303)),
            )
        }

        fn new_freight_car() -> RollingStock {
            RollingStock::new_freight_car(
                String::from("Gbhs"),
                None,
                Railway::new("FS"),
                Epoch::V,
                Some(FreightCarType::SwingRoofWagon),
                None,
                Some(String::from("marrone")),
                Some(LengthOverBuffer::new(122)),
            )
        }

        fn new_locomotive_catalog_item() -> CatalogItem {
            CatalogItem::new(
                Brand::new("ACME"),
                ItemNumber::new("123456").unwrap(),
                String::from("My first catalog item"),
                vec![new_locomotive()],
                PowerMethod::DC,
                Scale::from_name("H0").unwrap(),
                1,
            )
        }

        fn new_passenger_cars_catalog_item() -> CatalogItem {
            CatalogItem::new(
                Brand::new("Roco"),
                ItemNumber::new("654321").unwrap(),
                String::from("My first catalog item"),
                vec![new_passenger_car(), new_passenger_car()],
                PowerMethod::DC,
                Scale::from_name("H0").unwrap(),
                2,
            )
        }

        fn new_set_catalog_item() -> CatalogItem {
            CatalogItem::new(
                Brand::new("ACME"),
                ItemNumber::new("123456").unwrap(),
                String::from("My first catalog item"),
                vec![
                    new_passenger_car(),
                    new_passenger_car(),
                    new_freight_car(),
                ],
                PowerMethod::DC,
                Scale::from_name("H0").unwrap(),
                2,
            )
        }

        #[test]
        fn it_should_create_new_catalog_items() {
            let item = CatalogItem::new(
                Brand::new("ACME"),
                ItemNumber::new("123456").unwrap(),
                String::from("My first catalog item"),
                vec![new_locomotive()],
                PowerMethod::DC,
                Scale::from_name("H0").unwrap(),
                1,
            );

            assert_eq!(&Brand::new("ACME"), item.brand());
            assert_eq!(&ItemNumber::new("123456").unwrap(), item.item_number());
            assert_eq!("My first catalog item", item.description());
            assert_eq!(&vec![new_locomotive()], item.rolling_stocks());
            assert_eq!(PowerMethod::DC, item.power_method());
            assert_eq!(&Scale::from_name("H0").unwrap(), item.scale());
            assert_eq!(1, item.count());
        }

        #[test]
        fn it_should_check_whether_catalog_item_is_a_locomotive() {
            let item = new_locomotive_catalog_item();
            assert!(true, item.is_locomotive());
        }

        #[test]
        fn it_should_extract_the_category_from_catalog_items() {
            let item1 = new_locomotive_catalog_item();
            let item2 = new_passenger_cars_catalog_item();

            assert_eq!(Category::Locomotives, item1.category());
            assert_eq!(Category::PassengerCars, item2.category());
        }

        #[test]
        fn it_should_produce_string_representations_from_catalog_items() {
            let item = new_locomotive_catalog_item();
            assert_eq!("ACME 123456 (L)", item.to_string());
        }

        #[test]
        fn it_should_check_whether_two_catalog_items_are_equal() {
            let item1 = new_locomotive_catalog_item();
            let item2 = new_locomotive_catalog_item();
            let item3 = new_passenger_cars_catalog_item();

            assert!(item1 == item2);
            assert!(item1 != item3);
        }
    }
}
