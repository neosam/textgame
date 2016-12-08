#[derive(Serialize, Deserialize, Debug)]
pub struct Holder<T, K: HolderKey> {
    pub items: Vec<T>,
    pub phantom: PhantomData<K>
}