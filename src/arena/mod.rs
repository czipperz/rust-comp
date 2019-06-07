mod allocator;
pub use allocator::*;

use std::alloc::*;
use std::ptr;

/// An arena memory storage unit.
///
/// This struct stores the arena.  The [`Allocator`] allocates memory in the
/// Arena.  By separating these this way, we are able to separate the lifetime
/// constraints without internal mutability.  Use the method
/// [`allocator`](#method.allocator) to get an [`Allocator`] for the `Arena`.
///
/// [`Allocator`]: struct.Allocator.html
///
/// This will allocate and emplace values of any type.  They will be stored
/// continuously and then be dropped at the end.  These values will never be
/// deallocated!  If a value inserted has a `Drop` impl, it is up to you to run
/// it manually using `std::ptr::drop_in_place` on the return value.
pub struct Arena {
    start: *mut u8,
    point: *mut u8,
    old: Vec<*mut u8>,
    chunk_layout: Layout,
}

impl Arena {
    /// Create a new arena.
    ///
    /// If you want a non-default chunk size, see
    /// [`with_layout`](#method.with_layout).
    ///
    /// Does not allocate any memory.
    pub fn new() -> Self {
        Self::with_layout(Layout::from_size_align(4 * 1024, 128).unwrap())
    }

    /// Create a new arena with chunks of the given alignment and size.
    ///
    /// Any values allocated in the arena must be able to fit within one chunk.
    /// The unsafe methods of [`Allocator`] require manual verification of this
    /// condition, while the safe methods do it automatically.  This should
    /// avoid runtime overhead where possible but will panic if not satisfied.
    ///
    /// [`Allocator`]: struct.Allocator.html
    pub fn with_layout(chunk_layout: Layout) -> Self {
        Arena {
            start: ptr::null_mut(),
            point: ptr::null_mut(),
            old: Vec::new(),
            chunk_layout,
        }
    }

    /// Make an [`Allocator`] for this arena.
    ///
    /// [`Allocator`]: struct.Allocator.html
    pub fn allocator<'a>(&'a mut self) -> Allocator<'a> {
        Allocator::new(self)
    }

    pub(self) unsafe fn alloc(&mut self, layout: Layout) -> *mut u8 {
        debug_assert!(layout.size() <= self.chunk_layout.size());
        debug_assert!(layout.align() <= self.chunk_layout.align());

        if self.start.is_null() {
            self.alloc_chunk();
        } else if self.start.add(self.chunk_layout.size()) < layout_end(self.point, layout) {
            self.old.push(self.start);
            self.alloc_chunk();
        }

        let point = layout_start(self.point, layout);
        self.point = layout_end(self.point, layout);
        point
    }

    unsafe fn alloc_chunk(&mut self) {
        self.start = alloc(self.chunk_layout);
        self.point = self.start;
    }
}

unsafe fn layout_start(ptr: *mut u8, layout: Layout) -> *mut u8 {
    ptr.add(ptr.align_offset(layout.align()))
}

unsafe fn layout_end(ptr: *mut u8, layout: Layout) -> *mut u8 {
    layout_start(ptr, layout).add(layout.size())
}

impl Drop for Arena {
    fn drop(&mut self) {
        if !self.start.is_null() {
            free_chunk(self.start, self.chunk_layout);
        }
        for ptr in &self.old {
            free_chunk(*ptr, self.chunk_layout);
        }
    }
}

