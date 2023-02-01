#[doc(hidden)]
pub trait OptTuple {
    type Output;

    fn to_opt(t: Self) -> Self::Output;
}

#[doc(hidden)]
pub use tuple_loop_macro::generate_for_loop as generate_for_loop__;

#[macro_export]
macro_rules! tuple_for {
    ($($t:tt)*) => {
        $crate::generate_for_loop__!($($t)*)
    };
}

macro_rules! generate_tuple_impls {
    ($($t:ident)*) => {
      generate_tuple_impls!(@make_rest ($($t)*) rest() remaining($($t)*));
    };

    (@make_rest ($i:ident $($ir:ident)*) rest($($rest:ident)*) remaining($_:ident $($rem:ident)*)) => {
      generate_tuple_impls!(@make_rest ($i $($ir)*) rest($($rest)* $i) remaining($($rem)*));
    };

    (@make_rest ($($i:ident)*) rest($($rest:ident)*) remaining()) => {
      generate_tuple_impls!(@munch ($($i)*) types() rest($($rest)*));
    };

    (@munch ($p:ident $($pr:ident)*) types($($t:ident)*) rest($_:ident $($rr:ident)*)) => {
      generate_tuple_impls!(@gen types($($t)* $p) rest($($rr)*));
      generate_tuple_impls!(@munch ($($pr)*) types($($t)* $p) rest($($rr)*));
    };

    (@gen types($($t:ident)+) rest($($r:ident)*)) => {
        #[allow(non_snake_case)]
        impl<$($t),*> OptTuple for ($($t),*,) {
            type Output = ($(Option<$t>),* , $(Option<$r>),*);
            fn to_opt(($($t),* ,): Self) -> Self::Output {
                ($(Some($t)),*, $(None as Option<$r>),*)
            }
        }

        #[allow(non_snake_case)]
        impl<'t, $($t),*> OptTuple for &'t ($($t),*,) {
            type Output = ($(Option<&'t $t>),* , $(Option<&'t $r>),*);
            fn to_opt(($($t),* ,): Self) -> Self::Output {
                ($(Some($t)),*, $(None as Option<&'t $r>),*)
            }
        }

        #[allow(non_snake_case)]
        impl<'t, $($t),*> OptTuple for &'t mut ($($t),*,) {
            type Output = ($(Option<&'t mut $t>),* , $(Option<&'t mut $r>),*);
            fn to_opt(($($t),* ,): Self) -> Self::Output {
                ($(Some($t)),*, $(None as Option<&'t mut $r>),*)
            }
        }
    };


    (@munch () types($($t:ident)*) $($_:tt)*) => { };

}

generate_tuple_impls! { A B C D E F G H I J K L }
