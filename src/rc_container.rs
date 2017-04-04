use ::std::rc::{Rc, Weak};
use ::std::cell::{RefCell, Ref, RefMut, BorrowError};
use std::fmt;

pub struct Container<T>(Rc<RefCell<T>>);
impl<T> Container<T> {
    pub fn new(typ: T) -> Container<T> {
        Container(Rc::new(RefCell::new(typ)))
    }

    pub fn borrow(&self) -> Ref<T> {
        self.0.borrow()
    }
    pub fn borrow_mut(&self) -> RefMut<T> {
        self.0.borrow_mut()
    }
    pub fn try_borrow(&self) -> Result<Ref<T>, BorrowError> {
        self.0.try_borrow()
    }

    pub fn downgrade(&self) -> WeakContainer<T> {
        WeakContainer(Rc::downgrade(&self.0))
    }
}

#[derive(Debug)]
pub struct WeakContainer<T>(Weak<RefCell<T>>);
impl<T> WeakContainer<T> {
    pub fn upgrade(&self) -> Container<T> {
        Container(self.0.upgrade().unwrap())
    }
}

impl<T> fmt::Debug for Container<T> where T: fmt::Debug {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self.0.try_borrow() {
            Ok(borrow) =>
                write!(fmt, "Container({:?})", borrow),
            Err(_) =>
                write!(fmt, "Container(<borrowed>)"),
        }
    }
}

use ::std::marker::Sized;

impl<T: Sized> Clone for Container<T> {
    fn clone(&self) -> Container<T> {
        Container(self.0.clone())
    }
}
impl<T: Sized> Clone for WeakContainer<T> {
    fn clone(&self) -> WeakContainer<T> {
        WeakContainer(self.0.clone())
    }
}