fn free_chunk(ptr: *mut u8, chunk_layout: Layout) {
    unsafe { dealloc(ptr, chunk_layout) };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_doesnt_allocate() {
        let arena = Arena::new();
        assert_eq!(arena.start, arena.point);
        assert_eq!(arena.start, ptr::null_mut());
        assert!(arena.old.is_empty());
    }

    #[test]
    fn test_with_layout_doesnt_allocate() {
        let arena = Arena::with_layout(Layout::new::<i32>());
        assert_eq!(arena.start, arena.point);
        assert_eq!(arena.start, ptr::null_mut());
        assert!(arena.old.is_empty());
        assert_eq!(arena.chunk_layout, Layout::new::<i32>());
    }

    #[test]
    fn test_alloc_once() {
        let mut arena = Arena::new();
        let ptr = unsafe { arena.alloc(Layout::new::<i32>()) };
        assert!(!arena.start.is_null());
        assert_eq!(arena.start, ptr);
        assert_eq!(unsafe { arena.start.add(4) }, arena.point);
    }

    #[test]
    fn test_alloc_multiple_same_chunk() {
        let mut arena = Arena::new();
        unsafe { arena.alloc(Layout::new::<i32>()) };
        let ptr = unsafe { arena.alloc(Layout::new::<i32>()) };
        assert!(!arena.start.is_null());
        assert_eq!(unsafe { arena.start.add(4) }, ptr);
        assert_eq!(unsafe { arena.start.add(8) }, arena.point);
    }

    #[test]
    fn test_alloc_multiple_different_chunks() {
        let mut arena = Arena::with_layout(Layout::new::<i32>());

        let ptr1 = unsafe { arena.alloc(Layout::new::<i32>()) };
        assert_eq!(arena.start, ptr1);
        assert_eq!(unsafe { arena.start.add(4) }, arena.point);

        let ptr2 = unsafe { arena.alloc(Layout::new::<i32>()) };
        assert_eq!(arena.start, ptr2);
        assert_eq!(unsafe { arena.start.add(4) }, arena.point);
        assert_ne!(ptr1, ptr2);

        assert!(!arena.start.is_null());
        assert_eq!(&arena.old, &[ptr1]);
    }

    #[test]
    fn test_alloc_multiple_different_chunks_different_sizes() {
        let mut arena = Arena::with_layout(Layout::from_size_align(8, 4).unwrap());

        let ptr1 = unsafe { arena.alloc(Layout::from_size_align(3, 1).unwrap()) };
        let ptr2 = unsafe { arena.alloc(Layout::from_size_align(3, 1).unwrap()) };
        let ptr3 = unsafe { arena.alloc(Layout::from_size_align(3, 1).unwrap()) };

        assert_eq!(&arena.old, &[ptr1]);
        assert_eq!(ptr2, unsafe { ptr1.add(3) });
        assert_eq!(arena.start, ptr3);
        assert_eq!(arena.point, unsafe { ptr3.add(3) });
    }

    #[test]
    fn test_alloc_with_alignment() {
        let mut arena = Arena::with_layout(Layout::from_size_align(8, 4).unwrap());

        let ptr1 = unsafe { arena.alloc(Layout::from_size_align(3, 4).unwrap()) };
        let ptr2 = unsafe { arena.alloc(Layout::from_size_align(3, 4).unwrap()) };

        assert_eq!(unsafe { ptr1.add(4) }, ptr2);
    }

    #[test]
    fn test_alloc_with_alignment_doesnt_pad_end() {
        let mut arena = Arena::with_layout(Layout::from_size_align(7, 4).unwrap());

        let ptr1 = unsafe { arena.alloc(Layout::from_size_align(3, 4).unwrap()) };
        let ptr2 = unsafe { arena.alloc(Layout::from_size_align(3, 4).unwrap()) };

        assert_eq!(unsafe { ptr1.add(4) }, ptr2);
    }

    #[test]
    fn test_alloc_with_alignment_splits_due_to_alignment() {
        let mut arena = Arena::with_layout(Layout::from_size_align(6, 4).unwrap());

        let ptr1 = unsafe { arena.alloc(Layout::from_size_align(3, 4).unwrap()) };
        let ptr2 = unsafe { arena.alloc(Layout::from_size_align(3, 4).unwrap()) };

        assert_ne!(unsafe { ptr1.add(4) }, ptr2);
        assert_eq!(&arena.old, &[ptr1]);
    }
}
