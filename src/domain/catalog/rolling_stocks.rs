use std::fmt;
use std::str;

use heck::ShoutySnakeCase;

use itertools::Itertools;
use thiserror::Error;

use crate::domain::catalog::categories::{
    Category, FreightCarType, LocomotiveType, PassengerCarType, TrainType,
};
use crate::domain::catalog::railways::Railway;

/// The model railway industry adopted an 'Era', or 'Epoch' system; the idea being to group models
/// into a defined time bracket, so that locomotives, coaching and wagon stock could be reasonably
/// grouped together.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
#[allow(non_snake_case)]
pub enum Epoch {
    I,
    II,
    IIa,
    IIb,
    III,
    IIIa,
    IIIb,
    IV,
    IVa,
    IVb,
    V,
    Va,
    Vb,
    Vm,
    VI,
    Multiple(Box<Epoch>, Box<Epoch>),
}

impl str::FromStr for Epoch {
    type Err = EpochParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err(EpochParseError::BlankValue);
        }

        if s.contains("/") {
            let tokens: Vec<&str> =
                s.split_terminator("/").sorted().dedup().collect();
            if tokens.len() == 2 {
                let first = Epoch::parse_str(tokens[0])?;
                let second = Epoch::parse_str(tokens[1])?;
                Ok(Epoch::Multiple(Box::new(first), Box::new(second)))
            } else {
                Err(EpochParseError::InvalidNumberOfValues)
            }
        } else {
            Epoch::parse_str(s)
        }
    }
}

#[derive(Error, Debug)]
pub enum EpochParseError {
    #[error("Epoch value cannot be blank")]
    BlankValue,
    #[error("Invalid number of elements for epoch values")]
    InvalidNumberOfValues,
    #[error("Invalid value for epoch")]
    InvalidValue,
}

impl Epoch {
    // Helper method to parse just the simple value
    fn parse_str(value: &str) -> Result<Self, EpochParseError> {
        match value {
            "I" => Ok(Epoch::I),
            "II" => Ok(Epoch::II),
            "IIa" => Ok(Epoch::IIa),
            "IIb" => Ok(Epoch::IIb),
            "III" => Ok(Epoch::III),
            "IIIa" => Ok(Epoch::IIIa),
            "IIIb" => Ok(Epoch::IIIb),
            "IV" => Ok(Epoch::IV),
            "IVa" => Ok(Epoch::IVa),
            "IVb" => Ok(Epoch::IVb),
            "V" => Ok(Epoch::V),
            "Va" => Ok(Epoch::Va),
            "Vb" => Ok(Epoch::Vb),
            "Vm" => Ok(Epoch::Vm),
            "VI" => Ok(Epoch::VI),
            _ => Err(EpochParseError::InvalidValue),
        }
    }
}

impl fmt::Display for Epoch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Epoch::Multiple(ep1, ep2) => write!(f, "{}/{}", &ep1, &ep2),
            _ => write!(f, "{:?}", self),
        }
    }
}

/// The control method for this railway model.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Control {
    /// The model can be fitted with a dcc decoder.
    DccReady,

    /// The model has a dcc decoder installed.
    Dcc,

    /// The model has a dcc decoder installed with the sound module.
    DccSound,
}

impl fmt::Display for Control {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = format!("{:?}", self);
        write!(f, "{}", s.to_shouty_snake_case())
    }
}

impl str::FromStr for Control {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err("Control value cannot be blank");
        }

        match s {
            "DCC_READY" => Ok(Control::DccReady),
            "DCC" => Ok(Control::Dcc),
            "DCC_SOUND" => Ok(Control::DccSound),
            _ => Err("Invalid value for control [allowed values are DCC, DCC_READY, DCC_SOUND]"),
        }
    }
}

/// The lenght over buffer for the model.
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct LengthOverBuffer(u32);

impl LengthOverBuffer {
    /// Creates a new value, the provided value must be positive.
    pub fn new(value: u32) -> Self {
        if value <= 0 {
            panic!("Length over buffer cannot be 0 or negative");
        }
        LengthOverBuffer(value)
    }
}

