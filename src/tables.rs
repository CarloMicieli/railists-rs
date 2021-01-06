use prettytable::{table, Table};
use rust_decimal::prelude::*;

use crate::domain::collecting::{
    collections::{
        Collection, CollectionStats, Depot, Year, YearlyCollectionStats,
    },
    wish_lists::WishList,
};

pub trait AsTable {
    fn to_table(self) -> Table;
}

impl AsTable for WishList {
    fn to_table(mut self) -> Table {
        self.sort_items();

        let mut table = Table::new();
        table.add_row(row![
            "#",
            "Brand",
            "Item number",
            "Cat.",
            "Priority",
            "Scale",
            "PM",
            "Description",
            "Count",
            "Price range",
        ]);

        for (ind, it) in self.get_items().iter().enumerate() {
            let ci = it.catalog_item();

            let price_range = if let Some((min, max)) = it.price_range() {
                format!("from {} to {}", min.price(), max.price())
            } else {
                String::from("-")
            };

            table.add_row(row![
                ind + 1,
                b -> ci.brand().name(),
                ci.item_number(),
                c -> ci.category(),
                c -> it.priority().to_string(),
                ci.scale(),
                ci.power_method(),
                i -> substring(ci.description()),
                r -> ci.count(),
                c -> price_range,
            ]);
        }

        table
    }
}

impl AsTable for Depot {
    fn to_table(self) -> Table {
        let mut table = Table::new();

        table.add_row(row![
            "#",
            "Class name",
            "Road number",
            "Series",
            "Livery",
            "Brand",
            "Item Number",
            "With decoder",
            "DCC",
        ]);

        for (id, card) in self.locomotives().iter().enumerate() {
            let with_dec = if card.with_decoder() { "Y" } else { "N" };

            table.add_row(row![
                c -> (id + 1).to_string(),
                b -> card.class_name().to_string(),
                card.road_number().to_string(),
                card.series().unwrap_or_default(),
                card.livery().unwrap_or_default(),
                card.brand().to_string(),
                card.item_number().to_string(),
                c -> with_dec.to_string(),
                c -> card.dcc_interface()
                    .map(|dcc| dcc.to_string())
                    .unwrap_or_default(),
            ]);
        }

        table
    }
}

impl AsTable for CollectionStats {
    fn to_table(self) -> Table {
        let mut table = Table::new();
        table.add_row(row![
            "Year",
            "Locomotives (no.)",
            "Locomotives (EUR)",
            "Trains (no.)",
            "Trains (EUR)",
            "Passenger Cars (no.)",
            "Passenger Cars (EUR)",
            "Freight Cars (no.)",
            "Freight Cars (EUR)",
            "Total (no.)",
            "Total (EUR)"
        ]);

        for s in self.values_by_year() {
            table.add_row(row![
                s.year().to_string(),
                r -> s.number_of_locomotives().to_string(),
                r -> s.locomotives_value().to_string(),
                r -> s.number_of_trains().to_string(),
                r -> s.trains_value().to_string(),
                r -> s.number_of_passenger_cars().to_string(),
                r -> s.passenger_cars_value().to_string(),
                r -> s.number_of_freight_cars().to_string(),
                r -> s.freight_cars_value().to_string(),
                r -> s.number_of_rolling_stocks().to_string(),
                r -> s.total_value().to_string(),
            ]);
        }

        table.add_row(row![
            "TOTAL",
            r -> self.number_of_locomotives().to_string(),
            r -> self.locomotives_value().to_string(),
            r -> self.number_of_trains().to_string(),
            r -> self.trains_value().to_string(),
            r -> self.number_of_passenger_cars().to_string(),
            r -> self.passenger_cars_value().to_string(),
            r -> self.number_of_freight_cars().to_string(),
            r -> self.freight_cars_value().to_string(),
            r -> self.number_of_rolling_stocks().to_string(),
            r -> self.total_value().to_string(),
        ]);

        table
    }
}

impl AsTable for Collection {
    fn to_table(mut self) -> Table {
        self.sort_items();

        let mut table = Table::new();
        table.add_row(row![
            "#",
            "Brand",
            "Item number",
            "Scale",
            "PM",
            "Cat.",
            "Description",
            "Count",
            "Added",
            "Price",
            "Shop"
        ]);

        for (ind, it) in self.get_items().iter().enumerate() {
            let ci = it.catalog_item();
            let purchase = it.purchased_info();

            table.add_row(row![
                ind + 1,
                b -> ci.brand().name(),
                ci.item_number(),
                ci.scale(),
                ci.power_method(),
                c -> ci.category(),
                i -> substring(ci.description()),
                r -> ci.count(),
                purchase.purchased_date().format("%Y-%m-%d").to_string(),
                r -> purchase.price(),
                purchase.shop(),
            ]);
        }

        table
    }
}

fn substring(s: &str) -> String {
    if s.len() < 50 {
        s.to_owned()
    } else {
        let mut output = s[0..47].to_owned();
        output.push_str("...");
        output
    }
}
