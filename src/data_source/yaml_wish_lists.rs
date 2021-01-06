use crate::domain::{
    catalog::{
        brands::Brand,
        catalog_items::{CatalogItem, DeliveryDate, ItemNumber, PowerMethod},
        rolling_stocks::RollingStock,
        scales::Scale,
    },
    collecting::wish_lists::{WishList, WishListItem},
};

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
    #[serde(rename = "rollingStocks")]
    pub rolling_stocks: Vec<YamlRollingStock>,
}

impl YamlWishList {
    pub fn to_wish_list(self) -> anyhow::Result<WishList> {
        let mut wish_list = WishList::new(&self.name, self.version);

        for item in self.elements {
            let catalog_item = Self::parse_catalog_item(item)?;
            wish_list.add_item(catalog_item);
        }

        Ok(wish_list)
    }

    fn parse_catalog_item(
        elem: YamlWishListItem,
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
}
