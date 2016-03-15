#![cfg(bin)]

#![feature(reflect_marker)]
#![feature(associated_type_defaults)]

#[macro_use]
mod macros;
mod heterogenous_set;
mod entity;
mod components;
mod system;

use entity::{EntityId, EntityStore};
use system::System;

#[derive(Debug)]
struct Test;

component!(Test);
component!(u32);

// TODO: Write a parallelisation special-case based on Output = ()? i.e. all
//       Output = () systems can be run asyncronously.
struct DoSomethingSystem;
impl<'a> System<'a, ()> for DoSomethingSystem {
    type Input = (&'static u32, &'static Test);
    type Output = u32;

    fn update(
        &mut self,
        entities: &[(EntityId, (&u32, &Test))]
    ) -> Vec<(EntityId, u32, ())> {
        let v = entities.into_iter().collect::<Vec<_>>();

        println!("Entities: {:?}", v);

        v.into_iter()
            .map(|&(e, (i, _))| (e, i + 1, ()))
            .collect()
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
