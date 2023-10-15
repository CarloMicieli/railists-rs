use crate::domain::{
    catalog::{
        brands::Brand,
        catalog_items::{CatalogItem, DeliveryDate, ItemNumber, PowerMethod},
        rolling_stocks::RollingStock,
        scales::Scale,
    },
    collecting::{
        wish_lists::{PriceInfo, Priority, WishList, WishListItem},
        Price,
    },
};
use std::convert::TryFrom;

use super::yaml_rolling_stocks::YamlRollingStock;

#[derive(Debug, Deserialize)]
pub struct YamlWishList {
    pub name: String,
    #[serde(rename = "modifiedAt")]
    pub modified_at: String,
    pub version: u8,
    pub elements: Vec<YamlWishListItem>,
}

#[derive(Debug, Deserialize)]
pub struct YamlWishListItem {
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
    pub priority: Option<String>,
    #[serde(rename = "rollingStocks")]
    pub rolling_stocks: Vec<YamlRollingStock>,
    #[serde(default = "Vec::new")]
    pub prices: Vec<YamlPrice>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct YamlPrice {
    pub shop: String,
    pub price: String,
}

impl std::convert::TryFrom<YamlWishList> for WishList {
    type Error = anyhow::Error;

    fn try_from(value: YamlWishList) -> Result<Self, Self::Error> {
        let mut wish_list = WishList::new(&value.name, value.version);

        for item in value.elements {
            let mut prices: Vec<PriceInfo> = Vec::new();

            for p in item.prices.iter() {
                let price = p.price.parse::<Price>().unwrap();
                let pi = PriceInfo::new(&p.shop, price);
                prices.push(pi);
            }

            let priority = if let Some(p) = item.priority.clone() {
                p.parse::<Priority>()?
            } else {
                Default::default()
            };
            let catalog_item = YamlWishList::parse_catalog_item(item)?;

            wish_list.add_item(catalog_item, priority, prices);
        }

        Ok(wish_list)
    }
}

impl YamlWishList {
    fn parse_catalog_item(
        elem: YamlWishListItem,
    ) -> anyhow::Result<CatalogItem> {
        let mut rolling_stocks: Vec<RollingStock> = Vec::new();
        for rs in elem.rolling_stocks {
            let rolling_stock = RollingStock::try_from(rs)?;
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
}
