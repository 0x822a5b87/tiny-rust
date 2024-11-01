use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::ops::{Deref, DerefMut};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

struct ReferenceCount<Object> {
    object: Object,
    counter: AtomicUsize,
}

impl<T> ReferenceCount<T> {
    pub fn new(object: T) -> ReferenceCount<T> {
        ReferenceCount {
            object,
            counter: AtomicUsize::new(1),
        }
    }

    pub fn incr(&self) {
        self.counter.fetch_add(1, Ordering::Relaxed);
    }

    pub fn decr(&self) {
        self.counter.fetch_sub(1, Ordering::Relaxed);
    }

    pub fn get_count(&self) -> usize {
        self.counter.load(Ordering::Relaxed)
    }
}

impl <T> Drop for ReferenceCount<T> {
    fn drop(&mut self) {
        println!("Dropping reference count: {}", self.get_count());
    }
}

pub struct SharedPointer<T> {
    // Here are some explanations about why we use these techniques:
    // Q: Why do we use Arc instead of directly using RefCell?
    // A: Because we don't know the number of reference in the compiler-time.
    //
    // Q: Why do we use RefCell instead of directly using ReferenceCount?
    // A: We need RefCell because sometimes we are going to incr/decr the reference counter.
    sp: Arc<RefCell<ReferenceCount<T>>>,
}

impl<T> SharedPointer<T> {
    pub fn new(t: T) -> SharedPointer<T> {
        SharedPointer {
            sp: Arc::new(RefCell::new(ReferenceCount::new(t))),
        }
    }

    pub fn use_count(&self) -> usize {
        self.sp.borrow().get_count()
    }
}

// Clone pointer rather than object
impl<T> Clone for SharedPointer<T>
{
    fn clone(&self) -> Self {
        self.sp.borrow().incr();
        SharedPointer {
            sp: Arc::clone(&self.sp),
        }
    }
}

impl<T> Drop for SharedPointer<T> {
    fn drop(&mut self) {
        self.sp.borrow().decr();
        if self.use_count() <= 0 {

        }
    }
}

// impl <T> Copy for SharedPointer<T> where T: Copy {
//
// }


// impl<T> Deref for SharedPointer<T> {
//     type Target = ();
//
//     fn deref(&self) -> &Self::Target {
//         println!("deref");
//         self
//     }
// }
//
// impl<T> DerefMut for SharedPointer<T> {
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         println!("deref");
//         self.rc.decr();
//
//         self
//     }
// }

// impl<T> Borrow<T> for SharedPointer<T> {
//     fn borrow(&self) -> &T {
//         self.rc.borrow_mut().incr();
//         &self.object
//     }
// }
//
// impl<T> BorrowMut<T> for SharedPointer<T> {
//     fn borrow_mut(&mut self) -> &mut T {
//         self.rc.borrow_mut().incr();
//         &mut self.object
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_thread() {
        // TODO test in thread.
    }

    #[test]
    fn test_closure() {
        // TODO test smart pointer in closure
    }

    #[test]
    fn test_object_without_copy() {
        // this is a huge difference between cpp and rust
        // in cpp, copy of SharedPointer will increase the reference count
        // in rust, move doesn't increase the reference count
    }

    #[test]
    fn test_object_with_copy() {
        // TODO test object with copy
    }

    #[test]
    fn test_primitive_type() {
        let sp_0 = SharedPointer::new(10);
        assert_eq!(sp_0.use_count(), 1);

        let sp_1 = sp_0;
        assert_eq!(sp_1.use_count(), 1);

        {
            // reference count must automatically decrement after variable leaves the scope
            let sp_2 = sp_1.clone();
            assert_eq!(sp_1.use_count(), 2);
            assert_eq!(sp_2.use_count(), 2);
        }

        assert_eq!(sp_1.use_count(), 1);
    }

    #[test]
    fn test_reference_count() {
        let mut rc = SharedPointer::new(10);
        assert_eq!(rc.use_count(), 1);
    }
}
