use crate::domain::catalog::{
    categories::{FreightCarType, LocomotiveType, PassengerCarType, TrainType},
    railways::Railway,
    rolling_stocks::{
        Control, DccInterface, Epoch, LengthOverBuffer, RollingStock,
        ServiceLevel,
    },
};

#[derive(Debug, Deserialize, Clone)]
pub struct YamlRollingStock {
    #[serde(rename = "typeName")]
    pub type_name: String,
    #[serde(rename = "roadNumber")]
    pub road_number: Option<String>,
    pub series: Option<String>,
    pub railway: String,
    pub epoch: String,
    #[serde(default)]
    pub category: String,
    #[serde(rename = "subCategory")]
    pub sub_category: Option<String>,
    pub depot: Option<String>,
    pub length: Option<u32>,
    pub livery: Option<String>,
    #[serde(rename = "serviceLevel")]
    pub service_level: Option<String>,
    pub control: Option<String>,
    #[serde(rename = "dccInterface")]
    pub dcc_interface: Option<String>,
}

impl std::convert::TryFrom<YamlRollingStock> for RollingStock {
    type Error = anyhow::Error;

    fn try_from(value: YamlRollingStock) -> Result<Self, Self::Error> {
        let length_over_buffer = value.length.map(LengthOverBuffer::new);
        let control = value.control.and_then(|c| c.parse::<Control>().ok());
        let dcc_interface = value
            .dcc_interface
            .and_then(|dcc| dcc.parse::<DccInterface>().ok());

        let epoch = value.epoch.parse::<Epoch>()?;

        match value.category.as_str() {
            "LOCOMOTIVE" => Ok(RollingStock::new_locomotive(
                value.type_name,
                value.road_number.unwrap_or_default(),
                value.series,
                Railway::new(&value.railway),
                epoch,
                value
                    .sub_category
                    .and_then(|c| c.parse::<LocomotiveType>().ok())
                    .unwrap(),
                value.depot,
                value.livery,
                length_over_buffer,
                control,
                dcc_interface,
            )),
            "TRAIN" => Ok(RollingStock::new_train(
                value.type_name,
                value.road_number,
                1,
                Railway::new(&value.railway),
                epoch,
                value.sub_category.and_then(|c| c.parse::<TrainType>().ok()),
                value.depot,
                value.livery,
                length_over_buffer,
                control,
                dcc_interface,
            )),
            "PASSENGER_CAR" => Ok(RollingStock::new_passenger_car(
                value.type_name,
                value.road_number,
                Railway::new(&value.railway),
                epoch,
                value
                    .sub_category
                    .and_then(|c| c.parse::<PassengerCarType>().ok()),
                value
                    .service_level
                    .and_then(|sl| sl.parse::<ServiceLevel>().ok()),
                value.depot,
                value.livery,
                length_over_buffer,
            )),
            "FREIGHT_CAR" => Ok(RollingStock::new_freight_car(
                value.type_name,
                value.road_number,
                Railway::new(&value.railway),
                epoch,
                value
                    .sub_category
                    .and_then(|c| c.parse::<FreightCarType>().ok()),
                value.depot,
                value.livery,
                length_over_buffer,
            )),
            _ => Err(anyhow!("Invalid rolling stock type")),
        }
    }
}
