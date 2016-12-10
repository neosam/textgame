#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Item {
    pub keyword: String,
    pub label: String,
    pub description: String
}