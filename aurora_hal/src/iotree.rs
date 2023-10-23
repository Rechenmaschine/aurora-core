pub trait IoTree {
    fn get_tree() -> &'static Self;
}

#[macro_export]
macro_rules! init_io_tree {
    ($($field_name:ident: $field_type:ident),+) => {
        mod __iotree {
            use std::sync::OnceLock;
            $(
            use super::$field_type;
            )+
            use $crate::iotree::IoTree;

            #[derive(Default)]
            pub(crate) struct __IoTree {
                $(
                pub $field_name: $field_type
                ),+
            }

            static __IO_TREE: OnceLock<__IoTree> = OnceLock::new();

            pub(crate) fn get_io_tree() -> &'static __IoTree {
                __IO_TREE.get_or_init(__IoTree::default)
            }

            impl IoTree for __IoTree {
                fn get_tree() -> &'static Self {
                    get_io_tree()
                }
            }
        }

        pub(crate) use __iotree::get_io_tree;
    };
}
