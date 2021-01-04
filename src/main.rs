#![allow(unused_imports)]
#![allow(dead_code)]

#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate prettytable;

mod cli;
mod domain;
mod tables;
mod yaml;

use csv;
use domain::collecting::collections::{Collection, CollectionStats, Depot};
use tables::AsTable;

fn main() {
    pretty_env_logger::init();

    let matches = cli::get_matches();
    match matches.subcommand() {
        ("collection", Some(cmd_args)) => match cmd_args.subcommand() {
            ("list", Some(subc_args)) => {
                let filename = subc_args
                    .value_of("file")
                    .expect("collection file is required");
                let c =
                    crate::yaml::load_collection_from_file(filename.to_owned())
                        .expect("Unable to load collection");

                let table = c.to_table();
                table.printstd();
            }
            ("csv", Some(subc_args)) => {
                let filename = subc_args
                    .value_of("file")
                    .expect("collection file is required");
                let output_filename = subc_args
                    .value_of("output-file")
                    .expect("Output file is required");

                let c =
                    crate::yaml::load_collection_from_file(filename.to_owned())
                        .expect("Unable to load collection");

                write_collection_as_csv(c, output_filename)
                    .expect("Error during csv export");
            }
            ("stats", Some(subc_args)) => {
                let filename = subc_args
                    .value_of("file")
                    .expect("collection file is required");
                let c =
                    crate::yaml::load_collection_from_file(filename.to_owned())
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
            ("depot", Some(subc_args)) => {
                let filename = subc_args
                    .value_of("file")
                    .expect("collection file is required");
                let c =
                    crate::yaml::load_collection_from_file(filename.to_owned())
                        .expect("Unable to load collection");

                let depot = Depot::from_collection(&c);

                println!("{} locomotive(s)", depot.len());

                let table = depot.to_table();
                table.printstd();
            }
            _ => {}
        },
        // ("migrate", Some(cmd_args)) => {
        //     let filename = cmd_args
        //     .value_of("file")
        //     .expect("collection file is required");
        //     let c = crate::yaml::parse_yaml_collection(filename.to_owned())
        //         .expect("Unable to load collection");

        //     let items = c.elements.iter().map(|it| {
        //         YamlCollectionItem2 {
        //             brand: it.brand.to_owned(),
        //             item_number: it.item_number.to_owned(),
        //             power_method: "DC".to_owned(),
        //             count: it.count,
        //             scale: "H0".to_owned(),
        //             description: it.description.to_owned(),
        //             rolling_stocks: it.rolling_stocks.clone(),
        //             purchase_info: it.purchase_info.clone(),
        //         }
        //     });

        //     let c2 = YamlCollection2 {
        //         description: c.description,
        //         version: c.version,
        //         modified_at: c.modified_at,
        //         elements: items.collect::<Vec<YamlCollectionItem2>>(),
        //     };

        //     println!("{}", serde_yaml::to_string(&c2).unwrap());
        // }
        _ => {}
    }
}

fn write_collection_as_csv(
    collection: Collection,
    output_file: &str,
) -> anyhow::Result<()> {
    let mut wtr = csv::Writer::from_path(output_file)?;

    wtr.write_record(&[
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

        wtr.write_record(&[
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
