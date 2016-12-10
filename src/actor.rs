

#[derive(Serialize, Deserialize, Debug)]
pub struct Container {
    pub value: u32,
    pub max: u32
}

/// Value should be at least 1 to avoid immediate death.
impl Default for Container {
    fn default() -> Self {
        Container {
            value: 1,
            max: 1
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Actor {
    pub keyword: String,
    pub name: String,
    pub health: Container,
    pub visible: bool,
    pub display: bool
}