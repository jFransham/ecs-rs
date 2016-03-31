use std::marker::PhantomData;
use components::{GetComponent, SetComponent};
use entity::{EntityId, EntityStore};

pub trait System<UpdateData, SystemState: Sized = ()> {
    fn update(
        &mut self,
        es: &mut EntityStore,
        ud: &UpdateData,
    ) -> Option<SystemState>;
}

pub trait SimpleSystem<UpdateData> {
    type Input: for<'b> GetComponent<'b>;
    type Output: SetComponent;

    fn update_simple<'b>(
        &mut self,
        entities: &[(EntityId, <Self::Input as GetComponent<'b>>::Out)],
        ud: &UpdateData
    ) -> Vec<(EntityId, Self::Output)>;
}

impl<U, S: Sized, T: SimpleSystem<U>> System<U, S> for T {
    fn update(&mut self, es: &mut EntityStore, ud: &U) -> Option<S> {
        let pairs = {
            let ents = es.entity_component_pairs::<T::Input>();
            self.update_simple(&ents, ud)
        };

        for (e, c) in pairs {
            c.set_component(es, e);
        }

        None
    }
}

pub trait SystemContainer<U, S: Sized = ()> {
    fn update_all(&mut self, _: &mut EntityStore, _: &U) -> Option<S>;
}

impl<U, S: Sized> SystemContainer<U, S> for Vec<Box<System<U, S>>> {
    fn update_all(&mut self, es: &mut EntityStore, ud: &U) -> Option<S> {
        let mut out = None;

        for s in self {
            if let (&None, s @ Some(_)) = (&out, s.update(es, &ud)) {
                out = s;
            }
        }

        out
    }
}

impl_tuple_system! {
    (
        A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X,
        Y, Z
    )
}

pub struct SystemStore<
    U, S: Sized = (), T: SystemContainer<U, S> = Vec<Box<System<U, S>>>
> {
    entity_store: EntityStore,
    systems: T,
    _p: PhantomData<(U, S)>,
}

impl<U, S, T: SystemContainer<U, S>> SystemStore<U, S, T> {
    pub fn with_systems(s: T) -> Self {
        SystemStore {
            entity_store: EntityStore::new(),
            systems: s,
            _p: PhantomData,
        }
    }

    pub fn update(&mut self, ud: U) -> Option<S> {
        self.systems.update_all(&mut self.entity_store, &ud)
    }
}
