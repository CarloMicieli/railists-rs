use crate::domain::catalog::catalog_items::CatalogItem;

#[derive(Debug)]
pub struct WishList {
    name: String,
    version: u8,
    items: Vec<WishListItem>,
}

impl WishList {
    pub fn new(name: &str, version: u8) -> Self {
        WishList {
            name: name.to_owned(),
            version,
            items: Vec::new(),
        }
    }

    pub fn add_item(&mut self, catalog_item: CatalogItem) {
        let item = WishListItem { catalog_item };
        self.items.push(item);
    }

    pub fn get_items(&self) -> &Vec<WishListItem> {
        &self.items
    }

    pub fn sort_items(&mut self) {
        self.items.sort();
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct WishListItem {
    catalog_item: CatalogItem,
}

impl WishListItem {
    pub fn catalog_item(&self) -> &CatalogItem {
        &self.catalog_item
    }
}
