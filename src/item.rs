use base::Watchable;

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Item {
    pub keyword: String,
    pub label: String,
    pub description: String
}

impl Watchable for Item {
    fn watch(&self) -> String {
        let mut res = String::with_capacity(1024);
        res.push_str(&self.label);
        res.push('\n');
        res.push_str(&self.description);
        res.push('\n');
        res
    }
}