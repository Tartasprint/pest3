//! A [macro](crate::sequence_type) to generate sequence type
//! and generated sequence types that has 2 to 15 fields,
//! which are named `field_{i}`.

/// Generate sequence type.
///
/// ## Example
///
/// ```rust
/// pest3_core::sequence_type!(Sequence2, (field_0, T0, TRIVIA_0), (field_1, T1, TRIVIA_1));
/// ```
#[macro_export]
macro_rules! sequence_type {
    ( $name:ident, $( ( $field:ident, $type:ident, $trivia:ident ) ),* $(,)? ) => {
        /// Sequence type generated by `pest::sequence_type!`.
        ///
        /// - `0`: No trivia.
        /// - `1`: Optional trivia.
        /// - `2`: Mandatory trivia.
        #[derive(Clone, Debug, Eq, Hash, PartialEq)]
        pub struct $name<$( $type, $trivia, )*> {
            $(
                #[doc = ::core::stringify!(Field for $type.)]
                pub $field: $type,
            )*
            __trivia: ::core::marker::PhantomData<($($trivia, )*)>,
        }
        impl<'i, $($type, $trivia, )*>
        $name<$($type, $trivia, )*> {
            /// As a tuple of references.
            pub fn as_tuple(&self) -> ($(&$type, )*) {
                ( $( &self.$field, )* )
            }
            /// Into a tuple of values.
            pub fn into_tuple(self) -> ($($type, )*) {
                ( $( self.$field, )* )
            }
        }
        impl<'i, R, $($type, $trivia, )*>
        $crate::typed::TypedNode<'i, R> for $name<$($type, $trivia, )*>
        where
            R: $crate::typed::RuleType,
            $(
                $type: $crate::typed::TypedNode<'i, R>,
                $trivia: $crate::typed::TypedNode<'i, R>,
            )*
        {
            #[inline]
            fn try_parse_with_partial(
                mut input: $crate::Position<'i>,
                stack: &mut $crate::Stack<$crate::Span<'i>>,
                tracker: &mut $crate::typed::Tracker<'i, R>,
            ) -> ::core::option::Option<($crate::Position<'i>, Self)> {
                let mut i = 0usize;
                $(
                    i += 1;
                    if i > 1 {
                        input = $trivia::check_with_partial(input, stack, tracker)?;
                    }
                    let (next, $field) = $type::try_parse_with_partial(input, stack, tracker)?;
                    input = next;
                )*
                let res = Self {
                    $($field, )*
                    __trivia: ::core::marker::PhantomData,
                };
                ::core::option::Option::Some((input, res))
            }
            #[inline]
            fn check_with_partial(
                mut input: $crate::Position<'i>,
                stack: &mut $crate::Stack<$crate::Span<'i>>,
                tracker: &mut $crate::typed::Tracker<'i, R>,
            ) -> ::core::option::Option<$crate::Position<'i>> {
                let mut i = 0usize;
                $(
                    i += 1;
                    if i > 1 {
                        input = $trivia::check_with_partial(input, stack, tracker)?;
                    }
                    let next = $type::check_with_partial(input, stack, tracker)?;
                    input = next;
                )*
                ::core::option::Option::Some(input)
            }
        }
        impl<R, $($type, $trivia, )*>
        $crate::typed::PairContainer<R> for $name<$($type, $trivia, )*>
        where
            $(
                $type: $crate::typed::PairContainer<R>,
            )*
        {
            fn for_each_child_pair(&self, f: &mut impl $crate::std::FnMut($crate::token::Pair<R>)) {
                $(
                    self.$field.for_self_or_for_each_child_pair(f);
                )*
            }
        }
    };
}

pub use sequence_type;

sequence_type! {
    Sequence2,
    (field_0, T0, TR0),
    (field_1, T1, TR1)
}

sequence_type! {
    Sequence3,
    (field_0, T0, TR0),
    (field_1, T1, TR1),
    (field_2, T2, TR2)
}

sequence_type! {
    Sequence4,
    (field_0, T0, TR0),
    (field_1, T1, TR1),
    (field_2, T2, TR2),
    (field_3, T3, TR3)
}

sequence_type! {
    Sequence5,
    (field_0, T0, TR0),
    (field_1, T1, TR1),
    (field_2, T2, TR2),
    (field_3, T3, TR3),
    (field_4, T4, TR4)
}

sequence_type! {
    Sequence6,
    (field_0, T0, TR0),
    (field_1, T1, TR1),
    (field_2, T2, TR2),
    (field_3, T3, TR3),
    (field_4, T4, TR4),
    (field_5, T5, TR5)
}

