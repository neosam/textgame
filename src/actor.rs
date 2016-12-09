

#[derive(Serialize, Deserialize, Debug)]
pub struct Container {
    pub value: u32,
    pub max: u32
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Actor {
    pub name: String,
    pub health: Container,
    pub visible: bool,
    pub display: bool
}

impl Actor {
    pub fn new() -> Actor {
        Actor {
            name: "".to_string(),
            visible: true,
            display: false,
            health: Container {
                value: 0,
                max: 0
            }
        }
    }
}