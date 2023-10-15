use std::fmt;
use std::str;

use heck::ToShoutySnakeCase;

/// The enumeration of the model categories.
#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Clone, Copy)]
pub enum Category {
    /// The steam locomotives category
    Locomotives,

    /// The trains category
    Trains,

    /// The freight cars category
    FreightCars,

    /// The passenger cars category
    PassengerCars,
}

impl Category {
    const LOCOMOTIVE_SYMBOL: char = 'L';
    const PASSENGER_CAR_SYMBOL: char = 'P';
    const FREIGHT_CAR_SYMBOL: char = 'F';
    const TRAIN_SYMBOL: char = 'T';

    /// Returns a symbol (just a single char) to represent the current category.
    pub fn symbol(&self) -> char {
        match &self {
            Category::Locomotives => Category::LOCOMOTIVE_SYMBOL,
            Category::FreightCars => Category::FREIGHT_CAR_SYMBOL,
            Category::PassengerCars => Category::PASSENGER_CAR_SYMBOL,
            Category::Trains => Category::TRAIN_SYMBOL,
        }
    }
}

impl fmt::Display for Category {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.symbol())
    }
}

/// The different kind of freight cars
#[derive(Debug, PartialEq)]
pub enum FreightCarType {
    AutoTransportCars,
    BrakeWagon,
    ContainerCars,
    CoveredFreightCars,
    DumpCars,
    Gondola,
    HeavyGoodsWagons,
    HingedCoverWagons,
    HopperWagon,
    RefrigeratorCars,
    SiloContainerCars,
    SlideTarpaulinWagon,
    SlidingWallBoxcars,
    SpecialTransport,
    StakeWagons,
    SwingRoofWagon,
    TankCars,
    TelescopeHoodWagons,
    DeepWellFlatCars,
}

impl str::FromStr for FreightCarType {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err("Freight car type value cannot be blank");
        }

        match s {
            "AUTO_TRANSPORT_CARS" => Ok(FreightCarType::AutoTransportCars),
            "BRAKE_WAGON" => Ok(FreightCarType::BrakeWagon),
            "CONTAINER_CARS" => Ok(FreightCarType::ContainerCars),
            "COVERED_FREIGHT_CARS" => Ok(FreightCarType::CoveredFreightCars),
            "DUMP_CARS" => Ok(FreightCarType::DumpCars),
            "GONDOLA" => Ok(FreightCarType::Gondola),
            "HEAVY_GOODS_WAGONS" => Ok(FreightCarType::HeavyGoodsWagons),
            "HINGED_COVER_WAGONS" => Ok(FreightCarType::HingedCoverWagons),
            "HOPPER_WAGON" => Ok(FreightCarType::HopperWagon),
            "REFRIGERATOR_CARS" => Ok(FreightCarType::RefrigeratorCars),
            "SILO_CONTAINER_CARS" => Ok(FreightCarType::SiloContainerCars),
            "SLIDE_TARPAULIN_WAGON" => Ok(FreightCarType::SlideTarpaulinWagon),
            "SLIDING_WALL_BOXCARS" => Ok(FreightCarType::SlidingWallBoxcars),
            "SPECIAL_TRANSPORT" => Ok(FreightCarType::SpecialTransport),
            "STAKE_WAGONS" => Ok(FreightCarType::StakeWagons),
            "SWING_ROOF_WAGON" => Ok(FreightCarType::SwingRoofWagon),
            "TANK_CARS" => Ok(FreightCarType::TankCars),
            "TELESCOPE_HOOD_WAGONS" => Ok(FreightCarType::TelescopeHoodWagons),
            "DEEP_WELL_FLAT_CARS" => Ok(FreightCarType::DeepWellFlatCars),
            _ => Err("Invalid value for freight car type"),
        }
    }
}

/// The different kinds of locomotives
#[derive(Debug, PartialEq)]
#[allow(clippy::enum_variant_names)]
pub enum LocomotiveType {
    /// The steam locomotives category
    SteamLocomotive,

    /// The diesel locomotives category
    DieselLocomotive,

    /// The electric locomotives category
    ElectricLocomotive,
}

