use super::*;
use std::alloc::Layout;
use std::marker::PhantomData;
use std::ptr;

/// An arena memory allocator for a specific [`Arena`].
///
/// An `Allocator` is constructed using the [`allocator`] method of [`Arena`].
/// It will allocate memory inside that arena.  Objects placed in that memory
/// will never be destructed.  Thus if you insert an object implementing `Drop`,
/// it is up to you to run the destructor manually using
/// `std::ptr::drop_in_place`.
///
/// [`Arena`]: struct.Arena.html
/// [`allocator`]: struct.Arena.html#method.allocator
pub struct Allocator<'a> {
    arena: *mut Arena,
    phantom: PhantomData<&'a mut Arena>,
}

impl<'a> Allocator<'a> {
    pub(super) fn new(arena: &'a mut Arena) -> Self {
        Allocator {
            arena,
            phantom: PhantomData,
        }
    }

    /// This will allocate and emplace the value of `t`.
    ///
    /// The variable will never be deallocated.  If it has a `Drop` impl, it is
    /// up to you to run it manually using `std::ptr::drop_in_place` on the
    /// return value.
    ///
    /// # Panics
    ///
    /// This will panic if the size or alignment of the type is greater than
    /// size or alignment of the chunk, respectively.  If you need a larger
    /// [`Arena`], use [`Arena::with_layout`].
    ///
    /// [`Arena`]: struct.Arena.html
    /// [`Arena::with_layout`]: struct.Arena.html#with_layout
    pub fn alloc<T>(&self, t: T) -> &'a mut T {
        let layout = Layout::new::<T>();
        assert!(layout.size() <= self.arena().chunk_layout.size());
        assert!(layout.align() <= self.arena().chunk_layout.align());
        unsafe {
            let buffer = self.alloc_layout(layout) as *mut T;
            ptr::write(buffer, t);
            &mut *buffer
        }
    }

    /// This will allocate aligned memory sized to fit the `Layout`.
    ///
    /// The memory at the pointer's address will be deallocated at the end of
    /// lifetime `'a`, when the `Arena` is dropped.
    #[inline(always)]
    pub unsafe fn alloc_layout(&self, layout: Layout) -> *mut u8 {
        self.arena().alloc(layout)
    }

    #[inline(always)]
    fn arena(&self) -> &'a mut Arena {
        unsafe { &mut *self.arena }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_allocator_new() {
        let mut arena = Arena::new();
        assert!(ptr::eq(&mut arena, arena.allocator().arena));
        assert!(ptr::eq(&mut arena, Allocator::new(&mut arena).arena));
    }

    #[test]
    fn test_alloc_basic() {
        let mut arena = Arena::new();
        let allocator = arena.allocator();
        assert_eq!(*allocator.alloc(3), 3);
    }

    #[test]
    fn test_drop_not_called() {
        use std::cell::Cell;

        struct IncOnDrop<'a> {
            cell: &'a Cell<i32>,
        }

        impl Drop for IncOnDrop<'_> {
            fn drop(&mut self) {
                self.cell.set(self.cell.get() + 1);
            }
        }

        let cell = Cell::default();
        assert_eq!(cell.get(), 0);
        {
            let mut arena = Arena::new();
            let allocator = arena.allocator();
            let inc_on_drop = allocator.alloc(IncOnDrop { cell: &cell });
            assert!(ptr::eq(inc_on_drop.cell, &cell));
            assert_eq!(cell.get(), 0);
        }
        assert_eq!(cell.get(), 0);
    }

    #[test]
    fn test_lifetimes_work() {
        let mut arena = Arena::new();
        let allocator = arena.allocator();
        let x = allocator.alloc(1);
        let y = allocator.alloc(2);
        assert_ne!(*x, *y);
        assert!(!ptr::eq(x, y));
    }
}
