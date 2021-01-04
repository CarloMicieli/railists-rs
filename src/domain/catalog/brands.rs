//! This module contains everything related to brands.
use std::fmt;

/// A model railways manufacturer.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Brand(String);

impl Brand {
    /// Creates a new brand with the given name.
    pub fn new(name: &str) -> Self {
        Brand(name.to_owned())
    }

    /// Returns this brand name
    pub fn name(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for Brand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod brand_tests {
        use super::*;

        #[test]
        fn it_should_create_new_brands() {
            let b = Brand::new("ACME");
            assert_eq!("ACME", b.name());
        }

        #[test]
        fn it_should_display_brand_as_string() {
            let b = Brand::new("ACME");
            assert_eq!("ACME", b.to_string());
        }
    }
}
