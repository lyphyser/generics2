#![no_std]
#![deny(warnings)]

#[doc(hidden)]
pub use core::compile_error as std_compile_error;
#[doc(hidden)]
pub use core::concat as std_concat;
#[doc(hidden)]
pub use core::stringify as std_stringify;

#[macro_export]
macro_rules! parse {
    (
        $callback:path { $($callback_args:tt)* } < $($token:tt)*
    ) => {
        $crate::parse_generics_impl! { [$callback] [$($callback_args)*] [] [] [$($token)*] }
    };
    (
        $callback:path { $($callback_args:tt)* } $(($($tuple:tt)*))? where $($token:tt)*
    ) => {
        $crate::std_compile_error!("unexpected 'where' without generics preceding");
    };
    (
        $callback:path { $($callback_args:tt)* } $($token:tt)*
    ) => {
        $callback ! { $($callback_args)* [] [] [] $($token)* }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! parse_generics_impl {
    (
        [$callback:path]
        [$($callback_args:tt)*]
        [$($g:tt)*]
        [$($r:tt)*]
        [$param:ident $($token:tt)*]
    ) => {
        $crate::parse_generics_impl! { 
            @param
            [$param]
            [$callback] [$($callback_args)*] [$($g)*] [$($r)*] 
            [$($token)*]
        }
    };
    (
        [$callback:path]
        [$($callback_args:tt)*]
        [$($g:tt)*]
        [$($r:tt)*]
        [$param:lifetime $($token:tt)*]
    ) => {
        $crate::parse_generics_impl! { 
            @param
            [$param]
            [$callback] [$($callback_args)*] [$($g)*] [$($r)*] 
            [$($token)*]
        }
    };
    (
        [$callback:path]
        [$($callback_args:tt)*]
        [$($g:tt)*]
        [$($r:tt)*]
        [$x:tt $($token:tt)*]
    ) => {
        $crate::std_compile_error!($crate::std_concat!(
            "unexpected token '",
            $crate::std_stringify!($x),
            "', expected ident, or lifetime"
        ));
    };
    (
        [$callback:path]
        [$($callback_args:tt)*]
        [$($([$($g:tt)*])+)?]
        [$($r:tt)*]
        []
    ) => {
        $crate::std_compile_error!($crate::std_concat!(
            "missing '>' after '",
            $crate::std_stringify!( < $($($($g)*),+ ,)? ),
            "'"
        ));
    };
    (
        @param
        [$param:tt]
        [$callback:path]
        [$($callback_args:tt)*]
        [$($g:tt)*]
        [$($r:tt)*]
        [ : $($token:tt)*]
    ) => {
        $crate::parse_generics_impl! {
            @constrained_param
            [$param]
            []
            [$callback] [$($callback_args)*] [$($g)*] [$($r)*] 
            [$($token)*]
        }
    };
    (
        @param
        [$param:tt]
        [$callback:path]
        [$($callback_args:tt)*]
        [$($g:tt)*]
        [$($r:tt)*]
        [ > $($token:tt)*]
    ) => {
        $crate::parse_generics_impl! {
            @done
            [$callback] [$($callback_args)*]
            [$($g)* [$param]]
            [$($r)* [$param]]
            [$($token)*]
        }
    };
    (
        @param
        [$param:tt]
        [$callback:path]
        [$($callback_args:tt)*]
        [$($g:tt)*]
        [$($r:tt)*]
        [ , > $($token:tt)*]
    ) => {
        $crate::parse_generics_impl! {
            @done
            [$callback] [$($callback_args)*]
            [$($g)* [$param]]
            [$($r)* [$param]]
            [$($token)*]
        }
    };
    (
        @param
        [$param:tt]
        [$callback:path]
        [$($callback_args:tt)*]
        [$($g:tt)*]
        [$($r:tt)*]
        [ , $($token:tt)*]
    ) => {
        $crate::parse_generics_impl! {
            [$callback] [$($callback_args)*]
            [$($g)* [$param]]
            [$($r)* [$param]]
            [$($token)*]
        }
    };
    (
        @param
        [$param:tt]
        [$callback:path]
        [$($callback_args:tt)*]
        [$($g:tt)*]
        [$($r:tt)*]
        [$x:tt $($token:tt)*]
    ) => {
        $crate::std_compile_error!($crate::std_concat!(
            "unexpected token '",
            $crate::std_stringify!($x),
            "', expected ':', ',', or '>'"
        ));
    };
    (
        @param
        [$param:tt]
        [$callback:path]
        [$($callback_args:tt)*]
        [$($([$($g:tt)*])+)?]
        [$($r:tt)*]
        []
    ) => {
        $crate::std_compile_error!($crate::std_concat!(
            "missing '>' after '",
            $crate::std_stringify!( < $($($($g)*),+ ,)? $param ),
            "'"
        ));
    };
    (
        @constrained_param
        [$param:tt]
        [$($constraint:tt)*]
        [$callback:path]
        [$($callback_args:tt)*]
        [$($g:tt)*]
        [$($r:tt)*]
        [ < $($token:tt)*]
    ) => {
        $crate::parse_generics_impl! {
            @angles_in_constraint
            [$param]
            [$($constraint)*]
            [] []
            [$callback] [$($callback_args)*] [$($g)*] [$($r)*] 
            [$($token)*]
        }
    };
    (
        @constrained_param
        [$param:tt]
        [$($constraint:tt)*]
        [$callback:path]
        [$($callback_args:tt)*]
        [$($g:tt)*]
        [$($r:tt)*]
        [ > $($token:tt)*]
    ) => {
        $crate::parse_generics_impl! {
            @done
            [$callback] [$($callback_args)*]
            [$($g)* [$param : $($constraint)*]]
            [$($r)* [$param]]
            [$($token)*]
        }
    };
    (
        @constrained_param
        [$param:tt]
        [$($constraint:tt)*]
        [$callback:path]
        [$($callback_args:tt)*]
        [$($g:tt)*]
        [$($r:tt)*]
        [ , > $($token:tt)*]
    ) => {
        $crate::parse_generics_impl! {
            @done
            [$callback] [$($callback_args)*]
            [$($g)* [$param : $($constraint)*]]
            [$($r)* [$param]]
            [$($token)*]
        }
    };
    (
        @constrained_param
        [$param:tt]
        [$($constraint:tt)*]
        [$callback:path]
        [$($callback_args:tt)*]
        [$($g:tt)*]
        [$($r:tt)*]
        [ , $($token:tt)*]
    ) => {
        $crate::parse_generics_impl! {
            [$callback] [$($callback_args)*]
            [$($g)* [$param : $($constraint)*]]
            [$($r)* [$param]]
            [$($token)*]
        }
    };
    (
        @constrained_param
        [$param:tt]
        [$($constraint:tt)*]
        [$callback:path]
        [$($callback_args:tt)*]
        [$($g:tt)*]
        [$($r:tt)*]
        [ $x:tt $($token:tt)*]
    ) => {
        $crate::parse_generics_impl! {
            @constrained_param
            [$param]
            [$($constraint)* $x]
            [$callback] [$($callback_args)*] [$($g)*] [$($r)*] 
            [$($token)*]
        }
    };
    (
        @constrained_param
        [$param:tt]
        [$($constraint:tt)*]
        [$callback:path]
        [$($callback_args:tt)*]
        [$($g:tt)*]
        [$($r:tt)*]
        []
    ) => {
        $crate::std_compile_error!($crate::std_concat!(
            "missing '>' after '",
            $crate::std_stringify!( < $($($($g)*),+ ,)? $param : $($constraint)* ),
            "'"
        ));
    };
    (
        @angles_in_constraint
        [$param:tt]
        [$($constraint:tt)*]
        [$($inside_angles:tt)*]
        []
        [$callback:path]
        [$($callback_args:tt)*]
        [$($g:tt)*]
        [$($r:tt)*]
        [ > $($token:tt)*]
    ) => {
        $crate::parse_generics_impl! {
            @constrained_param
            [$param]
            [$($constraint)* < $($inside_angles)* > ]
            [$callback] [$($callback_args)*] [$($g)*] [$($r)*] 
            [$($token)*]
        }
    };
    (
        @angles_in_constraint
        [$param:tt]
        [$($constraint:tt)*]
        [$($inside_angles:tt)*]
        [[$($parent_level:tt)*] $([$($outer_levels:tt)*])*]
        [$callback:path]
        [$($callback_args:tt)*]
        [$($g:tt)*]
        [$($r:tt)*]
        [ > $($token:tt)*]
    ) => {
        $crate::parse_generics_impl! {
            @angles_in_constraint
            [$param] [$($constraint)*]
            [$($parent_level)* < $($inside_angles)* > ]
            [$([$($outer_levels)*])*]
            [$callback] [$($callback_args)*] [$($g)*] [$($r)*] 
            [$($token)*]
        }
    };
    (
        @angles_in_constraint
        [$param:tt]
        [$($constraint:tt)*]
        [$($inside_angles:tt)*]
        [$([$($outer_levels:tt)*])*]
        [$callback:path]
        [$($callback_args:tt)*]
        [$($g:tt)*]
        [$($r:tt)*]
        [ < $($token:tt)*]
    ) => {
        $crate::parse_generics_impl! {
            @angles_in_constraint
            [$param] [$($constraint)*]
            []
            [[$($inside_angles:tt)*] $([$($outer_levels)*])*]
            [$callback] [$($callback_args)*] [$($g)*] [$($r)*] 
            [$($token)*]
        }
    };
    (
        @angles_in_constraint
        [$param:tt]
        [$($constraint:tt)*]
        [$($inside_angles:tt)*]
        [$([$($outer_levels:tt)*])*]
        [$callback:path]
        [$($callback_args:tt)*]
        [$($g:tt)*]
        [$($r:tt)*]
        [$x:tt $($token:tt)*]
    ) => {
        $crate::parse_generics_impl! {
            @angles_in_constraint
            [$param] [$($constraint)*]
            [$($inside_angles)* $x]
            [$([$($outer_levels)*])*]
            [$callback] [$($callback_args)*] [$($g)*] [$($r)*] 
            [$($token)*]
        }
    };
    (
        @angles_in_constraint
        [$param:tt]
        [$($constraint:tt)*]
        [$($inside_angles:tt)*]
        [$([$($outer_levels:tt)*])*]
        [$callback:path]
        [$($callback_args:tt)*]
        [$($([$($g:tt)*])+)?]
        [$($r:tt)*]
        []
    ) => {
        $crate::std_compile_error!($crate::std_concat!(
            "missing '>' after '",
            $crate::std_stringify!(
                < $($($($g)*),+ ,)? $param : $($constraint)*
                $( < $($outer_levels)* )* < $($inside_angles)*
            ),
            "'"
        ));
    };
    (
        @done
        [$callback:path]
        [$($callback_args:tt)*]
        [$([$($g:tt)*])+]
        [$([$($r:tt)*])+]
        [$(($($tuple:tt)*))? where $($token:tt)*]
    ) => {
        $crate::parse_where_clause_impl! {
            [$callback]
            [$($callback_args)*]
            [ < $($($g)*),+ > ]
            [ < $($($r)*),+ > ]
            [] [$(($($tuple)*))?] [$($token)*]
        }
    };
    (
        @done
        [$callback:path]
        [$($callback_args:tt)*]
        [$([$($g:tt)*])+]
        [$([$($r:tt)*])+]
        [$($token:tt)*]
    ) => {
        $callback ! {
            $($callback_args)*
            [ < $($($g)*),+ > ]
            [ < $($($r)*),+ > ]
            []
            $($token)*
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! parse_where_clause_impl {
    (
        [$callback:path]
        [$($callback_args:tt)*]
        [$($g:tt)*] [$($r:tt)*]
        [$($($w:tt)+)?]
        [$(($($tuple:tt)*))?] 
        [ ; $($token:tt)* ]
    ) => {
        $callback ! { 
            $($callback_args)*
            [$($g)*]
            [$($r)*]
            [$(where $($w)+)?]
            $(($($tuple:tt)*))?
            ; $($token)*
        }
    };
    (
        [$callback:path]
        [$($callback_args:tt)*]
        [$($g:tt)*] [$($r:tt)*]
        [$($w:tt)*]
        [($($tuple:tt)*)] 
        [ { $($body:tt)* } $($token:tt)* ]
    ) => {
        $crate::std_compile_error!("unexpected token '{', expected ';'");
    };
    (
        [$callback:path]
        [$($callback_args:tt)*]
        [$($g:tt)*] [$($r:tt)*]
        [$($($w:tt)+)?]
        [] 
        [ { $($body:tt)* } $($token:tt)* ]
    ) => {
        $callback ! { 
            $($callback_args)*
            [$($g)*]
            [$($r)*]
            [$(where $($w)+)?]
            { $($body)* } $($token)*
        }
    };
    (
        [$callback:path]
        [$($callback_args:tt)*]
        [$($g:tt)*] [$($r:tt)*]
        [$($w:tt)*]
        [$(($($tuple:tt)*))?] 
        [$token:tt $($other_tokens:tt)*]
    ) => {
        $crate::parse_where_clause_impl! { 
            [$callback]
            [$($callback_args)*]
            [$($g)*] [$($r)*]
            [$($w)* $token]
            [$(($($tuple)*))?] 
            [$($other_tokens)*]
        }
    };
    (
        [$callback:path]
        [$($callback_args:tt)*]
        [$($g:tt)*] [$($r:tt)*]
        [$($w:tt)*]
        [($($tuple:tt)*)] 
        []
    ) => {
        $crate::std_compile_error!($crate::std_concat!(
            "missing ';' after '",
            $crate::std_stringify!($($w)*),
            "'"
        ));
    };
    (
        [$callback:path]
        [$($callback_args:tt)*]
        [$($g:tt)*] [$($r:tt)*]
        [$($w:tt)*]
        [] 
        []
    ) => {
        $crate::std_compile_error!($crate::std_concat!(
            "missing ';' or '{' after '",
            $crate::std_stringify!($($w)*),
            "'"
        ));
    };
}

#[cfg(test)]
mod tests {
    macro_rules! impl_test_trait {
        (
            struct $name:ident $($token:tt)*
        ) => {
            parse! {
                impl_test_trait {
                    @impl struct $name
                }
                $($token)*
            }
        };
        (
            @impl struct $name:ident [$($g:tt)*] [$($r:tt)*] [$($w:tt)*] $($body:tt)*
        ) => {
            impl $($g)* TestTrait for $name $($r)* $($w)* { }
        };
    }

    trait TestTrait { }

    struct TestStruct { }

    impl_test_trait! {
        struct TestStruct { }
    }

    struct TestGenericStruct<'a, T: 'static> {
        a: &'a (),
        t: T,
    }

    impl_test_trait! {
        struct TestGenericStruct<'a, T: 'static> { }
    }

    #[test]
    fn it_works() {
        let test_struct = TestStruct { };
        let _: &dyn TestTrait = &test_struct;
        let test_generic_struct = TestGenericStruct {
            a: &(),
            t: ()
        };
        let _ = test_generic_struct.a;
        let _ = test_generic_struct.t;
        let _: &dyn TestTrait = &test_generic_struct;
    }
}
