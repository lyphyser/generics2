#![no_std]
#![deny(warnings)]
#![doc(test(attr(deny(warnings))))]
#![doc(test(attr(allow(dead_code))))]
#![doc(test(attr(allow(unused_variables))))]

#[doc(hidden)]
pub use core::compile_error as std_compile_error;
#[doc(hidden)]
pub use core::concat as std_concat;
#[doc(hidden)]
pub use core::stringify as std_stringify;

/// Parses (optional) generics and (optional) subsequent where clause.
///
/// This macro accepts an input in the following form:
///
/// ```ignore
/// $callback_macro { $($callback_macro_args)* }
/// $(
///     < $generics >
///     $( $tokens_between_generics_and_where_clause )*
///     $(
///         where $where_clause
///     )?
/// )?
/// $(
///     $( ; | { $($body)* } )
///     $($remaining_tokens)*
/// )?
/// ```
///
/// and expands into
///
/// ```ignore
/// $callback_macro! {
///     $( $callback_macro_args )*
///     [ $( < $generics > )? ]
///     [ $( < $generics_without_constraints > )? ]
///     [ $( where $where_clause )? ]
///     $($( $tokens_between_generics_and_where_clause )*)?
///     $(
///         $( ; | { $($body)* } )
///         $($remaining_tokens)*
///     )?
/// }
/// ```
///
/// # Examples
///
/// ```rust
/// pub trait TheTrait { }
///
/// #[doc(hidden)]
/// pub use generics::parse as generics_parse;
/// #[doc(hidden)]
/// pub use std::compile_error as std_compile_error;
///
/// #[macro_export]
/// macro_rules! impl_the_trait {
///     (
///         $name:ident $($token:tt)*
///     ) => {
///         $crate::generics_parse! {
///             $crate::impl_the_trait {
///                 @impl $name
///             }
///             $($token)*
///         }
///     };
///     (
///         @impl $name:ident [$($g:tt)*] [$($r:tt)*] [$($w:tt)*]
///     ) => {
///         impl $($g)* $crate::TheTrait for $name $($r)* $($w)* { }
///     };
///     (
///         @impl $name:ident [$($g:tt)*] [$($r:tt)*] [$($w:tt)*] $($token:tt)+ 
///     ) => {
///         $crate::std_compile_error!(
///             "invalid input, allowed input is '$name $( < $generics > $(where $where_clause)? )?'"
///         );
///     };
/// }
/// ```
#[macro_export]
macro_rules! parse {
    (
        $callback:path { $($callback_args:tt)* } < $($token:tt)*
    ) => {
        $crate::parse_generics_impl! { [$crate::parse_callback] [$callback [$($callback_args)*]] [] [] [$($token)*] }
    };
    (
        $callback:path { $($callback_args:tt)* } $($token:tt)*
    ) => {
        $crate::deny_where_clause_impl! { [$crate::parse_callback] [$callback [$($callback_args)*]] [] [$($token)*] }
    };
}

/// Callback for `parse_raw` that calls a callback for `parse` 
#[doc(hidden)]
#[macro_export]
macro_rules! parse_callback {
    (
        $callback:path
        [$($callback_args:tt)*]
        [
            [$([$([$($g:tt)*])*])?]
            [$([$([$($r:tt)*])*])?]
            [$($w:tt)*]
            $($extra:tt)*
        ]
        $($rest:tt)*
    ) => {
        $callback ! {
            $($callback_args)*
            [ $(< $($($g)*),+ >)? ]
            [ $(< $($($r)*),+ >)? ]
            [$($w)*]
            $($rest)*
        }
    };
}

