#![no_std]
#![deny(warnings)]

#[doc(hidden)]
pub use core::compile_error as std_compile_error;
#[doc(hidden)]
pub use core::concat as std_concat;
#[doc(hidden)]
pub use core::stringify as std_stringify;

#[macro_export]
macro_rules! shim_and_where_clause {
    (
        [$callback:path] [$($callback_args:tt)*] [ < $($token:tt)* ]
    ) => {
        $crate::shim_impl! { [$crate::where_clause_impl] [ <> [$callback] [$($callback_args)*]] [] [] [$($token)*] }
    };
    (
        [$callback:path] [$($callback_args:tt)*] [ where $($token:tt)* ]
    ) => {
        $crate::where_clause_impl! { [$callback] [$($callback_args)* [] []] [] [$($token)*] }
    };
    (
        [$callback:path] [$($callback_args:tt)*] [$($token:tt)*]
    ) => {
        $callback ! { $($callback_args)* [] [] [] $($token)* }
    };
}

#[macro_export]
macro_rules! where_clause {
    (
        [$callback:path] [$($callback_args:tt)*] [ where $($token:tt)* ]
    ) => {
        $crate::where_clause_impl! { [$callback] [$($callback_args)*] [] [$($token)*] }
    };
    (
        [$callback:path] [$($callback_args:tt)*] [$($token:tt)*]
    ) => {
        $callback ! { $($callback_args)* [] $($token)* }
    };
}

#[macro_export]
macro_rules! shim {
    (
        [$callback:path] [$($callback_args:tt)*] [ < $($token:tt)* ]
    ) => {
        $crate::shim_impl! { [$callback] [$($callback_args)*] [] [] [$($token)*] }
    };
    (
        [$callback:path] [$($callback_args:tt)*] [$($token:tt)*]
    ) => {
        $callback ! { $($callback_args)* [] [] $($token)* }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! where_clause_impl {
    (
        <>
        [$callback:path]
        [$($callback_args:tt)*]
        [$($g:tt)*] [$($r:tt)*]
        [ where $($token:tt)*]
    ) => {
        $crate::where_clause_impl! { 
            [$callback]
            [$($callback_args)* [$($g)*] [$($r)*]]
            []
            [$($token)*]
        }
    };
    (
        <>
        [$callback:path]
        [$($callback_args:tt)*]
        [$($g:tt)*] [$($r:tt)*]
        [$($token:tt)*]
    ) => {
        $callback ! { 
            $($callback_args)* [$($g)*] [$($r)*] [] $($token)*
        }
    };
    (
        [$callback:path]
        [$($callback_args:tt)*]
        [$($w:tt)*]
        [ ; ]
    ) => {
        $callback ! { 
            $($callback_args)*
            [$($w)*]
            ;
        }
    };
    (
        [$callback:path]
        [$($callback_args:tt)*]
        [$($w:tt)*]
        [ { $($body:tt)* } ]
    ) => {
        $callback ! { 
            $($callback_args)*
            [$($w)*]
            { $($body)* }
        }
    };
    (
        [$callback:path]
        [$($callback_args:tt)*]
        [$($w:tt)*]
        [$token:tt $($other_tokens:tt)*]
    ) => {
        $crate::where_clause_impl! { 
            [$callback]
            [$($callback_args)*]
            [$($w)* $token]
            [$($other_tokens)*]
        }
    };
    (
        [$callback:path]
        [$($callback_args:tt)*]
        [$($w:tt)*]
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
macro_rules! shim_impl {
    (
        [$callback:path]
        [$($callback_args:tt)*]
        [$($g:tt)*]
        [$($r:tt)*]
        [$param:ident $($token:tt)*]
    ) => {
        $crate::shim_impl! { 
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
        $crate::shim_impl! { 
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
        $crate::shim_impl! {
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
        $crate::shim_impl! {
            @break
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
        $crate::shim_impl! {
            @break
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
        $crate::shim_impl! {
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
        $crate::shim_impl! {
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
        $crate::shim_impl! {
            @break
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
        $crate::shim_impl! {
            @break
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
        $crate::shim_impl! {
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
        $crate::shim_impl! {
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
        $crate::shim_impl! {
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
        $crate::shim_impl! {
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
        $crate::shim_impl! {
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
        $crate::shim_impl! {
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
        @break
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
            [$($token)*]
        }
    };
}

#[cfg(test)]
mod tests {
    macro_rules! impl_test_trait {
        (
            struct $name:ident $($token:tt)*
        ) => {
            shim! {
                [impl_test_trait] [@impl [$name]] [$($token)*]
            }
        };
        (
            @impl
            [$name:ident] [$($g:tt)*] [$($r:tt)*] [$($body:tt)+]
        ) => {
            impl $($g)* TestTrait for $name $($r)* { }
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
