macro_rules! component {
    ($t:ty) => {
        impl $crate::components::SetComponent for $t {}
    };
}

macro_rules! impl_tuple_components {
    (($t:ident $(, $rest:ident )*)) => {
        impl<
            'a,
            $t: $crate::components::GetComponent<'a>,
            $( $rest: $crate::components::GetComponent<'a> ),*
        >GetComponent<'a> for ($t, $( $rest ),*) {
            type Out = (
                $t::Out,
                $(
                    $rest::Out,
                )*
            );

            #[allow(non_snake_case)]
            fn get_component<'b: 'a>(
                es: &'b $crate::entity::EntityStore,
                entity: $crate::entity::EntityId
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
                self,
                es: &mut $crate::entity::EntityStore,
                entity: $crate::entity::EntityId
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
            type Out = ();

            fn get_component<'b: 'a>(_: &'b EntityStore, _: EntityId) -> Option<Self> {
                Some(())
            }
        }

        impl SetComponent for () {
            fn set_component(
                self,
                _: &mut $crate::entity::EntityStore,
                _: $crate::entity::EntityId
            ) {}
        }
    };
}

macro_rules! impl_tuple_system {
    (($t:ident $(, $rest:ident )*)) => {
        impl<
            __U__,
            __M__,
            __S__,
            $t: $crate::system::System<__U__, __M__, __S__>,
            $(
                $rest: $crate::system::System<__U__, __M__, __S__>
            ),*
        >SystemSet<__U__, __M__, __S__> for ($t, $( $rest ),*) {
            #[allow(non_snake_case)]
            fn update_all(
                &mut self,
                mb: &mut Vec<__M__>,
                es: &mut EntityStore,
                ud: &__U__
            ) -> Option<__S__> {
                let &mut (ref mut $t, $( ref mut $rest ),*) = self;

                $t.update(mb, es, ud)
                $(
                    .or_else(|| $rest.update(mb, es, ud))
                )*
            }
        }

        impl_tuple_system! {
            ( $( $rest ),* )
        }
    };
    (()) => {
    };
}
