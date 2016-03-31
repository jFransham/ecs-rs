use std::marker::Reflect;
use std::collections::BTreeMap;
use dynamic::Dynamic;
use heterogenous_set::HeterogenousSet;
use components::{GetComponent, SetComponent};

pub type EntityId = usize;

pub struct EntityStore {
    next_id: EntityId,
    entities: BTreeMap<EntityId, HeterogenousSet>,
}

impl EntityStore {
    pub fn new() -> EntityStore {
        EntityStore { next_id: 0, entities: BTreeMap::new() }
    }

    pub fn entity_component_pairs<
        'a, 'b: 'a, T: GetComponent<'a>
    >(&'b self) -> Vec<(EntityId, T::Out)> {
        self.entities
            .keys()
            .filter_map(|&ent|
                T::get_component(&self, ent)
                    .map(|c| (ent, c))
            )
            .collect::<Vec<_>>()
    }

    pub fn get_component<'a, T: Reflect + 'static>(
        &self, id: EntityId
    ) -> Option<&T> {
        self.entities.get(&id).and_then(|l| l.get::<T>())
    }

    pub fn set_component<
        T: Reflect + SetComponent + 'static
    >(&mut self, id: EntityId, component: T) {
        if let Some(l) = self.entities.get_mut(&id) {
            l.insert(component);
        }
    }

    pub fn set_raw_component(
        &mut self,
        id: EntityId,
        component: Box<Dynamic>
    ) {
        if let Some(l) = self.entities.get_mut(&id) {
            l.insert_raw(component);
        }
    }

    pub fn create_entity(&mut self) -> EntityId {
        let entity = self.next_id;

        debug_assert!(!self.entities.contains_key(&entity));

        self.entities.insert(entity, HeterogenousSet::new());

        let mut next = self.next_id + 1;
        while self.entities.contains_key(&next) {
            next += 1;
        }

        self.next_id = next;

        entity
    }

    pub fn remove_entity(&mut self, ent: EntityId) {
        self.entities.remove(&ent);
        self.next_id = ent;
    }
}

