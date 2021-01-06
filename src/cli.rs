use clap::{App, Arg, ArgMatches, SubCommand};

pub fn get_matches() -> ArgMatches<'static> {
    let file_arg = Arg::with_name("file")
        .short("f")
        .long("file")
        .takes_value(true)
        .required(true)
        .value_name("file name")
        .help("The file name (required)");

    let collection_ls_subcommand = SubCommand::with_name("list")
        .alias("l")
        .arg(file_arg.clone())
        .about("List the collection elements");

    let collection_stats_subcommand = SubCommand::with_name("stats")
        .alias("s")
        .arg(file_arg.clone())
        .about("Calculate the collection statistics");

    let collection_depot_subcommand = SubCommand::with_name("depot")
        .alias("d")
        .arg(file_arg.clone())
        .about("Extract the depot information for locomotives");

    let collection_csv_subcommand = SubCommand::with_name("csv")
        .alias("c")
        .arg(file_arg.clone())
        .arg(
            Arg::with_name("output-file")
                .short("o")
                .long("output")
                .takes_value(true)
                .required(true)
                .value_name("file name")
                .help("The output file name (required)"),
        )
        .about("Export the collection as csv file");

    let collection_subcommand = SubCommand::with_name("collection")
        .alias("c")
        .subcommand(collection_ls_subcommand)
        .subcommand(collection_csv_subcommand)
        .subcommand(collection_stats_subcommand)
        .subcommand(collection_depot_subcommand)
        .about("Manage model railway collections");

    let wishlist_ls_subcommand = SubCommand::with_name("list")
        .alias("l")
        .arg(file_arg.clone())
        .about("List the wishlist elements");

    let wishlist_budget_subcommand = SubCommand::with_name("budget")
        .alias("b")
        .arg(file_arg.clone())
        .about("Calculate the wishlist required budget");

    let wishlist_subcommand = SubCommand::with_name("wishlist")
        .alias("w")
        .subcommand(wishlist_ls_subcommand)
        .subcommand(wishlist_budget_subcommand)
        .about("Manage model railway wishlist");

    // let migrate_subcommand = SubCommand::with_name("migrate")
    //     .arg(
    //         Arg::with_name("file")
    //             .short("f")
    //             .long("file")
    //             .takes_value(true)
    //             .required(true)
    //             .value_name("file name")
    //             .help("The file name (required)"),
    //     )
    //     .about("Migrate yaml file");

    App::new("railists")
        .version(env!("CARGO_PKG_VERSION"))
        .about("Model railway collection manager")
        .author(env!("CARGO_PKG_AUTHORS"))
        .subcommand(collection_subcommand)
        .subcommand(wishlist_subcommand)
        .get_matches()
}