/// NMRA and NEM Connectors for digital control (DCC)
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum DccInterface {
    Nem651,
    Nem652,
    Plux8,
    Plux16,
    Plux22,
    Next18,
    Mtc21,
}

impl str::FromStr for DccInterface {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err("Dcc interface value cannot be blank");
        }

        match s {
            "NEM_651" => Ok(DccInterface::Nem651),
            "NEM_652" => Ok(DccInterface::Nem652),
            "PLUX_8" => Ok(DccInterface::Plux8),
            "PLUX_16" => Ok(DccInterface::Plux16),
            "PLUX_22" => Ok(DccInterface::Plux22),
            "NEXT_18" => Ok(DccInterface::Next18),
            "MTC_21" => Ok(DccInterface::Mtc21),
            _ => Err("Invalid value for dcc interfaces"),
        }
    }
}

impl fmt::Display for DccInterface {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = format!("{:?}", self);
        write!(f, "{}", s.to_shouty_snake_case())
    }
}

/// It represents the service level for a passenger cars, like first or second class.
/// Values of service level can also include multiple service levels, like mixed first
/// and second class.
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ServiceLevel {
    FirstClass,
    SecondClass,
    ThirdClass,
    FirstAndSecondClass,
    FirstSecondAndThirdClass,
    SecondAndThirdClass,
}

impl ServiceLevel {
    const FIRST_CLASS: &'static str = "1cl";
    const SECOND_CLASS: &'static str = "2cl";
    const THIRD_CLASS: &'static str = "3cl";
}

impl fmt::Display for ServiceLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ServiceLevel::FirstClass => {
                write!(f, "{}", ServiceLevel::FIRST_CLASS)
            }
            ServiceLevel::SecondClass => {
                write!(f, "{}", ServiceLevel::SECOND_CLASS)
            }
            ServiceLevel::ThirdClass => {
                write!(f, "{}", ServiceLevel::THIRD_CLASS)
            }
            ServiceLevel::FirstAndSecondClass => write!(
                f,
                "{}/{}",
                ServiceLevel::FIRST_CLASS,
                ServiceLevel::SECOND_CLASS
            ),
            ServiceLevel::FirstSecondAndThirdClass => write!(
                f,
                "{}/{}/{}",
                ServiceLevel::FIRST_CLASS,
                ServiceLevel::SECOND_CLASS,
                ServiceLevel::THIRD_CLASS
            ),
            ServiceLevel::SecondAndThirdClass => write!(
                f,
                "{}/{}",
                ServiceLevel::SECOND_CLASS,
                ServiceLevel::THIRD_CLASS
            ),
        }
    }
}

impl str::FromStr for ServiceLevel {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err("item number cannot be blank");
        }

        let service_level;
        if s.contains("/") {
            let tokens: Vec<&str> =
                s.split_terminator("/").sorted().dedup().collect();

            if tokens.len() == 2 {
                let first = tokens[0];
                let second = tokens[1];
                if first == ServiceLevel::FIRST_CLASS
                    && second == ServiceLevel::SECOND_CLASS
                {
                    service_level = ServiceLevel::FirstAndSecondClass;
                } else if first == ServiceLevel::SECOND_CLASS
                    && second == ServiceLevel::THIRD_CLASS
                {
                    service_level = ServiceLevel::SecondAndThirdClass;
                } else {
                    return Err("Invalid mixed service level");
                }
            } else if tokens.len() == 3 {
                let first = tokens[0];
                let second = tokens[1];
                let third = tokens[2];

                if first == ServiceLevel::FIRST_CLASS
                    && second == ServiceLevel::SECOND_CLASS
                    && third == ServiceLevel::THIRD_CLASS
                {
                    service_level = ServiceLevel::FirstSecondAndThirdClass;
                } else {
                    return Err("Invalid mixed service level");
                }
            } else {
                return Err(
                    "Invalid mixed service level: max number of values is 3",
                );
            }
        } else {
            service_level = match s {
                ServiceLevel::FIRST_CLASS => ServiceLevel::FirstClass,
                ServiceLevel::SECOND_CLASS => ServiceLevel::SecondClass,
                ServiceLevel::THIRD_CLASS => ServiceLevel::ThirdClass,
                _ => return Err("Wrong value for service level"),
            };
        }
        Ok(service_level)
    }
}

