mod yaml_collections;
mod yaml_rolling_stocks;
mod yaml_wish_lists;

use crate::domain::collecting::{
    collections::Collection, wish_lists::WishList,
};
use serde_yaml;
use std::fs;
use yaml_collections::YamlCollection;
use yaml_wish_lists::YamlWishList;

#[derive(Debug)]
pub struct DataSource {
    filename: String,
}

impl DataSource {
    pub fn new(filename: &str) -> Self {
        DataSource {
            filename: filename.to_owned(),
        }
    }

    pub fn wish_list(&self) -> anyhow::Result<WishList> {
        let contents = fs::read_to_string(self.filename.clone())?;
        let yaml_wish_list: YamlWishList = serde_yaml::from_str(&contents)?;
        yaml_wish_list.to_wish_list()
    }

    pub fn collection(&self) -> anyhow::Result<Collection> {
        let contents = fs::read_to_string(self.filename.clone())?;
        let yaml_collection: YamlCollection = serde_yaml::from_str(&contents)?;
        yaml_collection.to_collection()
    }
}
