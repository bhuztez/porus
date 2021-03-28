use crate::fmt::{self, fwrite_str};
#[allow(unused_imports)]
use crate::fmt::{f, fwrite};
use crate::io::Sink;
use alloc::alloc::{Allocator, Global, Layout};
use core::cmp::Ordering;
use core::mem::size_of;
use core::ops::Deref;
use core::ptr::NonNull;
use core::slice::from_raw_parts;
use core::str;
use core::hint::unreachable_unchecked;

mod buffer;
pub use self::buffer::Buffer as StringBuffer;

#[cfg(target_endian = "little")]
#[derive(Clone, Copy)]
struct SharedString {
    counter: NonNull<usize>,
    length: usize,
    s: NonNull<u8>,
}

#[cfg(all(target_endian = "little", target_pointer_width = "64"))]
#[derive(Clone, Copy)]
struct InlineString {
    length: u8,
    s: [u8; 23],
}

#[cfg(target_endian = "little")]
#[derive(Clone, Copy)]
struct StaticString {
    _padding: usize,
    length: usize,
    s: *const u8,
}

#[cfg(target_endian = "big")]
#[derive(Clone, Copy)]
struct SharedString {
    s: NonNull<u8>,
    length: usize,
    counter: NonNull<usize>,
}

#[cfg(all(target_endian = "big", target_pointer_width = "64"))]
#[derive(Clone, Copy)]
struct InlineString {
    s: [u8; 23],
    length: u8,
}

#[cfg(target_endian = "big")]
#[derive(Clone, Copy)]
struct StaticString {
    s: *const u8,
    length: usize,
    _padding: usize,
}

union Union {
    shared: SharedString,
    inline: InlineString,
    static_: StaticString,
}

enum Tag {
    Shared,
    Inline,
    Static,
}

impl Union {
    fn tag(&self) -> Tag {
        match unsafe { self.inline.length & 0x3 } {
            0 => Tag::Shared,
            1 => Tag::Inline,
            2 => Tag::Static,
            _ => unsafe { unreachable_unchecked() },
        }
    }

    fn len(&self) -> usize {
        unsafe {
            match self.tag() {
                Tag::Shared => self.shared.length,
                Tag::Inline => u8::wrapping_shr(self.inline.length, 2) as usize,
                Tag::Static => self.static_.length,
            }
        }
    }

    unsafe fn as_ptr(&self) -> *const u8 {
        match self.tag() {
            Tag::Shared => self.shared.s.as_ptr(),
            Tag::Inline => self.inline.s.as_ptr(),
            Tag::Static => self.static_.s,
        }
    }

    fn as_bytes(&self) -> &[u8] {
        unsafe { from_raw_parts(self.as_ptr(), self.len()) }
    }
}

impl Clone for Union {
    fn clone(&self) -> Self {
        unsafe {
            match self.tag() {
                Tag::Shared => {
                    *self.shared.counter.as_ptr() =
                        usize::wrapping_add(*self.shared.counter.as_ref(), 1);
                    Self {
                        shared: Clone::clone(&self.shared),
                    }
                }
                Tag::Inline => Self {
                    inline: Clone::clone(&self.inline),
                },
                Tag::Static => Self {
                    static_: Clone::clone(&self.static_),
                },
            }
        }
    }
}

pub struct String<A: Allocator = Global> {
    s: Union,
    allocator: A,
}

impl<A: Allocator + Default> From<&'static [u8]> for String<A> {
    fn from(s: &'static [u8]) -> Self {
        Self {
            s: Union {
                static_: StaticString {
                    s: s.as_ptr(),
                    length: s.len(),
                    _padding: 2,
                },
            },
            allocator: Default::default(),
        }
    }
}

impl<A: Allocator> Deref for String<A> {
    type Target = str;

    #[inline]
    fn deref(&self) -> &str {
        unsafe { str::from_utf8_unchecked(self.as_ref()) }
    }
}

impl<A: Allocator> AsRef<[u8]> for String<A> {
    fn as_ref(&self) -> &[u8] {
        self.s.as_bytes()
    }
}

impl<A: Allocator> PartialEq for String<A> {
    fn eq(&self, other: &Self) -> bool {
        PartialEq::eq(self.as_ref(), other.as_ref())
    }
}

impl<A: Allocator> PartialOrd for String<A> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        PartialOrd::partial_cmp(self.as_ref(), other.as_ref())
    }
}

impl<A: Allocator + Clone> Clone for String<A> {
    fn clone(&self) -> Self {
        Self {
            s: Clone::clone(&self.s),
            allocator: Clone::clone(&self.allocator),
        }
    }
}

impl<A: Allocator> Drop for String<A> {
    fn drop(&mut self) {
        if let Tag::Shared = self.s.tag() {
            unsafe {
                if let Some(c) = usize::checked_sub(*self.s.shared.counter.as_ref(), 1) {
                    *self.s.shared.counter.as_mut() = c;
                } else {
                    Allocator::deallocate(
                        &self.allocator,
                        self.s.shared.counter.cast(),
                        Layout::array::<u8>(usize::wrapping_add(
                            size_of::<usize>(),
                            self.s.shared.length,
                        ))
                        .unwrap(),
                    );
                }
            }
        }
    }
}

impl<'a> fmt::String for &'a String {
    fn write<S: Sink>(self, s: &mut S) {
        fwrite_str(s, self);
    }
}

/// Create [`String`](string::String) using interpolation of runtime
/// expressions, i.e. alternative to `format!` in `std!`.
///
/// # Examples
/// ```
/// # use porus::prelude::*;
/// assert_eq!(b"test", stringf!("test").as_ref());
/// assert_eq!(b"hello world", stringf!("hello {:s}", "world").as_ref());
/// assert_eq!(b"x = 10, y = 30", stringf!("x = {:d}, y = {:d}", 10, 30).as_ref());
/// ```
pub macro stringf($($arg:tt)*) {
    {
        let mut buffer: StringBuffer = Default::default();
        fwrite(&mut buffer, &mut f!($($arg)*));
        let string: String = From::from(buffer);
        string
    }
}

#[cfg(test)]
mod tests {
    use super::{String, StringBuffer};
    use crate::scan::fread;

    #[test]
    fn test_inline_string_buffer() {
        let source = &mut From::from(b"abc " as &_);
        let mut buffer = <StringBuffer as Default>::default();
        fread(source, &mut buffer);
        let s1: String = From::from(buffer);
        let s2: String = From::from(b"abc" as &'static [u8]);
        assert!(s1 == s2);
    }

    #[test]
    fn test_shared_string_buffer() {
        let source = &mut From::from(b"abcdefghijklmnopqrstuvwxyz" as &_);
        let mut buffer = <StringBuffer as Default>::default();
        fread(source, &mut buffer);
        let s1: String = From::from(buffer);
        let s2: String = From::from(b"abcdefghijklmnopqrstuvwxyz" as &'static [u8]);
        assert!(s1 == s2);
    }
}
