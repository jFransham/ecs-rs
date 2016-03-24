use std::marker::Reflect;
use std::any::{TypeId, Any};
use std::collections::HashMap;

/**
 * Represents a dynamic set of objects of unknown type
 **/
pub struct HeterogenousSet {
    content: HashMap<TypeId, Box<Any>>,
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
        self.content.insert(type_id, Box::new(element) as Box<Any>);
    }

    pub fn insert_raw(&mut self, element: Box<Any>) {
        let type_id = (*element).get_type_id();
        self.content.insert(type_id, element);
    }
}

