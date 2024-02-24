#![no_std]
#![warn(clippy::pedantic, missing_docs, clippy::cargo)]
#![allow(clippy::wildcard_imports)]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![doc = include_str!("doc.md")]

#[macro_export]
#[doc = include_str!("doc.md")]
macro_rules! non_exhaustive {
    {$type:ty {$($field:ident: $value:expr,)* ..$default:expr}} => {{
        #[allow(unused)]
        let mut value: $type = $default;
        $(value.$field = $value;)*
        value
    }};
    {$type:ty {$($field:ident: $value:expr)* $(,)?}} => {
        $crate::non_exhaustive!($type {$($field: $value,)* ..::core::default::Default::default()})
    };
}

#[cfg(test)]
mod import_resolution {
    use crate::non_exhaustive;

    #[non_exhaustive]
    #[derive(Default)]
    struct Test {}

    #[test]
    fn test() {
        non_exhaustive!(Test {});
    }
}
