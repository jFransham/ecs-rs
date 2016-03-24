use std::marker::Reflect;
use entity::{EntityId, EntityStore};

pub trait GetComponent<'a>: Sized {
    type Out;

    fn get_component<'b: 'a>(es: &'b EntityStore, entity: EntityId)
        -> Option<Self::Out>;
}

pub trait SetComponent: Sized + Reflect + 'static {
    fn set_component(self, es: &mut EntityStore, entity: EntityId) {
        es.set_component(entity, self);
    }
}

impl<'a, 'any, T: Sized + Reflect + 'static> GetComponent<'a> for &'any T {
    type Out = &'a T;

    fn get_component<'b: 'a>(
        es: &'b EntityStore,
        entity: EntityId
    ) -> Option<Self::Out> {
        es.get_component::<T>(entity)
    }
}

impl<'a, T: GetComponent<'a>> GetComponent<'a> for Option<T> {
    type Out = Option<T::Out>;

    fn get_component<'b: 'a>(
        es: &'b EntityStore,
        entity: EntityId
    ) -> Option<Self::Out> {
        Some(T::get_component(es, entity))
    }
}

impl_tuple_components! { (A, B, C, D, E, F, G, H, I, J, K) }
