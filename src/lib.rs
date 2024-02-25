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
    {$type:ty {$($field:ident: $value:expr),* $(,)?}} => {
        $crate::non_exhaustive!($type {$($field: $value,)* ..::core::default::Default::default()})
    };
}

#[cfg(test)]
#[allow(clippy::all, unused, clippy::pedantic)]
mod test {
    use crate::non_exhaustive;

    #[non_exhaustive]
    #[derive(Default)]
    struct Test {
        a: usize,
        b: usize,
        c: usize,
    }

    #[test]
    #[rustfmt::skip]
    fn test() {
        non_exhaustive!(Test {});
        non_exhaustive!(Test { a: 1 });
        non_exhaustive!(Test { a: 1, a: 2 });
        non_exhaustive!(Test { a: 1, a: 2, a: 3 });
        non_exhaustive!(Test { a: 1, });
        non_exhaustive!(Test { a: 1, a: 2, });
        non_exhaustive!(Test { a: 1, a: 2, a: 3, });
        non_exhaustive!(Test { ..Default::default() });
        non_exhaustive!(Test { a: 1, ..Default::default() });
        non_exhaustive!(Test { a: 1, a: 2, ..Default::default() });
        non_exhaustive!(Test { a: 1, a: 2, a: 3, ..Default::default() });
    }
}