#[derive(Debug, PartialEq)]
pub enum RollingStock {
    Locomotive {
        class_name: String,
        road_number: String,
        series: Option<String>,
        railway: Railway,
        epoch: Epoch,
        category: LocomotiveType,
        depot: Option<String>,
        livery: Option<String>,
        length_over_buffer: Option<LengthOverBuffer>,
        control: Option<Control>,
        dcc_interface: Option<DccInterface>,
    },
    FreightCar {
        type_name: String,
        road_number: Option<String>,
        railway: Railway,
        epoch: Epoch,
        category: Option<FreightCarType>,
        depot: Option<String>,
        livery: Option<String>,
        length_over_buffer: Option<LengthOverBuffer>,
    },
    PassengerCar {
        type_name: String,
        road_number: Option<String>,
        railway: Railway,
        epoch: Epoch,
        category: Option<PassengerCarType>,
        service_level: Option<ServiceLevel>,
        depot: Option<String>,
        livery: Option<String>,
        length_over_buffer: Option<LengthOverBuffer>,
    },
    Train {
        type_name: String,
        road_number: Option<String>,
        n_of_elements: u8,
        railway: Railway,
        epoch: Epoch,
        category: Option<TrainType>,
        depot: Option<String>,
        livery: Option<String>,
        length_over_buffer: Option<LengthOverBuffer>,
        control: Option<Control>,
        dcc_interface: Option<DccInterface>,
    },
}

impl RollingStock {
    pub fn depot(&self) -> Option<&str> {
        match self {
            RollingStock::Locomotive {
                depot: Some(depot), ..
            } => Some(depot),
            _ => None,
        }
    }

    pub fn class_name(&self) -> Option<&str> {
        match self {
            RollingStock::Locomotive { class_name, .. } => Some(class_name),
            _ => None,
        }
    }

    pub fn road_number(&self) -> Option<&str> {
        match self {
            RollingStock::Locomotive { road_number, .. } => Some(road_number),
            _ => None,
        }
    }

    pub fn series(&self) -> Option<&str> {
        match self {
            RollingStock::Locomotive {
                series: Some(series),
                ..
            } => Some(series),
            _ => None,
        }
    }

    pub fn livery(&self) -> Option<&str> {
        match self {
            RollingStock::Locomotive {
                livery: Some(livery),
                ..
            } => Some(livery),
            _ => None,
        }
    }

    /// Returns the category for this rolling stock
    pub fn category(&self) -> Category {
        match self {
            RollingStock::Locomotive { .. } => Category::Locomotives,
            RollingStock::FreightCar { .. } => Category::FreightCars,
            RollingStock::PassengerCar { .. } => Category::PassengerCars,
            RollingStock::Train { .. } => Category::Trains,
        }
    }

    // pub fn epoch(&self) -> Epoch {
    //     match &self {
    //         RollingStock::Locomotive { epoch, .. } => *epoch.clone(),
    //         RollingStock::FreightCar { epoch, .. } => *epoch.clone(),
    //         RollingStock::PassengerCar { epoch, .. } => *epoch.clone(),
    //         RollingStock::Train { epoch, .. } => *epoch.clone(),
    //     }
    // }

    pub fn is_locomotive(&self) -> bool {
        self.category() == Category::Locomotives
    }

    pub fn with_decoder(&self) -> bool {
        match self {
            RollingStock::Locomotive {
                control: Some(control),
                ..
            } => *control != Control::DccReady,
            RollingStock::Train {
                control: Some(control),
                ..
            } => *control != Control::DccReady,
            _ => false,
        }
    }

    pub fn dcc_interface(&self) -> Option<DccInterface> {
        match self {
            RollingStock::Locomotive {
                dcc_interface: Some(dcc_interface),
                ..
            } => Some(*dcc_interface),
            RollingStock::Train {
                dcc_interface: Some(dcc_interface),
                ..
            } => Some(*dcc_interface),
            _ => None,
        }
    }

