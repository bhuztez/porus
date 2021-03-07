use crate::deque::Deque;
use crate::pool::{self, Handle, Pool};
use alloc::alloc::Global;
use core::marker::PhantomData;

struct Link<H: Handle> {
    prev: Option<H>,
    next: Option<H>,
}

pub struct Node<H: Handle, T> {
    link: Link<H>,
    data: T,
}

pub struct DoublyLinkedList<
    T,
    H: Handle = pool::AllocHandle,
    P: Pool<Node<H, T>, Handle = H> = Global,
> {
    pool: P,
    sentinel: Link<H>,
    _data: PhantomData<T>,
}

impl<T, H: Handle, P: Pool<Node<H, T>, Handle = H>> DoublyLinkedList<T, H, P> {
    pub fn new_with_pool(pool: P) -> Self {
        Self {
            pool,
            sentinel: Link {
                prev: None,
                next: None,
            },
            _data: PhantomData,
        }
    }
}

impl<T, H: Handle, P: Pool<Node<H, T>, Handle = H> + Default> DoublyLinkedList<T, H, P> {
    #[must_use]
    pub fn new() -> Self {
        Self::new_with_pool(Default::default())
    }
}

impl<T, H: Handle, P: Pool<Node<H, T>, Handle = H> + Default> Default
    for DoublyLinkedList<T, H, P>
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T, H: Handle, P: Pool<Node<H, T>, Handle = H>> DoublyLinkedList<T, H, P> {
    pub fn front(&self) -> Option<P::Handle> {
        self.sentinel.next
    }

    pub fn back(&self) -> Option<P::Handle> {
        self.sentinel.prev
    }

    fn add_node(&mut self, data: T) -> P::Handle {
        let node = Node {
            link: Link {
                prev: None,
                next: None,
            },
            data,
        };
        pool::add(&mut self.pool, node)
    }

    fn get_link(&self, handle: Option<P::Handle>) -> &Link<P::Handle> {
        match handle {
            None => &self.sentinel,
            Some(h) => &pool::get(&self.pool, h).link,
        }
    }

    fn get_node_mut(&mut self, handle: Option<P::Handle>) -> &mut Link<P::Handle> {
        match handle {
            None => &mut self.sentinel,
            Some(h) => &mut pool::get_mut(&mut self.pool, h).link,
        }
    }

    pub fn prev(&self, handle: Option<P::Handle>) -> Option<P::Handle> {
        self.get_link(handle).prev
    }

    pub fn next(&self, handle: Option<P::Handle>) -> Option<P::Handle> {
        self.get_link(handle).next
    }

    pub fn get(&self, handle: P::Handle) -> &T {
        &pool::get(&self.pool, handle).data
    }

    pub fn get_mut(&mut self, handle: P::Handle) -> &mut T {
        &mut pool::get_mut(&mut self.pool, handle).data
    }

    pub fn insert_before(&mut self, data: T, reference: Option<P::Handle>) -> P::Handle {
        let new = self.add_node(data);
        let prev = self.prev(reference);
        self.get_node_mut(reference).prev = Some(new);
        self.get_node_mut(Some(new)).next = reference;
        self.get_node_mut(Some(new)).prev = prev;
        self.get_node_mut(prev).next = Some(new);
        new
    }

    pub fn insert_after(&mut self, data: T, reference: Option<P::Handle>) -> P::Handle {
        let new = self.add_node(data);
        let next = self.next(reference);
        self.get_node_mut(reference).next = Some(new);
        self.get_node_mut(Some(new)).prev = reference;
        self.get_node_mut(Some(new)).next = next;
        self.get_node_mut(next).prev = Some(new);
        new
    }

    pub fn remove(&mut self, handle: P::Handle) -> T {
        let prev = self.prev(Some(handle));
        let next = self.next(Some(handle));
        self.get_node_mut(prev).next = next;
        self.get_node_mut(next).prev = prev;
        pool::remove(&mut self.pool, handle).data
    }
}

impl<T, H: Handle, P: Pool<Node<H, T>, Handle = H>> Deque for DoublyLinkedList<T, H, P> {
    type Elem = T;

    fn is_empty(&self) -> bool {
        self.front().is_none()
    }

    fn push_front(&mut self, elem: T) {
        let front = self.front();
        self.insert_before(elem, front);
    }

    fn pop_front(&mut self) -> Option<T> {
        self.front().map(|handle| self.remove(handle))
    }

    fn push_back(&mut self, elem: T) {
        let back = self.back();
        self.insert_after(elem, back);
    }

    fn pop_back(&mut self) -> Option<T> {
        self.back().map(|handle| self.remove(handle))
    }

    fn front(&self) -> Option<&T> {
        self.front()
            .map(|handle| &pool::get(&self.pool, handle).data)
    }

    fn back(&self) -> Option<&T> {
        self.back()
            .map(|handle| &pool::get(&self.pool, handle).data)
    }
}
