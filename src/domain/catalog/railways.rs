//! This module contains everything related to railways.
use std::fmt;

/// It represents a railway company, which is an entity that operates a railroad track or trains.
#[derive(Debug, PartialEq, Clone)]
pub struct Railway(String);

impl Railway {
    /// Creates a new railway with this name
    pub fn new(name: &str) -> Self {
        Railway(name.to_owned())
    }

    /// Returns the name for this railway
    pub fn name(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for Railway {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod railway_tests {
        use super::*;

        #[test]
        fn it_should_create_new_railways() {
            let b = Railway::new("FS");
            assert_eq!("FS", b.name());
        }

        #[test]
        fn it_should_display_brand_as_string() {
            let b = Railway::new("FS");
            assert_eq!("FS", b.to_string());
        }
    }
}