sequence_type! {
    Sequence7,
    (field_0, T0, TR0),
    (field_1, T1, TR1),
    (field_2, T2, TR2),
    (field_3, T3, TR3),
    (field_4, T4, TR4),
    (field_5, T5, TR5),
    (field_6, T6, TR6)
}

sequence_type! {
    Sequence8,
    (field_0, T0, TR0),
    (field_1, T1, TR1),
    (field_2, T2, TR2),
    (field_3, T3, TR3),
    (field_4, T4, TR4),
    (field_5, T5, TR5),
    (field_6, T6, TR6),
    (field_7, T7, TR7)
}

sequence_type! {
    Sequence9,
    (field_0, T0, TR0),
    (field_1, T1, TR1),
    (field_2, T2, TR2),
    (field_3, T3, TR3),
    (field_4, T4, TR4),
    (field_5, T5, TR5),
    (field_6, T6, TR6),
    (field_7, T7, TR7),
    (field_8, T8, TR8)
}

sequence_type! {
    Sequence10,
    (field_0, T0, TR0),
    (field_1, T1, TR1),
    (field_2, T2, TR2),
    (field_3, T3, TR3),
    (field_4, T4, TR4),
    (field_5, T5, TR5),
    (field_6, T6, TR6),
    (field_7, T7, TR7),
    (field_8, T8, TR8),
    (field_9, T9, TR9)
}

sequence_type! {
    Sequence11,
    (field_0, T0, TR0),
    (field_1, T1, TR1),
    (field_2, T2, TR2),
    (field_3, T3, TR3),
    (field_4, T4, TR4),
    (field_5, T5, TR5),
    (field_6, T6, TR6),
    (field_7, T7, TR7),
    (field_8, T8, TR8),
    (field_9, T9, TR9),
    (field_10, T10, TR10)
}

sequence_type! {
    Sequence12,
    (field_0, T0, TR0),
    (field_1, T1, TR1),
    (field_2, T2, TR2),
    (field_3, T3, TR3),
    (field_4, T4, TR4),
    (field_5, T5, TR5),
    (field_6, T6, TR6),
    (field_7, T7, TR7),
    (field_8, T8, TR8),
    (field_9, T9, TR9),
    (field_10, T10, TR10),
    (field_11, T11, TR11)
}

sequence_type! {
    Sequence13,
    (field_0, T0, TR0),
    (field_1, T1, TR1),
    (field_2, T2, TR2),
    (field_3, T3, TR3),
    (field_4, T4, TR4),
    (field_5, T5, TR5),
    (field_6, T6, TR6),
    (field_7, T7, TR7),
    (field_8, T8, TR8),
    (field_9, T9, TR9),
    (field_10, T10, TR10),
    (field_11, T11, TR11),
    (field_12, T12, TR12)
}

sequence_type! {
    Sequence14,
    (field_0, T0, TR0),
    (field_1, T1, TR1),
    (field_2, T2, TR2),
    (field_3, T3, TR3),
    (field_4, T4, TR4),
    (field_5, T5, TR5),
    (field_6, T6, TR6),
    (field_7, T7, TR7),
    (field_8, T8, TR8),
    (field_9, T9, TR9),
    (field_10, T10, TR10),
    (field_11, T11, TR11),
    (field_12, T12, TR12),
    (field_13, T13, TR13)
}

sequence_type! {
    Sequence15,
    (field_0, T0, TR0),
    (field_1, T1, TR1),
    (field_2, T2, TR2),
    (field_3, T3, TR3),
    (field_4, T4, TR4),
    (field_5, T5, TR5),
    (field_6, T6, TR6),
    (field_7, T7, TR7),
    (field_8, T8, TR8),
    (field_9, T9, TR9),
    (field_10, T10, TR10),
    (field_11, T11, TR11),
    (field_12, T12, TR12),
    (field_13, T13, TR13),
    (field_14, T14, TR14)
}

sequence_type! {
    Sequence16,
    (field_0, T0, TR0),
    (field_1, T1, TR1),
    (field_2, T2, TR2),
    (field_3, T3, TR3),
    (field_4, T4, TR4),
    (field_5, T5, TR5),
    (field_6, T6, TR6),
    (field_7, T7, TR7),
    (field_8, T8, TR8),
    (field_9, T9, TR9),
    (field_10, T10, TR10),
    (field_11, T11, TR11),
    (field_12, T12, TR12),
    (field_13, T13, TR13),
    (field_14, T14, TR14),
    (field_15, T15, TR15)
}