    /// Creates a new freight car rolling stock
    pub fn new_freight_car(
        type_name: String,
        road_number: Option<String>,
        railway: Railway,
        epoch: Epoch,
        category: Option<FreightCarType>,
        depot: Option<String>,
        livery: Option<String>,
        length_over_buffer: Option<LengthOverBuffer>,
    ) -> Self {
        RollingStock::FreightCar {
            type_name,
            road_number,
            railway,
            epoch,
            category,
            depot,
            livery,
            length_over_buffer,
        }
    }

    /// Creates a new train rolling stock
    pub fn new_train(
        type_name: String,
        road_number: Option<String>,
        n_of_elements: u8,
        railway: Railway,
        epoch: Epoch,
        category: Option<TrainType>,
        depot: Option<String>,
        livery: Option<String>,
        length_over_buffer: Option<LengthOverBuffer>,
        control: Option<Control>,
        dcc_interface: Option<DccInterface>,
    ) -> Self {
        RollingStock::Train {
            type_name,
            road_number,
            n_of_elements,
            railway,
            epoch,
            category,
            depot,
            livery,
            length_over_buffer,
            control,
            dcc_interface,
        }
    }

    /// Creates a new locomotive rolling stock
    pub fn new_locomotive(
        class_name: String,
        road_number: String,
        series: Option<String>,
        railway: Railway,
        epoch: Epoch,
        category: LocomotiveType,
        depot: Option<String>,
        livery: Option<String>,
        length_over_buffer: Option<LengthOverBuffer>,
        control: Option<Control>,
        dcc_interface: Option<DccInterface>,
    ) -> Self {
        RollingStock::Locomotive {
            class_name,
            road_number,
            series,
            railway,
            epoch,
            category,
            depot,
            livery,
            length_over_buffer,
            control,
            dcc_interface,
        }
    }

