use std::marker::PhantomData;
use components::{GetComponent, SetComponent};
use entity::{EntityId, EntityStore};

pub trait System<UpdateData, Message, SystemState: Sized = ()> {
    fn update(
        &mut self,
        msgs: &mut Vec<Message>,
        es: &mut EntityStore,
        ud: &UpdateData,
    ) -> Option<SystemState>;
}

pub trait SimpleSystem<UpdateData, Message> {
    type Input: for<'b> GetComponent<'b>;
    type Output: SetComponent;

    fn update_simple<'b>(
        &mut self,
        entities: &[(EntityId, <Self::Input as GetComponent<'b>>::Out)],
        ud: &UpdateData
    ) -> (Vec<(EntityId, Self::Output)>, Vec<Message>);

    fn handle_message(&mut self, msg: Message) -> Option<Message> { Some(msg) }
}

impl<U, M, S: Sized, T: SimpleSystem<U, M>> System<U, M, S> for T {
    fn update(
        &mut self,
        ms: &mut Vec<M>,
        es: &mut EntityStore,
        ud: &U
    ) -> Option<S> {
        use std::mem::replace;

        let v = replace(ms, vec![]);

        *ms = v.into_iter()
            .filter_map(|a| self.handle_message(a))
            .collect();

        let (pairs, mut msgs) = {
            let ents = es.entity_component_pairs::<T::Input>();
            self.update_simple(&ents, ud)
        };

        ms.append(&mut msgs);

        for (e, c) in pairs {
            c.set_component(es, e);
        }

        None
    }
}

pub trait SystemSet<U, M, S: Sized = ()> {
    fn update_all(
        &mut self,
        _: &mut Vec<M>,
        _: &mut EntityStore,
        _: &U
    ) -> Option<S>;
}

impl<U, M, S: Sized> SystemSet<U, M, S> for Vec<Box<System<U, M, S>>> {
    fn update_all(
        &mut self,
        m: &mut Vec<M>,
        es: &mut EntityStore,
        ud: &U
    ) -> Option<S> {
        let mut out = None;

        for s in self {
            if let (&None, s @ Some(_)) = (&out, s.update(m, es, &ud)) {
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
    U, M, S: Sized = (), T: SystemSet<U, M, S> = Vec<Box<System<U, M, S>>>
> {
    entity_store: EntityStore,
    systems: T,
    message_buffer: Vec<M>,
    _p: PhantomData<(U, S)>,
}

impl<U, M, S, T: SystemSet<U, M, S>> SystemStore<U, M, S, T> {
    pub fn with_systems(s: T) -> Self {
        SystemStore {
            entity_store: EntityStore::new(),
            systems: s,
            message_buffer: vec![],
            _p: PhantomData,
        }
    }

    pub fn update(&mut self, ud: U) -> Option<S> {
        self.systems.update_all(
            &mut self.message_buffer,
            &mut self.entity_store,
            &ud
        )
    }
}
