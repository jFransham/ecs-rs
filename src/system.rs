use std::marker::PhantomData;
use components::{GetComponent, SetComponent};
use entity::{EntityId, EntityStore};

pub trait System<UpdateData> {
    fn update(
        &mut self,
        es: &mut EntityStore,
        ud: &UpdateData,
    );
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

impl<U, T: SimpleSystem<U>> System<U> for T {
    fn update(&mut self, es: &mut EntityStore, ud: &U) {
        let pairs = {
            let ents = es.entity_component_pairs::<T::Input>();
            self.update_simple(&ents, ud)
        };

        for (e, c) in pairs {
            c.set_component(es, e);
        }
    }
}

pub trait SystemContainer<U> {
    fn update_all(&mut self, _: &mut EntityStore, _: &U);
}

impl<U> SystemContainer<U> for Vec<Box<System<U>>> {
    fn update_all(&mut self, es: &mut EntityStore, ud: &U) {
        for s in self {
            s.update(es, &ud);
        }
    }
}

impl_tuple_system! {
    (
        A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X,
        Y, Z
    )
}

pub struct SystemStore<
    U, T: SystemContainer<U> = Vec<Box<System<U>>>
> {
    entity_store: EntityStore,
    systems: T,
    _p: PhantomData<U>,
}

impl<U, T: SystemContainer<U>> SystemStore<U, T> {
    pub fn with_systems(s: T) -> Self {
        SystemStore {
            entity_store: EntityStore::new(),
            systems: s,
            _p: PhantomData,
        }
    }

    pub fn update(&mut self, ud: U) {
        self.systems.update_all(&mut self.entity_store, &ud);
    }
}
