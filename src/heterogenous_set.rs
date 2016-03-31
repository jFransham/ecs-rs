use std::marker::Reflect;
use std::any::TypeId;
use std::collections::HashMap;
use dynamic::Dynamic;

/**
 * Represents a dynamic set of objects of unknown type
 **/
pub struct HeterogenousSet {
    content: HashMap<TypeId, Box<Dynamic>>,
}

impl HeterogenousSet {
    pub fn new() -> Self {
        HeterogenousSet { content: HashMap::new() }
    }

    pub fn get<T: Reflect + 'static>(&self) -> Option<&T> {
        self.content.get(
            &TypeId::of::<T>()
        ).and_then(|b| b.downcast_ref::<T>())
    }

    pub fn insert<T: Reflect + 'static>(&mut self, element: T) {
        let type_id = TypeId::of::<T>();
        self.content.insert(type_id, Dynamic::new(element));
    }

    pub fn insert_raw(&mut self, element: Box<Dynamic>) {
        let type_id = (*element).id();
        self.content.insert(type_id, element);
    }
}

