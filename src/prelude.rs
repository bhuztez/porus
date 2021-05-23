pub use alloc::collections::BTreeMap;
pub use alloc::collections::BTreeSet;
pub use alloc::collections::BinaryHeap;
pub use alloc::collections::VecDeque;
pub use alloc::vec;
pub use alloc::vec::Vec;
pub use core::cmp::Ordering::Equal;
pub use core::cmp::Ordering::Greater;
pub use core::cmp::Ordering::Less;

pub use crate::math::*;

#[must_use]
pub fn default<T: Default>() -> T {
    Default::default()
}

pub use crate::fmt::interleave;
pub use porus_macros::{printf, scanf, sscanf};

pub use crate::allocator;
pub use crate::pool::{self, Pool};

pub use crate::chunk::Chunk;

pub use crate::collection::{self, Collection};
pub use crate::deque::{self, Deque};
pub use crate::heap::{self, Heap};
pub use crate::list::sorting;
pub use crate::list::{self, List, ListMut};
pub use crate::set::{self, Set, SetMut};
pub use crate::stack::{self, Stack};

pub use crate::dheap::{self, DHeap};
pub use crate::dlist::DoublyLinkedList;
pub use crate::flist::SinglyLinkedList;
pub use crate::string::{String, StringBuffer};

/// the porus prelude
#[macro_export]
macro_rules! prelude {
    () => {
        #[allow(unused_imports)]
        use $crate::prelude::*;

        pub mod __porus_main {
            #[cfg(feature = "online-judge")]
            #[export_name = "main"]
            pub extern "C" fn porus_start() -> i32 {
                super::main();
                0
            }
        }
    };
    (leetcode) => {
        #[allow(unused_imports)]
        use $crate::prelude::*;
    };
}
