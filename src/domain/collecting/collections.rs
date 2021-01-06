use crate::domain::catalog::{
    catalog_items::CatalogItem, rolling_stocks::RollingStock,
};
use crate::domain::catalog::{catalog_items::ItemNumber, categories::Category};

use chrono::{Datelike, NaiveDate, NaiveDateTime, Utc};
use prettytable::Table;
use rust_decimal::prelude::*;
use std::{cmp, collections::HashMap, fmt, ops, str};

use crate::domain::catalog::rolling_stocks::DccInterface;
use crate::domain::collecting::Price;

/// A railway models collections, a collection stores a description and the items.
/// Everything else the application is able to determine from the collection content
/// is calculated on the fly.
#[derive(Debug, PartialEq)]
pub struct Collection {
    description: String,
    version: u8,
    modified_date: NaiveDateTime,
    items: Vec<CollectionItem>,
}

impl Collection {
    pub fn new(
        description: &str,
        version: u8,
        modified_date: NaiveDateTime,
    ) -> Self {
        Collection {
            description: description.to_owned(),
            version,
            modified_date,
            items: Vec::new(),
        }
    }

    /// Creates an empty collection.
    pub fn create_empty(description: &str) -> Self {
        Collection {
            description: description.to_owned(),
            version: 1,
            modified_date: Utc::now().naive_local(),
            items: Vec::new(),
        }
    }

    pub fn add_item(
        &mut self,
        catalog_item: CatalogItem,
        purchased_info: PurchasedInfo,
    ) {
        let collection_item = CollectionItem::new(catalog_item, purchased_info);
        self.items.push(collection_item);
    }

    /// Updates the modification fields (version and modified_date) for this collection.
    pub fn set_modified(
        &mut self,
        new_version: u8,
        modified_date: NaiveDateTime,
    ) {
        self.version = new_version;
        self.modified_date = modified_date;
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn get_items(&self) -> &Vec<CollectionItem> {
        &self.items
    }

    pub fn get(&self, index: usize) -> Option<&CollectionItem> {
        self.items.get(index)
    }

    pub fn sort_items(&mut self) {
        self.items.sort();
    }

    fn bump_version(&mut self) {
        self.version += 1;
        self.modified_date = Utc::now().naive_local();
    }
}

impl fmt::Display for Collection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Collection\n- version: {},\n- size: {} items,\n- last modified: {}\nitems:{}",
            self.version,
            self.len(),
            self.modified_date,
            self.items
                .iter()
                .map(|it| format!("\n  - {}", it))
                .collect::<String>()
        )
    }
}

impl ops::Index<usize> for Collection {
    type Output = CollectionItem;

    fn index(&self, index: usize) -> &Self::Output {
        &self.items[index]
    }
}

impl ops::IndexMut<usize> for Collection {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.items[index]
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct PurchasedInfo {
    shop: String,
    purchased_date: NaiveDate,
    price: Price,
}

impl PurchasedInfo {
    pub fn new(shop: &str, purchased_date: NaiveDate, price: Price) -> Self {
        PurchasedInfo {
            shop: shop.to_owned(),
            purchased_date,
            price,
        }
    }

    pub fn price(&self) -> &Price {
        &self.price
    }

    pub fn shop(&self) -> &str {
        &self.shop
    }

    pub fn purchased_date(&self) -> &NaiveDate {
        &self.purchased_date
    }
}

impl fmt::Display for PurchasedInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "purchased at '{}' on {} for {}",
            self.shop, self.purchased_date, self.price
        )
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct CollectionItem {
    catalog_item: CatalogItem,
    purchased_at: PurchasedInfo,
}

impl cmp::PartialOrd for CollectionItem {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl cmp::Ord for CollectionItem {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.catalog_item().cmp(other.catalog_item())
    }
}

impl CollectionItem {
    pub fn new(catalog_item: CatalogItem, purchased_at: PurchasedInfo) -> Self {
        CollectionItem {
            catalog_item,
            purchased_at,
        }
    }

