use std::marker::Reflect;
use either::Either;
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

impl<'a, A: GetComponent<'a>, B: GetComponent<'a>> GetComponent<'a>
    for Either<A, B>
{
    type Out = Either<A::Out, B::Out>;

    fn get_component<'b: 'a>(
        es: &'b EntityStore,
        entity: EntityId
    ) -> Option<Self::Out> {
        if let Some(a) = A::get_component(es, entity) {
            Some(Either::Left(a))
        } else if let Some(b) = B::get_component(es, entity) {
            Some(Either::Right(b))
        } else {
            None
        }
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
