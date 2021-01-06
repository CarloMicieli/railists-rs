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

impl YamlRollingStock {
    pub fn to_rolling_stock(self) -> anyhow::Result<RollingStock> {
        let length_over_buffer = self.length.map(|l| LengthOverBuffer::new(l));
        let control = self.control.and_then(|c| c.parse::<Control>().ok());
        let dcc_interface = self
            .dcc_interface
            .and_then(|dcc| dcc.parse::<DccInterface>().ok());

        let epoch = self.epoch.parse::<Epoch>()?;

        match self.category.as_str() {
            "LOCOMOTIVE" => Ok(RollingStock::new_locomotive(
                self.type_name,
                self.road_number.unwrap_or_default(),
                self.series,
                Railway::new(&self.railway),
                epoch,
                self.sub_category
                    .and_then(|c| c.parse::<LocomotiveType>().ok())
                    .unwrap(),
                self.depot,
                self.livery,
                length_over_buffer,
                control,
                dcc_interface,
            )),
            "TRAIN" => Ok(RollingStock::new_train(
                self.type_name,
                self.road_number,
                1,
                Railway::new(&self.railway),
                epoch,
                self.sub_category.and_then(|c| c.parse::<TrainType>().ok()),
                self.depot,
                self.livery,
                length_over_buffer,
                control,
                dcc_interface,
            )),
            "PASSENGER_CAR" => Ok(RollingStock::new_passenger_car(
                self.type_name,
                self.road_number,
                Railway::new(&self.railway),
                epoch,
                self.sub_category
                    .and_then(|c| c.parse::<PassengerCarType>().ok()),
                self.service_level
                    .and_then(|sl| sl.parse::<ServiceLevel>().ok()),
                self.depot,
                self.livery,
                length_over_buffer,
            )),
            "FREIGHT_CAR" => Ok(RollingStock::new_freight_car(
                self.type_name,
                self.road_number,
                Railway::new(&self.railway),
                epoch,
                self.sub_category
                    .and_then(|c| c.parse::<FreightCarType>().ok()),
                self.depot,
                self.livery,
                length_over_buffer,
            )),
            _ => Err(anyhow!("Invalid rolling stock type")),
        }
    }
}
