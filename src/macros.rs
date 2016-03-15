macro_rules! component {
    ($t:ty) => {
        impl SetComponent for $t {}
    };
}

macro_rules! impl_tuple_components {
    (($t:ident $(, $rest:ident )*)) => {
        impl<'a, $t: GetComponent<'a>, $( $rest: GetComponent<'a> ),*>
            GetComponent<'a> for ($t, $( $rest ),*)
        {
            type Out = (
                $t::Out,
                $(
                    $rest::Out,
                )*
            );

            #[allow(non_snake_case)]
            fn get_component<'b: 'a>(
                es: &'b EntityStore, entity: EntityId
            ) -> Option<Self::Out> {
                $t::get_component(es, entity)
                    .and_then(|first|
                        <( $( $rest ,)* )>::get_component(es, entity)
                            .map(|( $( $rest ,)* )|
                                 (
                                     first,
                                     $( $rest ),*
                                 )
                             )
                    )
            }
        }

        impl<'a, $t: SetComponent, $( $rest: SetComponent ),*>
            SetComponent for ($t, $( $rest ),*)
        {
            #[allow(non_snake_case)]
            fn set_component(
                self, es: &mut EntityStore, entity: EntityId
            ) {
                let ($t, $( $rest ,)*) = self;
                $t.set_component(es, entity);
                $(
                    $rest.set_component(es, entity);
                )*
            }
        }

        impl_tuple_components! {
            ( $( $rest ),* )
        }
    };
    (()) => {
        impl<'a> GetComponent<'a> for () {
            fn get_component<'b: 'a>(_: &'b EntityStore, _: EntityId) -> Option<Self> {
                Some(())
            }
        }
    };
}