    pub fn catalog_item(&self) -> &CatalogItem {
        &self.catalog_item
    }

    pub fn purchased_info(&self) -> &PurchasedInfo {
        &self.purchased_at
    }

    pub fn rolling_stocks(&self) -> &Vec<RollingStock> {
        self.catalog_item.rolling_stocks()
    }

    pub fn price_info(&self) -> (&Price, i32) {
        (
            &self.purchased_at.price,
            self.purchased_at.purchased_date.year(),
        )
    }
}

impl fmt::Display for CollectionItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}, {}", self.catalog_item, self.purchased_at)
    }
}

#[derive(Debug, PartialEq)]
pub struct Depot {
    locomotives: Vec<DepotCard>,
}

impl Depot {
    pub fn new() -> Self {
        Depot {
            locomotives: Vec::new(),
        }
    }

    pub fn from_collection(collection: &Collection) -> Self {
        let mut depot = Depot::new();

        for item in collection.get_items() {
            depot.add_catalog_item(item.catalog_item());
        }

        depot.locomotives.sort();
        depot
    }

    pub fn locomotives(&self) -> &Vec<DepotCard> {
        &self.locomotives
    }

    pub fn len(&self) -> usize {
        self.locomotives.len()
    }

    fn add_catalog_item(&mut self, ci: &CatalogItem) {
        let locomotives =
            ci.rolling_stocks().iter().filter(|it| it.is_locomotive());
        for rs in locomotives {
            let card = DepotCard::new(
                &rs.class_name().unwrap_or_default(),
                &rs.road_number().unwrap_or_default(),
                rs.series(),
                rs.livery(),
                &ci.brand().name(),
                ci.item_number(),
                rs.with_decoder(),
                rs.dcc_interface(),
            );

            self.locomotives.push(card);
        }
    }
}

/// A depot card contains the basic info for a model locomotive.
#[derive(Debug)]
pub struct DepotCard {
    class_name: String,
    road_number: String,
    series: Option<String>,
    livery: Option<String>,
    brand: String,
    item_number: ItemNumber,
    with_decoder: bool,
    dcc_interface: Option<DccInterface>,
}

impl DepotCard {
    pub fn new(
        class_name: &str,
        road_number: &str,
        series: Option<&str>,
        livery: Option<&str>,
        brand: &str,
        item_number: &ItemNumber,
        with_decoder: bool,
        dcc_interface: Option<DccInterface>,
    ) -> Self {
        DepotCard {
            class_name: class_name.to_owned(),
            road_number: road_number.to_owned(),
            series: series.map(|s| s.to_owned()),
            livery: livery.map(|s| s.to_owned()),
            brand: brand.to_owned(),
            item_number: item_number.clone(),
            with_decoder,
            dcc_interface,
        }
    }

    pub fn class_name(&self) -> &str {
        &self.class_name
    }

    pub fn road_number(&self) -> &str {
        &self.road_number
    }

    pub fn series(&self) -> Option<String> {
        self.series.clone()
    }

    pub fn livery(&self) -> Option<String> {
        self.livery.clone()
    }

    pub fn brand(&self) -> &str {
        &self.brand
    }

    pub fn item_number(&self) -> &ItemNumber {
        &self.item_number
    }

    pub fn with_decoder(&self) -> bool {
        self.with_decoder
    }

    pub fn dcc_interface(&self) -> Option<DccInterface> {
        self.dcc_interface
    }
}

impl cmp::PartialEq for DepotCard {
    fn eq(&self, other: &Self) -> bool {
        self.road_number == other.road_number
            && self.class_name == other.class_name
    }
}

impl cmp::Eq for DepotCard {}

impl cmp::PartialOrd for DepotCard {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl cmp::Ord for DepotCard {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        let cmp1 = self.class_name.cmp(&other.class_name);
        if cmp1 == cmp::Ordering::Equal {
            return self.road_number.cmp(&other.road_number);
        }

        cmp1
    }
}

