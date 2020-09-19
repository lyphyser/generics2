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
        $callback:path { $($callback_args:tt)* } $($token:tt)*
    ) => {
        $crate::deny_where_clause_impl! { [$callback] [$($callback_args)*] [] [$($token)*] }
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
            []
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
            []
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
            []
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
            []
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
        [$($inter:tt)*]
        [ ; $($token:tt)*]
    ) => {
        $callback ! {
            $($callback_args)*
            [ < $($($g)*),+ > ]
            [ < $($($r)*),+ > ]
            []
            $($inter)* ; $($token)*
        }
    };
    (
        @done
        [$callback:path]
        [$($callback_args:tt)*]
        [$([$($g:tt)*])+]
        [$([$($r:tt)*])+]
        [$($inter:tt)*]
        [ { $($body:tt)* } $($token:tt)*]
    ) => {
        $callback ! {
            $($callback_args)*
            [ < $($($g)*),+ > ]
            [ < $($($r)*),+ > ]
            []
            $($inter)* { $($body)* } $($token)*
        }
    };
    (
        @done
        [$callback:path]
        [$($callback_args:tt)*]
        [$([$($g:tt)*])+]
        [$([$($r:tt)*])+]
        [$($inter:tt)*]
        [where $($token:tt)*]
    ) => {
        $crate::parse_where_clause_impl! {
            [$callback]
            [$($callback_args)*]
            [ < $($($g)*),+ > ]
            [ < $($($r)*),+ > ]
            [] [$($inter)*] [$($token)*]
        }
    };
    (
        @done
        [$callback:path]
        [$($callback_args:tt)*]
        [$($g:tt)+]
        [$($r:tt)+]
        [$($inter:tt)*]
        [$token:tt $($other_tokens:tt)*]
    ) => {
        $crate::parse_generics_impl! {
            @done
            [$callback] [$($callback_args)*]
            [$($g)+]
            [$($r)+]
            [$($inter)* $token]
            [$($other_tokens)*]
        }
    };
    (
        @done
        [$callback:path]
        [$($callback_args:tt)*]
        [$([$($g:tt)*])+]
        [$([$($r:tt)*])+]
        [$($inter:tt)*]
        []
    ) => {
        $crate::std_compile_error!($crate::std_concat!(
            "missing ';', '{', or 'where' after '",
            $crate::std_stringify!(
                < $($($g)*),+ >
                [$($inter)*]
            ),
            "'"
        ));
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
        [$($inter:tt)*] 
        [ ; $($token:tt)* ]
    ) => {
        $callback ! { 
            $($callback_args)*
            [$($g)*]
            [$($r)*]
            [$(where $($w)+)?]
            $($inter)* ; $($token)*
        }
    };
    (
        [$callback:path]
        [$($callback_args:tt)*]
        [$($g:tt)*] [$($r:tt)*]
        [$($($w:tt)+)?]
        [$($inter:tt)*] 
        [ { $($body:tt)* } $($token:tt)* ]
    ) => {
        $callback ! { 
            $($callback_args)*
            [$($g)*]
            [$($r)*]
            [$(where $($w)+)?]
            $($inter)* { $($body)* } $($token)*
        }
    };
    (
        [$callback:path]
        [$($callback_args:tt)*]
        [$($g:tt)*] [$($r:tt)*]
        [$($w:tt)*]
        [$($inter:tt)*] 
        [$token:tt $($other_tokens:tt)*]
    ) => {
        $crate::parse_where_clause_impl! { 
            [$callback]
            [$($callback_args)*]
            [$($g)*] [$($r)*]
            [$($w)* $token]
            [$($inter)*]
            [$($other_tokens)*]
        }
    };
    (
        [$callback:path]
        [$($callback_args:tt)*]
        [$($g:tt)*] [$($r:tt)*]
        [$($w:tt)*]
        [$($inter:tt)*] 
        []
    ) => {
        $crate::std_compile_error!($crate::std_concat!(
            "missing ';' or '{' after '",
            $crate::std_stringify!($($w)*),
            "'"
        ));
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! deny_where_clause_impl {
    (
        [$callback:path]
        [$($callback_args:tt)*]
        [$($inter:tt)*]
        [ ; $($token:tt)*]
    ) => {
        $callback ! {
            $($callback_args)*
            []
            []
            []
            $($inter)* ; $($token)*
        }
    };
    (
        [$callback:path]
        [$($callback_args:tt)*]
        [$($inter:tt)*]
        [ { $($body:tt)* } $($token:tt)*]
    ) => {
        $callback ! {
            $($callback_args)*
            []
            []
            []
            $($inter)* { $($body)* } $($token)*
        }
    };
    (
        [$callback:path]
        [$($callback_args:tt)*]
        [$($inter:tt)*]
        [where $($token:tt)*]
    ) => {
        $crate::std_compile_error!("unexpected 'where' without generics preceding");
    };
    (
        [$callback:path]
        [$($callback_args:tt)*]
        [$($inter:tt)*]
        [$token:tt $($other_tokens:tt)*]
    ) => {
        $crate::deny_where_clause_impl! {
            [$callback] [$($callback_args)*]
            [$($inter)* $token]
            [$($other_tokens)*]
        }
    };
    (
        [$callback:path]
        [$($callback_args:tt)*]
        [$($inter:tt)*]
        []
    ) => {
        $crate::std_compile_error!($crate::std_concat!(
            "missing ';', '{', or 'where' after '",
            $crate::std_stringify!(
                < $($($g)*),+ >
                [$($inter)*]
            ),
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

    macro_rules! impl_tr {
        (
            struct $name:ident $($token:tt)*
        ) => {
            parse! {
                impl_tr {
                    @impl struct $name
                }
                $($token)*
            }
        };
        (
            @impl struct $name:ident [$($g:tt)*] [$($r:tt)*] [$($w:tt)*] become $tr:ident $($body:tt)*
        ) => {
            impl $($g)* $tr for $name $($r)* $($w)* { }
        };
    }

    trait TestTrait2 { }

    impl_tr! {
        struct TestStruct become TestTrait2 { }
    }

    impl_tr! {
        struct TestGenericStruct<'a, T> become TestTrait2 where T: 'static { }
    }
}
