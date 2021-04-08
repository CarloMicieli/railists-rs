use chrono::{NaiveDate, NaiveDateTime};

use super::yaml_rolling_stocks::YamlRollingStock;
use crate::domain::{
    catalog::{
        brands::Brand,
        catalog_items::{CatalogItem, DeliveryDate, ItemNumber, PowerMethod},
        rolling_stocks::RollingStock,
        scales::Scale,
    },
    collecting::{
        collections::{Collection, PurchasedInfo},
        Price,
    },
};

#[derive(Debug, Deserialize)]
pub struct YamlCollection {
    pub version: u8,
    pub description: String,
    #[serde(rename = "modifiedAt")]
    pub modified_at: String,
    pub elements: Vec<YamlCollectionItem>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct YamlCollectionItem {
    pub brand: String,
    #[serde(rename = "itemNumber")]
    pub item_number: String,
    pub description: String,
    #[serde(rename = "powerMethod")]
    pub power_method: String,
    pub scale: String,
    #[serde(rename = "deliveryDate")]
    pub delivery_date: Option<String>,
    pub count: u8,
    #[serde(rename = "rollingStocks")]
    pub rolling_stocks: Vec<YamlRollingStock>,
    #[serde(rename = "purchaseInfo")]
    pub purchase_info: YamlPurchaseInfo,
}

#[derive(Debug, Deserialize, Clone)]
pub struct YamlPurchaseInfo {
    pub date: String,
    pub price: String,
    pub shop: String,
}

impl YamlCollection {
    pub fn to_collection(self) -> anyhow::Result<Collection> {
        let modified_date = NaiveDateTime::parse_from_str(
            &self.modified_at,
            "%Y-%m-%d %H:%M:%S",
        )
        .unwrap();

        let mut collection =
            Collection::new(&self.description, self.version, modified_date);

        for item in self.elements {
            let purchased_info =
                Self::parse_purchase_info(item.purchase_info.clone())?;
            let catalog_item = Self::parse_catalog_item(item)?;

            collection.add_item(catalog_item, purchased_info)
        }

        Ok(collection)
    }

    fn parse_catalog_item(
        elem: YamlCollectionItem,
    ) -> anyhow::Result<CatalogItem> {
        let mut rolling_stocks: Vec<RollingStock> = Vec::new();
        for rs in elem.rolling_stocks {
            let rolling_stock = rs.to_rolling_stock()?;
            rolling_stocks.push(rolling_stock);
        }

        let mut delivery_date = None;
        if let Some(dd) = elem.delivery_date {
            delivery_date = Some(dd.parse::<DeliveryDate>()?);
        }

        let catalog_item = CatalogItem::new(
            Brand::new(&elem.brand),
            ItemNumber::new(&elem.item_number).expect("Invalid item number"),
            elem.description,
            rolling_stocks,
            elem.power_method
                .parse::<PowerMethod>()
                .expect("Invalid power method"),
            Scale::from_name(&elem.scale).unwrap(),
            delivery_date,
            elem.count,
        );

        Ok(catalog_item)
    }

    fn parse_purchase_info(
        elem: YamlPurchaseInfo,
    ) -> anyhow::Result<PurchasedInfo> {
        let purchased_date =
            NaiveDate::parse_from_str(&elem.date, "%Y-%m-%d").unwrap();

        let price = elem.price.parse::<Price>();

        let purchased_info =
            PurchasedInfo::new(&elem.shop, purchased_date, price.unwrap());
        Ok(purchased_info)
    }
}