#[derive(Debug, PartialEq)]
pub struct CollectionStats {
    total_value: Decimal,
    size: usize,
    values_by_year: Vec<YearlyCollectionStats>,
    totals: StatisticsTotals,
}

impl CollectionStats {
    pub fn from_collection(collection: &Collection) -> Self {
        let mut output: HashMap<Year, YearlyCollectionStats> = HashMap::new();

        for item in collection.get_items() {
            let year = item.purchased_info().purchased_date().year();

            output
                .entry(year)
                .or_insert(YearlyCollectionStats::new_from_item(item))
                .sum(item);
        }

        let mut values: Vec<YearlyCollectionStats> =
            output.values().cloned().collect();
        values.sort();

        let mut totals = StatisticsTotals::new();
        for it in values.iter() {
            totals.add(it);
        }

        let size = collection.len();
        let total_value: Price = Price::euro(totals.total_value.clone());

        CollectionStats {
            total_value: total_value.amount,
            size,
            values_by_year: values,
            totals,
        }
    }

    /// The total value of this collection
    pub fn total_value(&self) -> Decimal {
        self.total_value
    }

    /// The number of items included in this collection.
    /// In case a catalog item contains more rolling stocks, all of them are accounted for.
    pub fn size(&self) -> usize {
        self.size
    }

    pub fn values_by_year(&self) -> &Vec<YearlyCollectionStats> {
        &self.values_by_year
    }

    pub fn number_of_locomotives(&self) -> u8 {
        self.totals.number_of_locomotives
    }

    pub fn locomotives_value(&self) -> Decimal {
        self.totals.locomotives_value
    }

    pub fn number_of_passenger_cars(&self) -> u8 {
        self.totals.number_of_passenger_cars
    }

    pub fn passenger_cars_value(&self) -> Decimal {
        self.totals.passenger_cars_value
    }

    pub fn number_of_freight_cars(&self) -> u8 {
        self.totals.number_of_freight_cars
    }

    pub fn freight_cars_value(&self) -> Decimal {
        self.totals.freight_cars_value
    }

    pub fn number_of_trains(&self) -> u8 {
        self.totals.number_of_trains
    }

    pub fn trains_value(&self) -> Decimal {
        self.totals.trains_value
    }

    pub fn number_of_rolling_stocks(&self) -> u16 {
        self.totals.number_of_rolling_stocks
    }
}

pub type Year = i32;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct YearlyCollectionStats {
    year: Year,
    locomotives: (u8, Decimal),
    passenger_cars: (u8, Decimal),
    freight_cars: (u8, Decimal),
    trains: (u8, Decimal),
    total: (u8, Decimal),
}

impl YearlyCollectionStats {
    pub fn new(year: Year) -> Self {
        let zero: Decimal = Decimal::from(0);

        YearlyCollectionStats {
            year,
            locomotives: (0u8, zero.clone()),
            passenger_cars: (0u8, zero.clone()),
            freight_cars: (0u8, zero.clone()),
            trains: (0u8, zero.clone()),
            total: (0u8, zero),
        }
    }

    pub fn new_from_item(item: &CollectionItem) -> YearlyCollectionStats {
        let year = item.purchased_info().purchased_date().year();
        let mut stat = Self::new(year);
        stat.sum(item);
        stat
    }

    pub fn sum(&mut self, item: &CollectionItem) {
        match item.catalog_item().category() {
            Category::FreightCars => self.add_freight_cars(item),
            Category::Locomotives => self.add_locomotives(item),
            Category::PassengerCars => self.add_passenger_cars(item),
            Category::Trains => self.add_trains(item),
        }
        self.update_total(item);
    }

    pub fn year(&self) -> Year {
        self.year
    }

    pub fn number_of_locomotives(&self) -> u8 {
        let (c, _) = self.locomotives;
        c
    }

    pub fn locomotives_value(&self) -> Decimal {
        let (_, v) = self.locomotives;
        v
    }

    pub fn number_of_passenger_cars(&self) -> u8 {
        let (c, _) = self.passenger_cars;
        c
    }

