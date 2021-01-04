use chrono::prelude::*;
use serde_yaml;
use std::fs;

use crate::domain::catalog::catalog_items::{CatalogItem, ItemNumber};
use crate::domain::catalog::categories::{
    FreightCarType, LocomotiveType, PassengerCarType, TrainType,
};
use crate::domain::catalog::railways::Railway;
use crate::domain::catalog::rolling_stocks::{
    Control, DccInterface, Epoch, LengthOverBuffer, RollingStock, ServiceLevel,
};
use crate::domain::catalog::{brands::Brand, catalog_items::PowerMethod};
use crate::domain::collecting::collections::{
    Collection, CollectionItem, PurchasedInfo,
};
use crate::domain::collecting::Price;

pub fn load_collection_from_file(
    filename: String,
) -> Result<Collection, String> {
    parse_yaml_collection(filename).and_then(|c| to_collection(c))
}

pub fn parse_yaml_collection(
    filename: String,
) -> Result<YamlCollection, String> {
    if let Ok(contents) = fs::read_to_string(filename) {
        let my_collection: YamlCollection =
            serde_yaml::from_str(&contents).expect("Error during deserialize");
        Ok(my_collection)
    } else {
        Err(String::from("Ops, something went wrong"))
    }
}

fn to_collection(c: YamlCollection) -> Result<Collection, String> {
    let mut collection_items: Vec<CollectionItem> = Vec::new();
    for elem in c.elements {
        let mut rolling_stocks: Vec<RollingStock> = Vec::new();

        for rs in elem.rolling_stocks {
            let length_over_buffer =
                rs.length.map(|l| LengthOverBuffer::new(l));
            let control = rs.control.and_then(|c| c.parse::<Control>().ok());
            let dcc_interface = rs
                .dcc_interface
                .and_then(|dcc| dcc.parse::<DccInterface>().ok());

            let epoch = rs.epoch.parse::<Epoch>()?;

            match rs.category.as_str() {
                "LOCOMOTIVE" => {
                    let locomotive = RollingStock::new_locomotive(
                        rs.type_name,
                        rs.road_number.unwrap_or_default(),
                        rs.series,
                        Railway::new(&rs.railway),
                        epoch,
                        rs.sub_category
                            .and_then(|c| c.parse::<LocomotiveType>().ok())
                            .unwrap(),
                        rs.depot,
                        rs.livery,
                        length_over_buffer,
                        control,
                        dcc_interface,
                    );

                    rolling_stocks.push(locomotive);
                }
                "TRAIN" => {
                    let train = RollingStock::new_train(
                        rs.type_name,
                        rs.road_number,
                        1,
                        Railway::new(&rs.railway),
                        epoch,
                        rs.sub_category
                            .and_then(|c| c.parse::<TrainType>().ok()),
                        rs.depot,
                        rs.livery,
                        length_over_buffer,
                        control,
                        dcc_interface,
                    );

                    rolling_stocks.push(train);
                }
                "PASSENGER_CAR" => {
                    let passenger_car = RollingStock::new_passenger_car(
                        rs.type_name,
                        rs.road_number,
                        Railway::new(&rs.railway),
                        epoch,
                        rs.sub_category
                            .and_then(|c| c.parse::<PassengerCarType>().ok()),
                        rs.service_level
                            .and_then(|sl| sl.parse::<ServiceLevel>().ok()),
                        rs.depot,
                        rs.livery,
                        length_over_buffer,
                    );

                    rolling_stocks.push(passenger_car);
                }
                "FREIGHT_CAR" => {
                    let freight_car = RollingStock::new_freight_car(
                        rs.type_name,
                        rs.road_number,
                        Railway::new(&rs.railway),
                        epoch,
                        rs.sub_category
                            .and_then(|c| c.parse::<FreightCarType>().ok()),
                        rs.depot,
                        rs.livery,
                        length_over_buffer,
                    );

                    rolling_stocks.push(freight_car);
                }
                _ => {}
            }
        }

        let catalog_item = CatalogItem::new(
            Brand::new(&elem.brand),
            ItemNumber::new(&elem.item_number).expect("Invalid item number"),
            elem.description,
            rolling_stocks,
            elem.power_method
                .parse::<PowerMethod>()
                .expect("Invalid power method"),
            elem.scale,
            elem.count,
        );

        let purchased_date =
            NaiveDate::parse_from_str(&elem.purchase_info.date, "%Y-%m-%d")
                .unwrap();

        let price = elem.purchase_info.price.parse::<Price>();

        let purchased_info = PurchasedInfo::new(
            &elem.purchase_info.shop,
            purchased_date,
            price.unwrap(),
        );

        let collection_item = CollectionItem::new(catalog_item, purchased_info);

        collection_items.push(collection_item);
    }

    let modified_date =
        NaiveDateTime::parse_from_str(&c.modified_at, "%Y-%m-%d %H:%M:%S")
            .unwrap();

    collection_items.sort();

    Ok(Collection::init_from_data(
        &c.description,
        c.version,
        modified_date,
        collection_items,
    ))
}

#[derive(Debug, Serialize, Deserialize)]
pub struct YamlCollection {
    pub version: u8,
    pub description: String,
    #[serde(rename = "modifiedAt")]
    pub modified_at: String,
    pub elements: Vec<YamlCollectionItem>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde()]
pub struct YamlCollectionItem {
    pub brand: String,
    #[serde(rename = "itemNumber")]
    pub item_number: String,
    pub description: String,
    #[serde(rename = "powerMethod")]
    pub power_method: String,
    pub scale: String,
    pub count: u8,
    #[serde(rename = "rollingStocks")]
    pub rolling_stocks: Vec<YamlRollingStock>,
    #[serde(rename = "purchaseInfo")]
    pub purchase_info: YamlPurchaseInfo,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct YamlPurchaseInfo {
    pub date: String,
    pub price: String,
    pub shop: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct YamlRollingStock {
    #[serde(rename = "typeName")]
    pub type_name: String,
    #[serde(rename = "roadNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub road_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub series: Option<String>,
    pub railway: String,
    pub epoch: String,
    #[serde(default)]
    pub category: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "subCategory")]
    pub sub_category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub depot: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub length: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub livery: Option<String>,
    #[serde(rename = "serviceLevel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_level: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control: Option<String>,
    #[serde(rename = "dccInterface")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dcc_interface: Option<String>,
}
