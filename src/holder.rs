use std::marker::PhantomData;


#[cfg(feature = "serde_derive")]
include!("holder_types.in.rs");

#[cfg(feature = "serde_codegen")]
include!(concat!(env!("OUT_DIR"), "/holder_types.rs"));

pub trait HolderKey {
    fn new(usize) -> Self;
    fn get(&self) -> usize;
}

impl<T, K: HolderKey> Holder<T, K> {
    pub fn new() -> Self  {
        Holder {
            items: Vec::new(),
            phantom: PhantomData
        }
    }
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
        #[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
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