    /// Creates a new passenger car rolling stock
    pub fn new_passenger_car(
        type_name: String,
        road_number: Option<String>,
        railway: Railway,
        epoch: Epoch,
        category: Option<PassengerCarType>,
        service_level: Option<ServiceLevel>,
        depot: Option<String>,
        livery: Option<String>,
        length_over_buffer: Option<LengthOverBuffer>,
    ) -> Self {
        RollingStock::PassengerCar {
            type_name,
            road_number,
            railway,
            epoch,
            category,
            service_level,
            depot,
            livery,
            length_over_buffer,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod dcc_interface_tests {
        use super::*;

        #[test]
        fn it_should_parse_string_as_dcc_interfaces() {
            let dcc = "NEM_652".parse::<DccInterface>();
            assert!(dcc.is_ok());
            assert_eq!(dcc.unwrap(), DccInterface::Nem652);
        }

        #[test]
        fn it_should_fail_to_parse_invalid_string_as_dcc_interfaces() {
            let blank = "".parse::<DccInterface>();
            assert!(blank.is_err());

            let invalid = "invalid".parse::<DccInterface>();
            assert!(invalid.is_err());
        }

        #[test]
        fn it_should_display_dcc_interfaces() {
            let dcc = DccInterface::Nem652;
            assert_eq!("NEM652", dcc.to_string());
        }
    }

    mod epoch_tests {
        use super::*;

        #[test]
        fn it_should_convert_string_slices_to_epochs() {
            let epoch = "I".parse::<Epoch>();
            assert!(epoch.is_ok());
            assert_eq!(epoch.unwrap(), Epoch::I);
        }

        #[test]
        fn it_should_convert_string_slices_to_mixed_epochs() {
            let epoch = "I/II".parse::<Epoch>();
            assert!(epoch.is_ok());
            assert_eq!(
                epoch.unwrap(),
                Epoch::Multiple(Box::new(Epoch::I), Box::new(Epoch::II))
            );
        }

        #[test]
        fn it_should_fail_to_convert_invalid_values_to_epochs() {
            let empty_epoch = "".parse::<Epoch>();
            assert!(empty_epoch.is_err());

            let invalid_epoch = "invalid".parse::<Epoch>();
            assert!(invalid_epoch.is_err());
        }

        #[test]
        #[allow(non_snake_case)]
        fn it_should_diplay_epoch_values() {
            let epoch_I_II =
                Epoch::Multiple(Box::new(Epoch::I), Box::new(Epoch::II));
            let epoch_IVa = Epoch::IVa;

            assert_eq!("I/II", epoch_I_II.to_string());
            assert_eq!("IVa", epoch_IVa.to_string());
        }
    }

    mod control_tests {
        use super::*;

        #[test]
        fn it_should_parse_string_as_controls() {
            let c = "DCC_READY".parse::<Control>();
            assert!(c.is_ok());
            assert_eq!(c.unwrap(), Control::DccReady);
        }

        #[test]
        fn it_should_fail_to_parse_invalid_value_as_controls() {
            let blank = "".parse::<Control>();
            assert!(blank.is_err());

            let invalid = "invalid".parse::<Control>();
            assert!(invalid.is_err());
        }

        #[test]
        fn it_should_display_controls() {
            let c = Control::DccReady;
            assert_eq!("DCC_READY", c.to_string());
        }
    }

    mod rolling_stock_tests {
        use super::*;

        #[test]
        fn it_should_create_new_locomotives() {
            let railway_fs = Railway::new("FS");

            let rs = RollingStock::new_locomotive(
                String::from("E.656"),
                String::from("E.656 210"),
                Some(String::from("1a serie")),
                railway_fs.clone(),
                Epoch::IV,
                LocomotiveType::ElectricLocomotive,
                Some(String::from("Milano Centrale")),
                Some(String::from("blu/grigio")),
                Some(LengthOverBuffer::new(210)),
                Some(Control::DccReady),
                Some(DccInterface::Nem652),
            );

            match rs {
                RollingStock::Locomotive {
                    class_name,
                    road_number,
                    series,
                    railway,
                    epoch,
                    category,
                    depot,
                    livery,
                    length_over_buffer,
                    control,
                    dcc_interface,
                    ..
                } => {
                    assert_eq!(class_name, String::from("E.656"));
                    assert_eq!(road_number, String::from("E.656 210"));
                    assert_eq!(series, Some(String::from("1a serie")));
                    assert_eq!(railway, railway_fs);
                    assert_eq!(epoch, Epoch::IV);
                    assert_eq!(category, LocomotiveType::ElectricLocomotive);
                    assert_eq!(depot, Some(String::from("Milano Centrale")));
                    assert_eq!(livery, Some(String::from("blu/grigio")));
                    assert_eq!(
                        length_over_buffer,
                        Some(LengthOverBuffer::new(210))
                    );
                    assert_eq!(control, Some(Control::DccReady));
                    assert_eq!(dcc_interface, Some(DccInterface::Nem652));
                }
                _ => panic!(
                    "Invalid rolling stock type - expect a locomotive here!!!!"
                ),
            }
        }

        #[test]
        fn it_should_create_new_trains() {
            let railway_fs = Railway::new("FS");

            let rs = RollingStock::new_train(
                String::from("Etr 220"),
                None,
                4,
                railway_fs.clone(),
                Epoch::IV,
                Some(TrainType::ElectricMultipleUnits),
                Some(String::from("Milano Centrale")),
                Some(String::from("grigio nebbia/verde magnolia")),
                Some(LengthOverBuffer::new(800)),
                Some(Control::DccReady),
                Some(DccInterface::Nem652),
            );

            match rs {
                RollingStock::Train {
                    type_name,
                    road_number,
                    n_of_elements,
                    railway,
                    epoch,
                    category,
                    depot,
                    livery,
                    length_over_buffer,
                    control,
                    dcc_interface,
                    ..
                } => {
                    assert_eq!(type_name, String::from("Etr 220"));
                    assert_eq!(road_number, None);
                    assert_eq!(n_of_elements, 4);
                    assert_eq!(railway, railway_fs);
                    assert_eq!(epoch, Epoch::IV);
                    assert_eq!(
                        category,
                        Some(TrainType::ElectricMultipleUnits)
                    );
                    assert_eq!(depot, Some(String::from("Milano Centrale")));
                    assert_eq!(
                        livery,
                        Some(String::from("grigio nebbia/verde magnolia"))
                    );
                    assert_eq!(
                        length_over_buffer,
                        Some(LengthOverBuffer::new(800))
                    );
                    assert_eq!(control, Some(Control::DccReady));
                    assert_eq!(dcc_interface, Some(DccInterface::Nem652));
                }
                _ => panic!(
                    "Invalid rolling stock type - expect a train here!!!!"
                ),
            }
        }

        #[test]
        fn it_should_create_new_passenger_cars() {
            let railway_fs = Railway::new("FS");

            let rs = RollingStock::new_passenger_car(
                String::from("UIC-Z"),
                None,
                railway_fs.clone(),
                Epoch::IV,
                Some(PassengerCarType::OpenCoach),
                Some(ServiceLevel::FirstClass),
                None,
                Some(String::from("bandiera")),
                Some(LengthOverBuffer::new(303)),
            );

            match rs {
                RollingStock::PassengerCar {
                    type_name,
                    road_number,
                    railway,
                    epoch,
                    category,
                    depot,
                    livery,
                    length_over_buffer,
                    service_level,
                    ..
                } => {
                    assert_eq!(type_name, String::from("UIC-Z"));
                    assert_eq!(road_number, None);
                    assert_eq!(service_level, Some(ServiceLevel::FirstClass));
                    assert_eq!(railway, railway_fs);
                    assert_eq!(epoch, Epoch::IV);
                    assert_eq!(None, depot);
                    assert_eq!(category, Some(PassengerCarType::OpenCoach));
                    assert_eq!(livery, Some(String::from("bandiera")));
                    assert_eq!(length_over_buffer, Some(LengthOverBuffer::new(303)));
                }
                _ => panic!("Invalid rolling stock type - expect a passenger car here!!!!"),
            }
        }

        #[test]
        fn it_should_create_new_freight_cars() {
            let railway_fs = Railway::new("FS");

            let rs = RollingStock::new_freight_car(
                String::from("Gbhs"),
                None,
                railway_fs.clone(),
                Epoch::V,
                Some(FreightCarType::SwingRoofWagon),
                None,
                Some(String::from("marrone")),
                Some(LengthOverBuffer::new(122)),
            );

            match rs {
                RollingStock::FreightCar {
                    type_name,
                    road_number,
                    railway,
                    epoch,
                    category,
                    depot,
                    livery,
                    length_over_buffer,
                    ..
                } => {
                    assert_eq!(type_name, String::from("Gbhs"));
                    assert_eq!(road_number, None);
                    assert_eq!(railway, railway_fs);
                    assert_eq!(epoch, Epoch::V);
                    assert_eq!(None, depot);
                    assert_eq!(category, Some(FreightCarType::SwingRoofWagon));
                    assert_eq!(livery, Some(String::from("marrone")));
                    assert_eq!(length_over_buffer, Some(LengthOverBuffer::new(122)));
                }
                _ => panic!("Invalid rolling stock type - expect a freight car here!!!!"),
            }
        }
    }

    mod service_level_tests {
        use super::*;

        #[test]
        fn it_should_convert_string_slices_to_service_levels() {
            let service_level = "1cl".parse::<ServiceLevel>();
            assert!(service_level.is_ok());
            assert_eq!(service_level.unwrap(), ServiceLevel::FirstClass);
        }

        #[test]
        fn it_should_convert_string_slices_to_mixed_service_levels() {
            let service_level = "1cl/2cl/3cl/2cl".parse::<ServiceLevel>();
            assert!(service_level.is_ok());
            assert_eq!(
                service_level.unwrap(),
                ServiceLevel::FirstSecondAndThirdClass
            );
        }

        #[test]
        fn it_should_fail_to_convert_invalid_values_to_service_levels() {
            let empty_string = "".parse::<ServiceLevel>();
            assert!(empty_string.is_err());

            let invalid_value = "aaaa".parse::<ServiceLevel>();
            assert!(invalid_value.is_err());
        }

        #[test]
        fn it_should_fail_to_convert_string_slices_to_mixed_service_levels() {
            let wrong = "1cl/2cl/4cl/2cl".parse::<ServiceLevel>();
            assert!(wrong.is_err());
        }

        #[test]
        fn it_should_display_service_level_values() {
            assert_eq!("1cl", format!("{}", ServiceLevel::FirstClass));
            assert_eq!(
                "1cl/2cl",
                format!("{}", ServiceLevel::FirstAndSecondClass)
            );
        }
    }
}
