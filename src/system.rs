use components::{GetComponent, SetComponent};
use entity::{EntityId, EntityStore};

pub trait System<Messages, UpdateData> {
    fn update_all(&mut self, es: &mut EntityStore, ud: &UpdateData);
}

pub trait SimpleSystem<Messages, UpdateData> {
    type Input: for<'b> GetComponent<'b>;
    type Output: SetComponent;

    fn update<'b>(
        &mut self,
        entities: &[(EntityId, <Self::Input as GetComponent<'b>>::Out)],
        ud: &UpdateData
    ) -> Vec<(EntityId, Self::Output, Messages)>;

    fn handle_messages(
        &mut self,
        _: &mut EntityStore,
        m: Messages
    ) -> Messages {
        m
    }
}

impl<M, U, T: SimpleSystem<M, U>> System<M, U> for T {
    fn update_all(&mut self, es: &mut EntityStore, ud: &U) {
        let pairs = {
            let ents = es.entity_component_pairs::<T::Input>();
            self.update(&ents, ud)
        };

        for (e, c, _) in pairs {
            c.set_component(es, e);
        }
    }
}

/*
pub trait ReadonlySystem<Messages>: Send + Sync {
    type Input: for<'b> GetComponent<'b>;

    fn update<'b>(
        &mut self,
        entities: &[(EntityId, <Self::Input as GetComponent<'b>>::Out)]
    );
}

impl<Messages, T: ReadonlySystem<Messages>> SimpleSystem<Messages> for T {
    type Input = T::Input;
    type Output = ();

    fn update<'b>(
        &mut self,
        entities: &[(EntityId, <Self::Input as GetComponent<'b>>::Out)]
    ) -> Vec<(EntityId, (), Messages)> {
        ReadonlySystem::update(self, entities);
        vec![]
    }
}
*/

pub struct SystemStore<Messages, UpdateData> {
    entity_store: EntityStore,
    systems: Vec<Box<System<Messages, UpdateData>>>,
}

impl<Messages, UpdateData> SystemStore<Messages, UpdateData> {
    pub fn new<M, U>() -> SystemStore<M, U> {
        SystemStore {
            entity_store: EntityStore::new(),
            systems: vec![],
        }
    }

    pub fn with_systems(
        s: Vec<Box<System<Messages, UpdateData>>>
    ) -> SystemStore<Messages, UpdateData> {
        SystemStore {
            entity_store: EntityStore::new(),
            systems: s,
        }
    }

    pub fn update(&mut self, ud: UpdateData) {
        for s in self.systems.iter_mut() {
            s.update_all(&mut self.entity_store, &ud);
        }
    }
}