    pub fn passenger_cars_value(&self) -> Decimal {
        let (_, v) = self.passenger_cars;
        v
    }

    pub fn number_of_freight_cars(&self) -> u8 {
        let (c, _) = self.freight_cars;
        c
    }

    pub fn freight_cars_value(&self) -> Decimal {
        let (_, v) = self.freight_cars;
        v
    }

    pub fn number_of_trains(&self) -> u8 {
        let (c, _) = self.trains;
        c
    }

    pub fn trains_value(&self) -> Decimal {
        let (_, v) = self.trains;
        v
    }

    pub fn number_of_rolling_stocks(&self) -> u8 {
        let (c, _) = self.total;
        c
    }

    pub fn total_value(&self) -> Decimal {
        let (_, v) = self.total;
        v
    }

    fn add_locomotives(&mut self, item: &CollectionItem) {
        let (count, total_value) = &self.locomotives;
        self.locomotives = (
            count + item.catalog_item().count(),
            total_value + item.purchased_at.price().amount.clone(),
        );
    }

    fn add_passenger_cars(&mut self, item: &CollectionItem) {
        let (count, total_value) = &self.passenger_cars;
        self.passenger_cars = (
            count + item.catalog_item().count(),
            total_value + item.purchased_at.price().amount.clone(),
        );
    }

    fn add_freight_cars(&mut self, item: &CollectionItem) {
        let (count, total_value) = &self.freight_cars;
        self.freight_cars = (
            count + item.catalog_item().count(),
            total_value + item.purchased_at.price().amount.clone(),
        );
    }

    fn add_trains(&mut self, item: &CollectionItem) {
        let (count, total_value) = &self.trains;
        self.trains = (
            count + item.catalog_item().count(),
            total_value + item.purchased_at.price().amount.clone(),
        );
    }

    fn update_total(&mut self, item: &CollectionItem) {
        let (count, total_value) = &self.total;
        self.total = (
            count + item.catalog_item().count(),
            total_value + item.purchased_at.price().amount.clone(),
        );
    }
}

impl cmp::PartialOrd for YearlyCollectionStats {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl cmp::Ord for YearlyCollectionStats {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.year.cmp(&other.year)
    }
}

#[derive(Debug, PartialEq)]
pub struct StatisticsTotals {
    number_of_locomotives: u8,
    locomotives_value: Decimal,
    number_of_trains: u8,
    trains_value: Decimal,
    number_of_passenger_cars: u8,
    passenger_cars_value: Decimal,
    number_of_freight_cars: u8,
    freight_cars_value: Decimal,
    number_of_rolling_stocks: u16,
    total_value: Decimal,
}

impl StatisticsTotals {
    pub fn new() -> Self {
        StatisticsTotals {
            number_of_locomotives: 0u8,
            locomotives_value: Decimal::from(0),
            number_of_trains: 0u8,
            trains_value: Decimal::from(0),
            number_of_passenger_cars: 0u8,
            passenger_cars_value: Decimal::from(0),
            number_of_freight_cars: 0u8,
            freight_cars_value: Decimal::from(0),
            number_of_rolling_stocks: 0u16,
            total_value: Decimal::from(0),
        }
    }

    fn add(&mut self, yearly: &YearlyCollectionStats) {
        self.number_of_locomotives += yearly.number_of_locomotives();
        self.locomotives_value += yearly.locomotives_value();
        self.number_of_trains += yearly.number_of_trains();
        self.trains_value += yearly.trains_value();
        self.number_of_passenger_cars += yearly.number_of_passenger_cars();
        self.passenger_cars_value += yearly.passenger_cars_value();
        self.number_of_freight_cars += yearly.number_of_freight_cars();
        self.freight_cars_value += yearly.freight_cars_value();
        self.number_of_rolling_stocks +=
            yearly.number_of_rolling_stocks() as u16;
        self.total_value += yearly.total_value();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod collection_tests {
        use super::*;
    }
}
