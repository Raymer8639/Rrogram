use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
#[serde(untagged)]
pub enum Wiget {
    Button(Button),
    ElectricWire(ElectricWire),
    Lamp(Lamp),
}

#[derive(Deserialize, Serialize)]
pub struct ElectricWire {
    pub name: String,
    pub wigets: Vec<Wiget>,
    pub state: bool,
    pub layer: u32,
}

#[derive(Deserialize, Serialize)]
pub struct Button {
    pub name: String,
    pub wigets: Vec<Wiget>,
    pub state: bool,
    pub layer: u32,
}

#[derive(Deserialize, Serialize)]
pub struct Lamp {
    pub name: String,
    pub state: bool,
    pub layer: u32,
}

#[derive(Deserialize, Serialize)]
pub struct Project {
    pub wiget: Vec<Wiget>,
}