impl str::FromStr for LocomotiveType {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err("Locomotive type value cannot be blank");
        }

        match s {
            "ELECTRIC_LOCOMOTIVE" => Ok(LocomotiveType::ElectricLocomotive),
            "DIESEL_LOCOMOTIVE" => Ok(LocomotiveType::DieselLocomotive),
            "STEAM_LOCOMOTIVE" => Ok(LocomotiveType::SteamLocomotive),
            _ => Err("Invalid value for locomotive type"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum PassengerCarType {
    /// An "open coach" has a central aisle; the car's interior is often filled with row upon row of
    /// seats as in a passenger airliner.
    OpenCoach,

    /// "closed" coaches or "compartment" cars have a side corridor to connect individual compartments
    /// along the body of the train, each with two rows of seats facing each other.
    CompartmentCoach,

    /// A dining car (or diner) is used to serve meals to the passengers.
    DiningCar,

    /// Lounge cars carry a bar and public seating.
    Lounge,

    /// The observation car almost always operated as the last car in a passenger train, in US
    /// practice. Its interior could include features of a coach, lounge, diner, or sleeper. The
    /// main spotting feature was at the tail end of the car.
    Observation,

    ///Often called "sleepers" or "Pullman cars", these cars provide sleeping arrangements for
    ///passengers travelling at night. Early models were divided into sections, where coach
    /// seating converted at night into semi-private berths.
    SleepingCar,

    /// The baggage car is a car that was normally placed between the train's motive power and the
    /// remainder of the passenger train. The car's interior is normally wide open and is used to
    /// carry passengers' checked baggage.
    BaggageCar,

    DoubleDecker,

    CombineCar,

    DrivingTrailer,

    RailwayPostOffice,
}

impl str::FromStr for PassengerCarType {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err("Passenger car type value cannot be blank");
        }

        match s {
            "OPEN_COACH" => Ok(PassengerCarType::OpenCoach),
            "COMPARTMENT_COACH" => Ok(PassengerCarType::CompartmentCoach),
            "DINING_CAR" => Ok(PassengerCarType::DiningCar),
            "LOUNGE" => Ok(PassengerCarType::Lounge),
            "OBSERVATION" => Ok(PassengerCarType::Observation),
            "SLEEPING_CAR" => Ok(PassengerCarType::SleepingCar),
            "BAGGAGE_CAR" => Ok(PassengerCarType::BaggageCar),
            "DOUBLE_DECKER" => Ok(PassengerCarType::DoubleDecker),
            "COMBINE_CAR" => Ok(PassengerCarType::CombineCar),
            "DRIVING_TRAILER" => Ok(PassengerCarType::DrivingTrailer),
            "RAILWAY_POST_OFFICE" => Ok(PassengerCarType::RailwayPostOffice),
            _ => Err("Invalid value for passenger car type"),
        }
    }
}

impl fmt::Display for PassengerCarType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = format!("{:?}", self);
        write!(f, "{:?}", s.to_shouty_snake_case())
    }
}

/// The different kind of trains
#[derive(Debug, PartialEq)]
pub enum TrainType {
    /// The railcar category
    Railcars,

    /// The electric multiple unit category
    ElectricMultipleUnits,

    /// The train set category
    TrainSets,

    /// The starter sets (usually includes the tracks) category
    StarterSets,
}

impl str::FromStr for TrainType {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err("Train type value cannot be blank");
        }

        match s {
            "RAILCARS" => Ok(TrainType::Railcars),
            "ELECTRIC_MULTIPLE_UNITS" => Ok(TrainType::ElectricMultipleUnits),
            "TRAIN_SETS" => Ok(TrainType::TrainSets),
            "STARTER_SETS" => Ok(TrainType::StarterSets),
            _ => Err("Invalid value for train type"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod freight_car_type_tests {
        use super::*;

        #[test]
        fn it_should_convert_string_slices_to_freight_car_types() {
            let locomotive_type =
                "HEAVY_GOODS_WAGONS".parse::<FreightCarType>();
            assert!(locomotive_type.is_ok());
            assert_eq!(
                locomotive_type.unwrap(),
                FreightCarType::HeavyGoodsWagons
            );
        }

        #[test]
        fn it_should_fail_to_convert_invalid_values_to_freight_car_types() {
            let blank_value = "".parse::<FreightCarType>();
            assert!(blank_value.is_err());

            let invalid_value = "invalid value".parse::<FreightCarType>();
            assert!(invalid_value.is_err());
        }
    }

    mod train_type_tests {
        use super::*;

        #[test]
        fn it_should_convert_string_slices_to_train_types() {
            let train_type = "ELECTRIC_MULTIPLE_UNITS".parse::<TrainType>();
            assert!(train_type.is_ok());
            assert_eq!(train_type.unwrap(), TrainType::ElectricMultipleUnits);
        }

        #[test]
        fn it_should_fail_to_convert_invalid_values_to_train_types() {
            let blank_value = "".parse::<TrainType>();
            assert!(blank_value.is_err());

            let invalid_value = "invalid value".parse::<TrainType>();
            assert!(invalid_value.is_err());
        }
    }

    mod passenger_car_type_tests {
        use super::*;

        #[test]
        fn it_should_convert_string_slices_to_passenger_car_types() {
            let passenger_car_type =
                "COMPARTMENT_COACH".parse::<PassengerCarType>();
            assert!(passenger_car_type.is_ok());
            assert_eq!(
                passenger_car_type.unwrap(),
                PassengerCarType::CompartmentCoach
            );
        }

        #[test]
        fn it_should_fail_to_convert_invalid_values_to_passenger_car_types() {
            let blank_value = "".parse::<PassengerCarType>();
            assert!(blank_value.is_err());

            let invalid_value = "invalid value".parse::<PassengerCarType>();
            assert!(invalid_value.is_err());
        }
    }

    mod locomotive_type_tests {
        use super::*;

        #[test]
        fn it_should_convert_string_slices_to_locomotive_types() {
            let locomotive_type = "STEAM_LOCOMOTIVE".parse::<LocomotiveType>();
            assert!(locomotive_type.is_ok());
            assert_eq!(
                locomotive_type.unwrap(),
                LocomotiveType::SteamLocomotive
            );
        }

        #[test]
        fn it_should_fail_to_convert_invalid_values_to_locomotive_types() {
            let blank_value = "".parse::<LocomotiveType>();
            assert!(blank_value.is_err());

            let invalid_value = "invalid value".parse::<LocomotiveType>();
            assert!(invalid_value.is_err());
        }
    }
}
