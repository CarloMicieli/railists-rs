#![allow(unused_imports)]
#![allow(dead_code)]

#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate prettytable;
#[macro_use]
extern crate anyhow;

mod cli;
mod data_source;
mod domain;
mod tables;

use data_source::DataSource;
use domain::collecting::{
    collections::{Collection, CollectionStats, Depot},
    wish_lists::{Priority, WishListBudget},
};
use tables::AsTable;

fn main() {
    pretty_env_logger::init();

    let matches = cli::get_matches();
    match matches.subcommand() {
        Some(("collection", cmd_args)) => match cmd_args.subcommand() {
            Some(("list", subc_args)) => {
                let filename = subc_args
                    .get_one::<String>("file")
                    .expect("collection file is required");

                let data_source = DataSource::new(filename);
                let c = data_source
                    .collection()
                    .expect("Unable to load collection");

                let table = c.to_table();
                table.printstd();
            }
            Some(("csv", subc_args)) => {
                let filename = subc_args
                    .get_one::<String>("file")
                    .expect("collection file is required");
                let output_filename = subc_args
                    .get_one::<String>("output-file")
                    .expect("Output file is required");

                let data_source = DataSource::new(filename);
                let c = data_source
                    .collection()
                    .expect("Unable to load collection");

                write_collection_as_csv(c, output_filename)
                    .expect("Error during csv export");
            }
            Some(("stats", subc_args)) => {
                let filename = subc_args
                    .get_one::<String>("file")
                    .expect("collection file is required");
                let data_source = DataSource::new(filename);
                let c = data_source
                    .collection()
                    .expect("Unable to load collection");

                let stats = CollectionStats::from_collection(&c);
                println!(
                    "Total value........... {:.2} EUR",
                    stats.total_value()
                );
                println!("Rolling stocks/sets... {}", stats.size());

                let table = stats.to_table();
                table.printstd();
            }
            Some(("depot", subc_args)) => {
                let filename = subc_args
                    .get_one::<String>("file")
                    .expect("collection file is required");
                let data_source = DataSource::new(filename);
                let c = data_source
                    .collection()
                    .expect("Unable to load collection");
                let depot = Depot::from_collection(&c);

                println!("{} locomotive(s)", depot.len());

                let table = depot.to_table();
                table.printstd();
            }
            _ => {}
        },
        Some(("wishlist", cmd_args)) => match cmd_args.subcommand() {
            Some(("list", subc_args)) => {
                let filename = subc_args
                    .get_one::<String>("file")
                    .expect("wishlist file is required");

                let data_source = DataSource::new(filename);
                let wish_list = data_source
                    .wish_list()
                    .expect("Unable to load the wishlist");

                let table = wish_list.to_table();
                table.printstd();
            }
            Some(("budget", subc_args)) => {
                let filename = subc_args
                    .get_one::<String>("file")
                    .expect("wishlist file is required");

                let data_source = DataSource::new(filename);
                let wish_list = data_source
                    .wish_list()
                    .expect("Unable to load the wishlist");

                let budget = WishListBudget::from_wish_list(&wish_list);

                println!(
                    "High...... {} EUR",
                    budget.by_priority(Priority::High)
                );
                println!(
                    "Normal.... {} EUR",
                    budget.by_priority(Priority::Normal)
                );
                println!(
                    "Low....... {} EUR",
                    budget.by_priority(Priority::Low)
                );
            }
            _ => {}
        },
        _ => {}
    }
}

fn write_collection_as_csv(
    collection: Collection,
    output_file: &str,
) -> anyhow::Result<()> {
    let mut wtr = csv::Writer::from_path(output_file)?;

    wtr.write_record([
        "Brand",
        "ItemNumber",
        "Category",
        "Description",
        "Epoch",
        "Shop",
        "Date",
        "Count",
        "Price",
    ])?;

    for it in collection.get_items().iter() {
        let catalog_item = it.catalog_item();
        let purchase = it.purchased_info();

        wtr.write_record([
            catalog_item.brand().name(),
            catalog_item.item_number().value(),
            &catalog_item.category().to_string(),
            catalog_item.description(),
            "", //catalog_item.epoch(),
            purchase.shop(),
            &purchase.purchased_date().format("%Y-%m-%d").to_string(),
            &catalog_item.count().to_string(),
            &purchase.price().to_string(),
        ])?;
    }

    wtr.flush()?;
    Ok(())
}