/// Parses (optional) generics and (optional) subsequent where clause, keeping the structure of the parsed information.
///
/// This macro accepts an input in the following form:
///
/// ```ignore
/// $callback_macro { $($callback_macro_args)* }
/// $(
///     < $generics >
///     $( $tokens_between_generics_and_where_clause )*
///     $(
///         where $where_clause
///     )?
/// )?
/// $(
///     $( ; | { $($body)* } )
///     $($remaining_tokens)*
/// )?
/// ```
///
/// and expands into
///
/// ```ignore
/// $callback_macro! {
///     $( $callback_macro_args )*
///     [
///        [ $( [ $([ $generics ])* ] )? ]
///        [ $( [ $([ $generics_without_constraints ]) ] )? ]
///        [ $( where $where_clause )? ]
///        $($extra_reserved_for_future_expansion:tt)*
///     ]
///     $($( $tokens_between_generics_and_where_clause )*)?
///     $(
///         $( ; | { $($body)* } )
///         $($remaining_tokens)*
///     )?
/// }
/// ```
///
/// The $extra_reserved_for_future_expansion ends with multiple placeholders to force users
/// to use a wildcard match rather than an exact match, allowing to add new data
/// in future crate versions without breaking compatibility.
///
/// # Examples
///
/// ```rust
/// pub trait TheTrait { }
///
/// #[doc(hidden)]
/// pub use generics::parse as generics_parse;
/// #[doc(hidden)]
/// pub use std::compile_error as std_compile_error;
///
/// #[macro_export]
/// macro_rules! impl_the_trait {
///     (
///         $name:ident $($token:tt)*
///     ) => {
///         $crate::generics_parse! {
///             $crate::impl_the_trait {
///                 @impl $name
///             }
///             $($token)*
///         }
///     };
///     (
///         @impl $name:ident [[$([$([$($g:tt)*])*])?] [$([$([$($r:tt)*])*])?] [$(where $($w:tt)*)?] $($extra:tt)*]
///     ) => {
///         impl $(<$($($g)*),*>)? $crate::TheTrait for $name $(<$($($r)*),*>)? $(where $($w)*)? { }
///     };
///     (
///         @impl $name:ident [[$($g:tt)*] [$($r:tt)*] [$($w:tt)*] $($extra:tt)*] $($token:tt)+ 
///     ) => {
///         $crate::std_compile_error!(
///             "invalid input, allowed input is '$name $( < $generics > $(where $where_clause)? )?'"
///         );
///     };
/// }
/// ```
#[macro_export]
macro_rules! parse_raw {
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
        [const $param:ident $($token:tt)*]
    ) => {
        $crate::parse_generics_impl! {
            @param
            [[const $param] [$param]]
            [$callback] [$($callback_args)*] [$($g)*] [$($r)*]
            [$($token)*]
        }
    };
    (
        [$callback:path]
        [$($callback_args:tt)*]
        [$($g:tt)*]
        [$($r:tt)*]
        [$param:ident $($token:tt)*]
    ) => {
        $crate::parse_generics_impl! { 
            @param
            [[$param] [$param]]
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
            [[$param] [$param]]
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
        [[$($gparam:tt)*] [$($rparam:tt)*]]
        [$callback:path]
        [$($callback_args:tt)*]
        [$($g:tt)*]
        [$($r:tt)*]
        [ : $($token:tt)*]
    ) => {
        $crate::parse_generics_impl! {
            @constrained_param [:]
            [[$($gparam)*] [$($rparam)*]]
            [] []
            [$callback] [$($callback_args)*] [$($g)*] [$($r)*]
            [$($token)*]
        }
    };
    (
        @param
        [[$($gparam:tt)*] [$($rparam:tt)*]]
        [$callback:path]
        [$($callback_args:tt)*]
        [$($g:tt)*]
        [$($r:tt)*]
        [ = $($token:tt)*]
    ) => {
        $crate::parse_generics_impl! {
            @constrained_param [=]
            [[$($gparam)*] [$($rparam)*]]
            [] []
            [$callback] [$($callback_args)*] [$($g)*] [$($r)*] 
            [$($token)*]
        }
    };
    (
        @param
        [[$($gparam:tt)*] [$($rparam:tt)*]]
        [$callback:path]
        [$($callback_args:tt)*]
        [$($g:tt)*]
        [$($r:tt)*]
        [ > $($token:tt)*]
    ) => {
        $crate::parse_generics_impl! {
            @done
            [$callback] [$($callback_args)*]
            [$($g)* [$($gparam)*]]
            [$($r)* [$($rparam)*]]
            []
            [$($token)*]
        }
    };
    (
        @param
        [[$($gparam:tt)*] [$($rparam:tt)*]]
        [$callback:path]
        [$($callback_args:tt)*]
        [$($g:tt)*]
        [$($r:tt)*]
        [ >> $($token:tt)*]
    ) => {
        $crate::parse_generics_impl! {
            @done
            [$callback] [$($callback_args)*]
            [$($g)* [$($gparam)*]]
            [$($r)* [$($rparam)*]]
            []
            [ > $($token)*]
        }
    };
    (
        @param
        [[$($gparam:tt)*] [$($rparam:tt)*]]
        [$callback:path]
        [$($callback_args:tt)*]
        [$($g:tt)*]
        [$($r:tt)*]
        [ , > $($token:tt)*]
    ) => {
        $crate::parse_generics_impl! {
            @done
            [$callback] [$($callback_args)*]
            [$($g)* [$($gparam)*]]
            [$($r)* [$($rparam)*]]
            []
            [$($token)*]
        }
    };
    (
        @param
        [[$($gparam:tt)*] [$($rparam:tt)*]]
        [$callback:path]
        [$($callback_args:tt)*]
        [$($g:tt)*]
        [$($r:tt)*]
        [ , >> $($token:tt)*]
    ) => {
        $crate::parse_generics_impl! {
            @done
            [$callback] [$($callback_args)*]
            [$($g)* [$($gparam)*]]
            [$($r)* [$($rparam)*]]
            []
            [ > $($token)*]
        }
    };
    (
        @param
        [[$($gparam:tt)*] [$($rparam:tt)*]]
        [$callback:path]
        [$($callback_args:tt)*]
        [$($g:tt)*]
        [$($r:tt)*]
        [ , $($token:tt)*]
    ) => {
        $crate::parse_generics_impl! {
            [$callback] [$($callback_args)*]
            [$($g)* [$($gparam)*]]
            [$($r)* [$($rparam)*]]
            [$($token)*]
        }
    };
    (
        @param
        [[$($gparam:tt)*] [$($rparam:tt)*]]
        [$callback:path]
        [$($callback_args:tt)*]
        [$($g:tt)*]
        [$($r:tt)*]
        [$x:tt $($token:tt)*]
    ) => {
        $crate::std_compile_error!($crate::std_concat!(
            "unexpected token '",
            $crate::std_stringify!($x),
            "', expected ':', '=', ',', or '>'"
        ));
    };
    (
        @param
        [[$([$($gparam:tt)*])*] [$($rparam:tt)*]]
        [$callback:path]
        [$($callback_args:tt)*]
        [$($([$($g:tt)*])+)?]
        [$($r:tt)*]
        []
    ) => {
        $crate::std_compile_error!($crate::std_concat!(
            "missing '>' after '",
            $crate::std_stringify!( < $($($($g)*),+ ,)? $([$($gparam)*])* ),
            "'"
        ));
    };
    (
        @constrained_param [$kind:tt]
        [[$($gparam:tt)*] [$($rparam:tt)*]]
        [$($constraint:tt)*] [$($value:tt)*]
        [$callback:path]
        [$($callback_args:tt)*]
        [$($g:tt)*]
        [$($r:tt)*]
        [ < $($token:tt)*]
    ) => {
        $crate::parse_generics_impl! {
            @angles_in_constraint [$kind]
            [[$($gparam)*] [$($rparam)*]]
            [$($constraint)*] [$($value)*]
            [] []
            [$callback] [$($callback_args)*] [$($g)*] [$($r)*] 
            [$($token)*]
        }
    };
    (
        @constrained_param [$kind:tt]
        [[$($gparam:tt)*] [$($rparam:tt)*]]
        [$($constraint:tt)*] [$($value:tt)*]
        [$callback:path]
        [$($callback_args:tt)*]
        [$($g:tt)*]
        [$($r:tt)*]
        [ << $($token:tt)*]
    ) => {
        $crate::parse_generics_impl! {
            @angles_in_constraint [$kind]
            [[$($gparam)*] [$($rparam)*]]
            [$($constraint)*] [$($value)*]
            [] []
            [$callback] [$($callback_args)*] [$($g)*] [$($r)*]
            [ < $($token)*]
        }
    };
    (
        @constrained_param [$kind:tt]
        [[$($gparam:tt)*] [$($rparam:tt)*]]
        [$($constraint:tt)*] [$($value:tt)*]
        [$callback:path]
        [$($callback_args:tt)*]
        [$($g:tt)*]
        [$($r:tt)*]
        [ > $($token:tt)*]
    ) => {
        $crate::parse_generics_impl! {
            @done
            [$callback] [$($callback_args)*]
            [$($g)* [$($gparam)* : $($constraint)*]]
            [$($r)* [$($rparam)*]]
            []
            [$($token)*]
        }
    };
    (
        @constrained_param [$kind:tt]
        [[$($gparam:tt)*] [$($rparam:tt)*]]
        [$($constraint:tt)*] [$($value:tt)*]
        [$callback:path]
        [$($callback_args:tt)*]
        [$($g:tt)*]
        [$($r:tt)*]
        [ >> $($token:tt)*]
    ) => {
        $crate::parse_generics_impl! {
            @done
            [$callback] [$($callback_args)*]
            [$($g)* [$($gparam)* : $($constraint)*]]
            [$($r)* [$($rparam)*]]
            []
            [ > $($token)*]
        }
    };
    (
        @constrained_param [$kind:tt]
        [[$($gparam:tt)*] [$($rparam:tt)*]]
        [$($constraint:tt)*] [$($value:tt)*]
        [$callback:path]
        [$($callback_args:tt)*]
        [$($g:tt)*]
        [$($r:tt)*]
        [ , > $($token:tt)*]
    ) => {
        $crate::parse_generics_impl! {
            @done
            [$callback] [$($callback_args)*]
            [$($g)* [$($gparam)* : $($constraint)*]]
            [$($r)* [$($rparam)*]]
            []
            [$($token)*]
        }
    };
    (
        @constrained_param [$kind:tt]
        [[$($gparam:tt)*] [$($rparam:tt)*]]
        [$($constraint:tt)*] [$($value:tt)*]
        [$callback:path]
        [$($callback_args:tt)*]
        [$($g:tt)*]
        [$($r:tt)*]
        [ , >> $($token:tt)*]
    ) => {
        $crate::parse_generics_impl! {
            @done
            [$callback] [$($callback_args)*]
            [$($g)* [$($gparam)* : $($constraint)*]]
            [$($r)* [$($rparam)*]]
            []
            [ > $($token)*]
        }
    };
    (
        @constrained_param [$kind:tt]
        [[$($gparam:tt)*] [$($rparam:tt)*]]
        [$($constraint:tt)*] [$($value:tt)*]
        [$callback:path]
        [$($callback_args:tt)*]
        [$($g:tt)*]
        [$($r:tt)*]
        [ , $($token:tt)*]
    ) => {
        $crate::parse_generics_impl! {
            [$callback] [$($callback_args)*]
            [$($g)* [$($gparam)* : $($constraint)*]]
            [$($r)* [$($rparam)*]]
            [$($token)*]
        }
    };
    (
        @constrained_param [:]
        [[$($gparam:tt)*] [$($rparam:tt)*]]
        [$($constraint:tt)*] [$($value:tt)*]
        [$callback:path]
        [$($callback_args:tt)*]
        [$($g:tt)*]
        [$($r:tt)*]
        [ = $($token:tt)*]
    ) => {
        $crate::parse_generics_impl! {
            @constrained_param [=]
            [[$($gparam)*] [$($rparam)*]]
            [$($constraint)*] [$($value)*]
            [$callback] [$($callback_args)*] [$($g)*] [$($r)*]
            [$($token)*]
        }
    };
    (
        @constrained_param [:]
        [[$($gparam:tt)*] [$($rparam:tt)*]]
        [$($constraint:tt)*] [$($value:tt)*]
        [$callback:path]
        [$($callback_args:tt)*]
        [$($g:tt)*]
        [$($r:tt)*]
        [ $x:tt $($token:tt)*]
    ) => {
        $crate::parse_generics_impl! {
            @constrained_param [:]
            [[$($gparam)*] [$($rparam)*]]
            [$($constraint)* $x] [$($value)*]
            [$callback] [$($callback_args)*] [$($g)*] [$($r)*]
            [$($token)*]
        }
    };
    (
        @constrained_param [=]
        [[$($gparam:tt)*] [$($rparam:tt)*]]
        [$($constraint:tt)*] [$($value:tt)*]
        [$callback:path]
        [$($callback_args:tt)*]
        [$($g:tt)*]
        [$($r:tt)*]
        [ $x:tt $($token:tt)*]
    ) => {
        $crate::parse_generics_impl! {
            @constrained_param [=]
            [[$($gparam)*] [$($rparam)*]]
            [$($constraint)*] [$($value)* $x]
            [$callback] [$($callback_args)*] [$($g)*] [$($r)*]
            [$($token)*]
        }
    };
    (
        @constrained_param [$kind:tt]
        [[$($gparam:tt)*] [$($rparam:tt)*]]
        [$($($constraint:tt)+)?] [$($($value:tt)+)?]
        [$callback:path]
        [$($callback_args:tt)*]
        [$($g:tt)*]
        [$($r:tt)*]
        []
    ) => {
        $crate::std_compile_error!($crate::std_concat!(
            "missing '>' after '",
            $crate::std_stringify!( < $($($($g)*),+ ,)? $($gparam)* $( : $($constraint)+)? $( = $($value)+)? ),
            "'"
        ));
    };
    (
        @angles_in_constraint [:]
        [[$($gparam:tt)*] [$($rparam:tt)*]]
        [$($constraint:tt)*] [$($value:tt)*]
        [$($inside_angles:tt)*]
        []
        [$callback:path]
        [$($callback_args:tt)*]
        [$($g:tt)*]
        [$($r:tt)*]
        [ > $($token:tt)*]
    ) => {
        $crate::parse_generics_impl! {
            @constrained_param [:]
            [[$($gparam)*] [$($rparam)*]]
            [$($constraint)* < $($inside_angles)* > ] [$($value)*]
            [$callback] [$($callback_args)*] [$($g)*] [$($r)*]
            [$($token)*]
        }
    };
    (
        @angles_in_constraint [=]
        [[$($gparam:tt)*] [$($rparam:tt)*]]
        [$($constraint:tt)*] [$($value:tt)*]
        [$($inside_angles:tt)*]
        []
        [$callback:path]
        [$($callback_args:tt)*]
        [$($g:tt)*]
        [$($r:tt)*]
        [ > $($token:tt)*]
    ) => {
        $crate::parse_generics_impl! {
            @constrained_param [=]
            [[$($gparam)*] [$($rparam)*]]
            [$($constraint)*] [$($value)* < $($inside_angles)* > ]
            [$callback] [$($callback_args)*] [$($g)*] [$($r)*]
            [$($token)*]
        }
    };
    (
        @angles_in_constraint [:]
        [[$($gparam:tt)*] [$($rparam:tt)*]]
        [$($constraint:tt)*] [$($value:tt)*]
        [$($inside_angles:tt)*]
        []
        [$callback:path]
        [$($callback_args:tt)*]
        [$($g:tt)*]
        [$($r:tt)*]
        [ >> $($token:tt)*]
    ) => {
        $crate::parse_generics_impl! {
            @constrained_param [:]
            [[$($gparam)*] [$($rparam)*]]
            [$($constraint)* < $($inside_angles)* > ] [$($value)*]
            [$callback] [$($callback_args)*] [$($g)*] [$($r)*]
            [ > $($token)*]
        }
    };
    (
        @angles_in_constraint [=]
        [[$($gparam:tt)*] [$($rparam:tt)*]]
        [$($constraint:tt)*] [$($value:tt)*]
        [$($inside_angles:tt)*]
        []
        [$callback:path]
        [$($callback_args:tt)*]
        [$($g:tt)*]
        [$($r:tt)*]
        [ >> $($token:tt)*]
    ) => {
        $crate::parse_generics_impl! {
            @constrained_param [=]
            [[$($gparam)*] [$($rparam)*]]
            [$($constraint)*] [$($value)* < $($inside_angles)* > ]
            [$callback] [$($callback_args)*] [$($g)*] [$($r)*]
            [ > $($token)*]
        }
    };
    (
        @angles_in_constraint [$kind:tt]
        [[$($gparam:tt)*] [$($rparam:tt)*]]
        [$($constraint:tt)*] [$($value:tt)*]
        [$($inside_angles:tt)*]
        [[$($parent_level:tt)*] $([$($outer_levels:tt)*])*]
        [$callback:path]
        [$($callback_args:tt)*]
        [$($g:tt)*]
        [$($r:tt)*]
        [ > $($token:tt)*]
    ) => {
        $crate::parse_generics_impl! {
            @angles_in_constraint [$kind]
            [[$($gparam)*] [$($rparam)*]]
            [$($constraint)*] [$($value)*]
            [$($parent_level)* < $($inside_angles)* > ]
            [$([$($outer_levels)*])*]
            [$callback] [$($callback_args)*] [$($g)*] [$($r)*]
            [$($token)*]
        }
    };
    (
        @angles_in_constraint [$kind:tt]
        [[$($gparam:tt)*] [$($rparam:tt)*]]
        [$($constraint:tt)*] [$($value:tt)*]
        [$($inside_angles:tt)*]
        [[$($parent_level:tt)*] $([$($outer_levels:tt)*])*]
        [$callback:path]
        [$($callback_args:tt)*]
        [$($g:tt)*]
        [$($r:tt)*]
        [ >> $($token:tt)*]
    ) => {
        $crate::parse_generics_impl! {
            @angles_in_constraint [$kind]
            [[$($gparam)*] [$($rparam)*]]
            [$($constraint)*] [$($value)*]
            [$($parent_level)* < $($inside_angles)* > ]
            [$([$($outer_levels)*])*]
            [$callback] [$($callback_args)*] [$($g)*] [$($r)*] 
            [ > $($token)*]
        }
    };
    (
        @angles_in_constraint [$kind:tt]
        [[$($gparam:tt)*] [$($rparam:tt)*]]
        [$($constraint:tt)*] [$($value:tt)*]
        [$($inside_angles:tt)*]
        [$([$($outer_levels:tt)*])*]
        [$callback:path]
        [$($callback_args:tt)*]
        [$($g:tt)*]
        [$($r:tt)*]
        [ < $($token:tt)*]
    ) => {
        $crate::parse_generics_impl! {
            @angles_in_constraint [$kind]
            [[$($gparam)*] [$($rparam)*]]
            [$($constraint)*] [$($value)*]
            []
            [[$($inside_angles:tt)*] $([$($outer_levels)*])*]
            [$callback] [$($callback_args)*] [$($g)*] [$($r)*]
            [$($token)*]
        }
    };
    (
        @angles_in_constraint [$kind:tt]
        [[$($gparam:tt)*] [$($rparam:tt)*]]
        [$($constraint:tt)*] [$($value:tt)*]
        [$($inside_angles:tt)*]
        [$([$($outer_levels:tt)*])*]
        [$callback:path]
        [$($callback_args:tt)*]
        [$($g:tt)*]
        [$($r:tt)*]
        [ << $($token:tt)*]
    ) => {
        $crate::parse_generics_impl! {
            @angles_in_constraint [$kind]
            [[$($gparam)*] [$($rparam)*]]
            [$($constraint)*] [$($value)*]
            []
            [[$($inside_angles:tt)*] $([$($outer_levels)*])*]
            [$callback] [$($callback_args)*] [$($g)*] [$($r)*] 
            [ < $($token)*]
        }
    };
    (
        @angles_in_constraint [$kind:tt]
        [[$($gparam:tt)*] [$($rparam:tt)*]]
        [$($constraint:tt)*] [$($value:tt)*]
        [$($inside_angles:tt)*]
        [$([$($outer_levels:tt)*])*]
        [$callback:path]
        [$($callback_args:tt)*]
        [$($g:tt)*]
        [$($r:tt)*]
        [$x:tt $($token:tt)*]
    ) => {
        $crate::parse_generics_impl! {
            @angles_in_constraint [$kind]
            [[$($gparam)*] [$($rparam)*]]
            [$($constraint)*] [$($value)*]
            [$($inside_angles)* $x]
            [$([$($outer_levels)*])*]
            [$callback] [$($callback_args)*] [$($g)*] [$($r)*] 
            [$($token)*]
        }
    };
    (
        @angles_in_constraint [:]
        [[$($gparam:tt)*] [$($rparam:tt)*]]
        [$($constraint:tt)*] [$($($value:tt)+)?]
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
                < $($($($g)*),+ ,)? $($gparam)*
                : $($constraint)* $( < $($outer_levels)* )* < $($inside_angles)*
                $( = $($value)+)?
            ),
            "'"
        ));
    };
    (
        @angles_in_constraint [=]
        [[$($gparam:tt)*] [$($rparam:tt)*]]
        [$($($constraint:tt)+)?] [$($value:tt)*]
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
                < $($($($g)*),+ ,)? $($gparam)*
                $( : $($constraint)+)?
                = $($value)* $( < $($outer_levels)* )* < $($inside_angles)*
            ),
            "'"
        ));
    };
    (
        @done
        [$callback:path]
        [$($callback_args:tt)*]
        [$($g:tt)+]
        [$($r:tt)+]
        [$($inter:tt)*]
        [ ; $($token:tt)*]
    ) => {
        $callback ! {
            $($callback_args)*
            [
                [ [$($g)*] ]
                [ [$($r)*] ]
                []
                $crate $crate
            ]
            $($inter)* ; $($token)*
        }
    };
    (
        @done
        [$callback:path]
        [$($callback_args:tt)*]
        [$($g:tt)+]
        [$($r:tt)+]
        [$($inter:tt)*]
        [ $( { $($body:tt)* } $($token:tt)* )? ]
    ) => {
        $callback ! {
            $($callback_args)*
            [
                [ [$($g)*] ]
                [ [$($r)*] ]
                []
                $crate $crate
            ]
            $($inter)* $( { $($body)* } $($token)* )?
        }
    };
    (
        @done
        [$callback:path]
        [$($callback_args:tt)*]
        [$($g:tt)+]
        [$($r:tt)+]
        [$($inter:tt)*]
        [where $($token:tt)*]
    ) => {
        $crate::parse_where_clause_impl! {
            [$callback]
            [$($callback_args)*]
            [ [$($g)*] ]
            [ [$($r)*] ]
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
            [
                [$($g)*]
                [$($r)*]
                [$(where $($w)+)?]
                $crate $crate
            ]
            $($inter)* ; $($token)*
        }
    };
    (
        [$callback:path]
        [$($callback_args:tt)*]
        [$($g:tt)*] [$($r:tt)*]
        [$($($w:tt)+)?]
        [$($inter:tt)*] 
        [ $( { $($body:tt)* } $($token:tt)* )? ]
    ) => {
        $callback ! { 
            $($callback_args)*
            [
                [$($g)*]
                [$($r)*]
                [$(where $($w)+)?]
                $crate $crate
            ]
            $($inter)* $( { $($body)* } $($token)* )?
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
            [
                []
                []
                []
                $crate $crate
            ]
            $($inter)* ; $($token)*
        }
    };
    (
        [$callback:path]
        [$($callback_args:tt)*]
        [$($inter:tt)*]
        [ $( { $($body:tt)* } $($token:tt)* )? ]
    ) => {
        $callback ! {
            $($callback_args)*
            [
                []
                []
                []
                $crate $crate
            ]
            $($inter)* $( { $($body)* } $($token)* )?
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
}

/// Concats several [`parse`](parse) calls results together.
#[macro_export]
macro_rules! concat {
    (
        $callback:path { $($callback_args:tt)* }
        $($([$($g:tt)*] [$($r:tt)*] [$($w:tt)*]),+ $(,)?)?
    ) => {
        $crate::concat_impl! {
            [$callback] [$($callback_args)*]
            [$($([$($g)*])+)?] [$($([$($r)*])+)?] [$($([$($w)*])+)?]
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! concat_impl {
    (
        [$callback:path] [$($callback_args:tt)*]
        [$($g:tt)*] [$($r:tt)*] [$($w:tt)*]
    ) => {
        $crate::concat_g_impl! {
            @list
            [$crate::concat_impl] [@g [$callback] [$($callback_args)*] [$($r)*] [$($w)*]]
            [$($g)*]
            [] []
        }
    };
    (
        @g
        [$callback:path] [$($callback_args:tt)*] [$($r:tt)*] [$($w:tt)*]
        [$($g:tt)*]
    ) => {
        $crate::concat_r_impl! {
            @list
            [$crate::concat_impl] [@r [$callback] [$($callback_args)*] [$($g)*] [$($w)*]]
            [$($r)*]
            [] []
        }
    };
    (
        @r
        [$callback:path] [$($callback_args:tt)*] [$($g:tt)*] [$($w:tt)*]
        [$($r:tt)*]
    ) => {
        $crate::concat_w_impl! {
            @list
            [$crate::concat_impl] [@w [$callback] [$($callback_args)*] [$($g)*] [$($r)*]]
            [$($w)*]
            []
        }
    };
    (
        @w
        [$callback:path] [$($callback_args:tt)*] [$($g:tt)*] [$($r:tt)*]
        [$($w:tt)*]
    ) => {
        $callback ! {
            $($callback_args)*
            [$($g)*] [$($r)*] [$($w)*]
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! concat_g_impl {
    (
        @list
        [$callback:path] [$($callback_args:tt)*]
        [[] $($list:tt)*]
        [$($lifetimes:tt)*] [$($types:tt)*]
    ) => {
        $crate::concat_g_impl! {
            @list
            [$callback] [$($callback_args)*]
            [$($list)*]
            [$($lifetimes)*] [$($types)*]
        }
    };
    (
        @list
        [$callback:path] [$($callback_args:tt)*]
        [[ < $($item:tt)* ] $($list:tt)*]
        [$($lifetimes:tt)*] [$($types:tt)*]
    ) => {
        $crate::concat_g_impl! {
            @item
            [$callback] [$($callback_args)*] [$($list)*]
            [$($lifetimes)*] [$($types)*]
            []
            [$($item)*]
        }
    };
    (
        @list
        [$callback:path] [$($callback_args:tt)*]
        [[$($item:tt)*] $($list:tt)*]
        [$($lifetimes:tt)*] [$($types:tt)*]
    ) => {
        $crate::std_compile_error!($crate::std_concat!(
            "invalid generics '",
            $crate::std_stringify!($($item:tt)*),
            "'"
        ));
    };
    (
        @list
        [$callback:path] [$($callback_args:tt)*]
        []
        [] []
    ) => {
        $callback ! {
            $($callback_args)*
            []
        }
    };
    (
        @list
        [$callback:path] [$($callback_args:tt)*]
        []
        [$([$($lifetime:tt)*])+] []
    ) => {
        $callback ! {
            $($callback_args)*
            [ < $($($lifetime)*),+ > ]
        }
    };
    (
        @list
        [$callback:path] [$($callback_args:tt)*]
        []
        [] [$([$($ty:tt)*])+]
    ) => {
        $callback ! {
            $($callback_args)*
            [ < $($($ty)*),+ > ]
        }
    };
    (
        @list
        [$callback:path] [$($callback_args:tt)*]
        []
        [$([$($lifetime:tt)*])+] [$([$($ty:tt)*])+]
    ) => {
        $callback ! {
            $($callback_args)*
            [ < $($($lifetime)*),+ , $($($ty)*),+ > ]
        }
    };
    (
        @item
        [$callback:path] [$($callback_args:tt)*] [$($list:tt)*]
        [$($lifetimes:tt)*] [$($types:tt)*]
        [$lifetime:lifetime $($constraint:tt)*]
        [, $($tail:tt)*]
    ) => {
        $crate::concat_g_impl! {
            @item
            [$callback] [$($callback_args)*] [$($list)*]
            [$($lifetimes)* [$lifetime $($constraint)*]] [$($types)*]
            []
            [$($tail)*]
        }
    };
    (
        @item
        [$callback:path] [$($callback_args:tt)*] [$($list:tt)*]
        [$($lifetimes:tt)*] [$($types:tt)*]
        [$ty:ident $($constraint:tt)*]
        [, $($tail:tt)*]
    ) => {
        $crate::concat_g_impl! {
            @item
            [$callback] [$($callback_args)*] [$($list)*]
            [$($lifetimes)*] [$($types)* [$ty $($constraint)*]]
            []
            [$($tail)*]
        }
    };
    (
        @item
        [$callback:path] [$($callback_args:tt)*] [$($list:tt)*]
        [$($lifetimes:tt)*] [$($types:tt)*]
        [$($param:tt)*]
        [ < $($tail:tt)*]
    ) => {
        $crate::concat_g_impl! {
            @angles
            [$callback] [$($callback_args)*] [$($list)*]
            [$($lifetimes)*] [$($types)*] [$($param)*]
            []
            []
            [$($tail)*]
        }
    };
    (
        @item
        [$callback:path] [$($callback_args:tt)*] [$($list:tt)*]
        [$($lifetimes:tt)*] [$($types:tt)*]
        [$lifetime:lifetime $($constraint:tt)*]
        [ > ]
    ) => {
        $crate::concat_g_impl! {
            @list
            [$callback] [$($callback_args)*]
            [$($list)*]
            [$($lifetimes)* [$lifetime $($constraint)*]] [$($types)*]
        }
    };
    (
        @item
        [$callback:path] [$($callback_args:tt)*] [$($list:tt)*]
        [$($lifetimes:tt)*] [$($types:tt)*]
        [$ty:ident $($constraint:tt)*]
        [ > ]
    ) => {
        $crate::concat_g_impl! {
            @list
            [$callback] [$($callback_args)*]
            [$($list)*]
            [$($lifetimes)*] [$($types)* [$ty $($constraint)*]]
        }
    };
    (
        @item
        [$callback:path] [$($callback_args:tt)*] [$($list:tt)*]
        [$($lifetimes:tt)*] [$($types:tt)*]
        []
        [ > ]
    ) => {
        $crate::concat_g_impl! {
            @list
            [$callback] [$($callback_args)*]
            [$($list)*]
            [$($lifetimes)*] [$($types)*]
        }
    };
    (
        @item
        [$callback:path] [$($callback_args:tt)*] [$($list:tt)*]
        [$($lifetimes:tt)*] [$($types:tt)*]
        [$($param:tt)*]
        [ > $($tail:tt)+ ]
    ) => {
        $crate::std_compile_error!($crate::std_concat!(
            "invalid generics '",
            $crate::std_stringify!($($tail:tt)+),
            "'"
        ));
    };
    (
        @item
        [$callback:path] [$($callback_args:tt)*] [$($list:tt)*]
        [$($lifetimes:tt)*] [$($types:tt)*]
        [$($param:tt)*]
        [$token:tt $($tail:tt)*]
    ) => {
        $crate::concat_g_impl! {
            @item
            [$callback] [$($callback_args)*] [$($list)*]
            [$($lifetimes)*] [$($types)*]
            [$($param)* $token]
            [$($tail)*]
        }
    };
    (
        @item
        [$callback:path] [$($callback_args:tt)*] [$($list:tt)*]
        [$($lifetimes:tt)*] [$($types:tt)*]
        [$($param:tt)*]
        []
    ) => {
        $crate::std_compile_error!("invalid generics");
    };
    (
        @angles
        [$callback:path] [$($callback_args:tt)*] [$($list:tt)*]
        [$($lifetimes:tt)*] [$($types:tt)*] [$($param:tt)*]
        [$($outer_levels:tt)*]
        [$($content:tt)*]
        [ < $($tail:tt)*]
    ) => {
        $crate::concat_g_impl! {
            @angles
            [$callback] [$($callback_args)*] [$($list)*]
            [$($lifetimes)*] [$($types)*] [$($param)*]
            [[$($content)*] $($outer_levels)*]
            []
            [$($tail)*]
        }
    };
    (
        @angles
        [$callback:path] [$($callback_args:tt)*] [$($list:tt)*]
        [$($lifetimes:tt)*] [$($types:tt)*] [$($param:tt)*]
        [[$($outer_level:tt)*] $($other_outer_levels:tt)*]
        [$($content:tt)*]
        [ > $($tail:tt)*]
    ) => {
        $crate::concat_g_impl! {
            @angles
            [$callback] [$($callback_args)*] [$($list)*]
            [$($lifetimes)*] [$($types)*] [$($param)*]
            [$($other_outer_levels)*]
            [$($outer_level)* < $($content)* > ]
            [$($tail)*]
        }
    };
    (
        @angles
        [$callback:path] [$($callback_args:tt)*] [$($list:tt)*]
        [$($lifetimes:tt)*] [$($types:tt)*] [$($param:tt)*]
        []
        [$($content:tt)*]
        [ > $($tail:tt)*]
    ) => {
        $crate::concat_g_impl! {
            @item
            [$callback] [$($callback_args)*] [$($list)*]
            [$($lifetimes)*] [$($types)*]
            [$($param)* < $($content)* > ]
            [$($tail)*]
        }
    };
    (
        @angles
        [$callback:path] [$($callback_args:tt)*] [$($list:tt)*]
        [$($lifetimes:tt)*] [$($types:tt)*] [$($param:tt)*]
        [$($outer_levels:tt)*]
        [$($content:tt)*]
        [ $token:tt $($tail:tt)*]
    ) => {
        $crate::concat_g_impl! {
            @angles
            [$callback] [$($callback_args)*] [$($list)*]
            [$($lifetimes)*] [$($types)*] [$($param)*]
            [$($outer_levels)*]
            [$($content)* $token]
            [$($tail)*]
        }
    };
    (
        @angles
        [$callback:path] [$($callback_args:tt)*] [$($list:tt)*]
        [$($lifetimes:tt)*] [$($types:tt)*] [$($param:tt)*]
        [$($outer_levels:tt)*]
        [$($content:tt)*]
        []
    ) => {
        $crate::std_compile_error!("invalid generics");
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! concat_r_impl {
    (
        @list
        [$callback:path] [$($callback_args:tt)*]
        [[] $($list:tt)*]
        [$($lifetimes:tt)*] [$($types:tt)*]
    ) => {
        $crate::concat_r_impl! {
            @list
            [$callback] [$($callback_args)*]
            [$($list)*]
            [$($lifetimes)*] [$($types)*]
        }
    };
    (
        @list
        [$callback:path] [$($callback_args:tt)*]
        [[ < $($item:tt)* ] $($list:tt)*]
        [$($lifetimes:tt)*] [$($types:tt)*]
    ) => {
        $crate::concat_r_impl! {
            @item
            [$callback] [$($callback_args)*] [$($list)*]
            [$($lifetimes)*] [$($types)*]
            [, $($item)*]
        }
    };
    (
        @list
        [$callback:path] [$($callback_args:tt)*]
        [[$($item:tt)*] $($list:tt)*]
        [$($lifetimes:tt)*] [$($types:tt)*]
    ) => {
        $crate::std_compile_error!("invalid generics");
    };
    (
        @list
        [$callback:path] [$($callback_args:tt)*]
        []
        [] []
    ) => {
        $callback ! {
            $($callback_args)*
            []
        }
    };
    (
        @list
        [$callback:path] [$($callback_args:tt)*]
        []
        [$([$($lifetime:tt)*])+] []
    ) => {
        $callback ! {
            $($callback_args)*
            [ < $($($lifetime)*),+ > ]
        }
    };
    (
        @list
        [$callback:path] [$($callback_args:tt)*]
        []
        [] [$([$($ty:tt)*])+]
    ) => {
        $callback ! {
            $($callback_args)*
            [ < $($($ty)*),+ > ]
        }
    };
    (
        @list
        [$callback:path] [$($callback_args:tt)*]
        []
        [$([$($lifetime:tt)*])+] [$([$($ty:tt)*])+]
    ) => {
        $callback ! {
            $($callback_args)*
            [ < $($($lifetime)*),+ , $($($ty)*),+ > ]
        }
    };
    (
        @item
        [$callback:path] [$($callback_args:tt)*] [$($list:tt)*]
        [$($lifetimes:tt)*] [$($types:tt)*]
        [, $lifetime:lifetime $($tail:tt)*]
    ) => {
        $crate::concat_r_impl! {
            @item
            [$callback] [$($callback_args)*] [$($list)*]
            [$($lifetimes)* [$lifetime]] [$($types)*]
            [$($tail)*]
        }
    };
    (
        @item
        [$callback:path] [$($callback_args:tt)*] [$($list:tt)*]
        [$($lifetimes:tt)*] [$($types:tt)*]
        [, $ty:ident $($tail:tt)*]
    ) => {
        $crate::concat_r_impl! {
            @item
            [$callback] [$($callback_args)*] [$($list)*]
            [$($lifetimes)*] [$($types)* [$ty]]
            [$($tail)*]
        }
    };
    (
        @item
        [$callback:path] [$($callback_args:tt)*] [$($list:tt)*]
        [$($lifetimes:tt)*] [$($types:tt)*]
        [ $(,)? > ]
    ) => {
        $crate::concat_r_impl! {
            @list
            [$callback] [$($callback_args)*]
            [$($list)*]
            [$($lifetimes)*] [$($types)*]
        }
    };
    (
        @item
        [$callback:path] [$($callback_args:tt)*] [$($list:tt)*]
        [$($lifetimes:tt)*] [$($types:tt)*]
        [$($tail:tt)*]
    ) => {
        $crate::std_compile_error!($crate::std_concat!(
            "invalid generics '",
            $crate::std_stringify!($($tail:tt)+),
            "'"
        ));
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! concat_w_impl {
    (
        @list
        [$callback:path] [$($callback_args:tt)*]
        [[] $($list:tt)*]
        [$($w:tt)*]
    ) => {
        $crate::concat_w_impl! {
            @list
            [$callback] [$($callback_args)*]
            [$($list)*]
            [$($w)*]
        }
    };
    (
        @list
        [$callback:path] [$($callback_args:tt)*]
        [[ where ] $($list:tt)*]
        [$($w:tt)*]
    ) => {
        $crate::concat_w_impl! {
            @list
            [$callback] [$($callback_args)*]
            [$($list)*]
            [$($w)*]
        }
    };
    (
        @list
        [$callback:path] [$($callback_args:tt)*]
        [[ where $($item:tt)* ] $($list:tt)*]
        [$($w:tt)*]
    ) => {
        $crate::concat_w_impl! {
            @item
            [$callback] [$($callback_args)*] [$($list)*]
            [$($w)*]
            []
            [$($item)*]
        }
    };
    (
        @list
        [$callback:path] [$($callback_args:tt)*]
        [[$($item:tt)*] $($list:tt)*]
        [$($w:tt)*]
    ) => {
        $crate::std_compile_error!($crate::std_concat!(
            "invalid generics '",
            $crate::std_stringify!($($item:tt)+),
            "'"
        ));
    };
    (
        @list
        [$callback:path] [$($callback_args:tt)*]
        []
        [$($([$($w:tt)*])+)?]
    ) => {
        $callback ! {
            $($callback_args)*
            [$(where $($($w)*),+)?]
        }
    };
    (
        @item
        [$callback:path] [$($callback_args:tt)*] [$($list:tt)*]
        [$($w:tt)*]
        [$($item:tt)*]
        [$(,)?]
    ) => {
        $crate::concat_w_impl! {
            @list
            [$callback] [$($callback_args)*]
            [$($list)*]
            [$($w)* [$($item)*]]
        }
    };
    (
        @item
        [$callback:path] [$($callback_args:tt)*] [$($list:tt)*]
        [$($w:tt)*]
        [$($item:tt)*]
        [$token:tt $($tail:tt)*]
    ) => {
        $crate::concat_w_impl! {
            @item
            [$callback] [$($callback_args)*] [$($list)*]
            [$($w)*]
            [$($item)* $token]
            [$($tail)*]
        }
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

    struct TestGenericStructWithDefaultParameter<T=()>(T);

    impl_test_trait! {
        struct TestGenericStructWithDefaultParameter<T=()>(T);
    }

    struct TestGenericStructWithConstrainedDefaultParameter<T: 'static = ()>(T);

    impl_test_trait! {
        struct TestGenericStructWithConstrainedDefaultParameter<T: 'static = ()>(T);
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
        let test_generic_struct_ = TestGenericStructWithDefaultParameter(());
        let _: &dyn TestTrait = &test_generic_struct_;
        let _ = test_generic_struct_.0;
        let test_generic_struct__ = TestGenericStructWithConstrainedDefaultParameter(());
        let _: &dyn TestTrait = &test_generic_struct__;
        let _ = test_generic_struct__.0;
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

    macro_rules! struct_A {
        (
        ) => {
            concat_g_impl! {
                @list
                [struct_A] [@struct]
                [[ < 'a, 'b > ] [] [ < 'c, 'd, T: 'static, > ]]
                [] []
            }
        };
        (
            @struct [$($g:tt)*]
        ) => {
            struct A $($g)* {
                a: &'a (),
                b: &'b (),
                c: &'c (),
                d: &'d T,
            }
        };
    }

    struct_A!();

    #[test]
    fn run_concat_g_impl() {
        let x = A { a: &(), b: &(), c: &(), d: &0u16 };
        let _ = x.a;
        let _ = x.b;
        let _ = x.c;
        let _ = x.d;
    }

    macro_rules! struct_B {
        (
        ) => {
            concat_r_impl! {
                @list
                [struct_B] [@struct]
                [[ < 'a, 'b > ] [] [ < 'c, 'd, T, > ]]
                [] []
            }
        };
        (
            @struct [$($g:tt)*]
        ) => {
            struct B $($g)* {
                a: &'a (),
                b: &'b (),
                c: &'c (),
                d: &'d T,
            }
        };
    }

    struct_B!();

    #[test]
    fn run_concat_r_impl() {
        let x = B { a: &(), b: &(), c: &(), d: &0u16 };
        let _ = x.a;
        let _ = x.b;
        let _ = x.c;
        let _ = x.d;
    }
}
