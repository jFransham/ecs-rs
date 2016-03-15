use components::{GetComponent, SetComponent};
use entity::{EntityId, EntityStore};

pub trait System<'a, Messages> {
    type Input: for<'b> GetComponent<'b>;
    type Output: SetComponent;

    fn update_all<'b: 'a>(&mut self, es: &'b mut EntityStore) {
        let pairs = {
            let ents = es.entity_component_pairs::<Self::Input>();
            self.update(&ents)
        };

        for (e, c, _) in pairs {
            c.set_component(es, e);
        }
    }

    fn update<'b>(
        &mut self,
        entities: &[(EntityId, <Self::Input as GetComponent<'b>>::Out)]
    ) -> Vec<(EntityId, Self::Output, Messages)>;

    fn handle_messages(
        &mut self,
        _: &mut EntityStore,
        m: Messages
    ) -> Messages {
        m
    }
}

pub trait ReadonlySystem<'a, Messages>: Send + Sync {
    type Input: for<'b> GetComponent<'b>;

    fn update<'b>(
        &mut self,
        entities: &[(EntityId, <Self::Input as GetComponent<'b>>::Out)]
    );
}

impl<'a, Messages, T: ReadonlySystem<'a, Messages>> System<'a, Messages> for T {
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
