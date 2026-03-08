use std::{any::TypeId, collections::HashMap};

use std::any::Any;

pub mod debugging;

pub struct CompilerHelpers {
    map: HashMap<TypeId, Box<dyn Any>>,
}

impl CompilerHelpers {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    pub fn get_mut<T: Any>(&mut self) -> Option<&mut T> {
        self.map
            .get_mut(&TypeId::of::<T>())
            .map(|h| h.downcast_mut().expect("could not downcast"))
    }

    pub fn get_or_insert<T: Any>(&mut self, init: impl FnOnce() -> T) -> &mut T {
        self.map
            .entry(TypeId::of::<T>())
            .or_insert_with(|| Box::new(init()))
            .downcast_mut::<T>()
            .expect("could not downcast")
    }
}
