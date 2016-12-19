use std::marker::PhantomData;


#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Holder<T, K: HolderKey> {
    pub items: Vec<T>,
    pub phantom: PhantomData<K>
}

pub trait HolderKey {
    fn new(usize) -> Self;
    fn get(&self) -> usize;
}

impl<T, K: HolderKey> Holder<T, K> {
    pub fn add(&mut self, item: T) -> K {
        let res = self.items.len();
        self.items.push(item);
        HolderKey::new(res)
    }

    pub fn get(&self, i: K) -> &T {
        self.items.get(i.get()).unwrap()
    }

    pub fn get_mut(&mut self, i: K) -> &mut T {
        self.items.get_mut(i.get()).unwrap()
    }
}

#[macro_export]
macro_rules! key_gen {
    ($name:ident) => (
        #[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq, Default, Hash)]
        pub struct $name(usize);
        impl HolderKey for $name {
            fn new(i: usize) -> Self { $name(i) }
            fn get(&self) -> usize {
                let $name(i) = *self;
                i
            }
        }
    )
}