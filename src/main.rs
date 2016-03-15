#![feature(reflect_marker)]
#![feature(associated_type_defaults)]
#![feature(type_macros)]

use std::marker::Reflect;
use std::any::{TypeId, Any};
use std::collections::{HashMap, BTreeMap};

#[macro_use]
mod macros;

type EntityId = usize;
type Component = Reflect + 'static;

trait GetComponent<'a>: Sized {
    type Out = Self;

    fn get_component<'b: 'a>(es: &'b EntityStore, entity: EntityId)
        -> Option<Self::Out>;
}

trait SetComponent: Sized + Reflect + 'static {
    fn set_component(self, es: &mut EntityStore, entity: EntityId) {
        es.set_component(entity, self);
    }
}

impl<'a, T: Sized + Reflect + 'static> GetComponent<'a> for &'static T {
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

trait System<'a> {
    type Input: for<'b> GetComponent<'b>;
    type Output: SetComponent;

    fn update_all<'b: 'a>(&mut self, es: &'b mut EntityStore) {
        let pairs = {
            let ents = es.entity_component_pairs::<Self::Input>();
            self.update(ents)
        };

        for (e, c) in pairs {
            c.set_component(es, e);
        }
    }

    fn update<'b>(
        &mut self,
        entities: Vec<(EntityId, <Self::Input as GetComponent<'b>>::Out)>
    ) -> Vec<(EntityId, Self::Output)>;
}

#[derive(Debug)]
struct Test;

component!(Test);
component!(u32);

// TODO: Write a parallelisation special-case based on Output = ()? i.e. all
//       Output = () systems can be run asyncronously.
struct DoSomethingSystem;
impl<'a> System<'a> for DoSomethingSystem {
    type Input = (&'static u32, &'static Test);
    type Output = u32;

    fn update(
        &mut self,
        entities: Vec<(EntityId, (&u32, &Test))>
    ) -> Vec<(EntityId, u32)> {
        let v = entities.into_iter().collect::<Vec<_>>();

        println!("Entities: {:?}", v);

        v.into_iter()
            .map(|(e, (i, _))| (e, i + 1))
            .collect()
    }
}

struct EntityStore {
    next_id: EntityId,
    entities: BTreeMap<EntityId, HeterogenousSet>,
}

impl EntityStore {
    fn new() -> EntityStore {
        EntityStore { next_id: 0, entities: BTreeMap::new() }
    }

    fn entity_component_pairs<
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

    fn get_component<T: Reflect + 'static>(&self, id: EntityId) -> Option<&T> {
        self.entities.get(&id).and_then(|l| l.get::<T>())
    }

    fn set_component<T: Reflect + 'static>(
        &mut self,
        id: EntityId,
        component: T
    ) {
        if let Some(l) = self.entities.get_mut(&id) {
            l.insert(component);
        }
    }

    fn create_entity(&mut self) -> EntityId {
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

    fn remove_entity(&mut self, ent: EntityId) {
        self.entities.remove(&ent);
        self.next_id = ent;
    }
}

/**
 * Represents a dynamic set of objects of unknown type
 **/
struct HeterogenousSet {
    content: HashMap<TypeId, Box<Any>>,
}

impl HeterogenousSet {
    fn new() -> Self {
        HeterogenousSet { content: HashMap::new() }
    }

    fn get<T: Reflect + 'static>(&self) -> Option<&T> {
        self.content.get(
            &TypeId::of::<T>()
        ).and_then(|b| b.downcast_ref::<T>())
    }

    fn insert<T: Reflect + 'static>(&mut self, element: T) {
        let type_id = TypeId::of::<T>();
        self.content.insert(type_id, Box::new(element) as Box<Any>);
    }
}

fn main() {
    let mut store = EntityStore::new();
    let mut sys = DoSomethingSystem;

    let ent = store.create_entity();

    for i in 0..3u32 {
        let ent = store.create_entity();
        store.set_component(ent, Test);
        store.set_component(ent, i);
    }

    store.remove_entity(ent);

    for i in 3..5u32 {
        let ent = store.create_entity();
        store.set_component(ent, Test);
        store.set_component(ent, i);
    }

    for _ in 0..100 {
        sys.update_all(&mut store);
    }
}

#[test]
fn test_heterogenous_set() {
    let mut a = HeterogenousSet::new();
    a.insert(1u32);
    assert!(a.get::<u32>() == Some(&1));
}

#[test]
fn test_do_something_system() {
    let mut store = EntityStore::new();
    let mut sys = DoSomethingSystem;

    sys.update_all(&mut store);
}
