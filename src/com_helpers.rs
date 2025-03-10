use super::*;
use std::{
    mem::forget,
    ops::Deref,
    ptr::{NonNull, null_mut},
};

pub unsafe trait ComVtable: 'static + Copy {
    const IID: v3_iid;
    type Super: ComVtable;
}

#[repr(transparent)]
#[derive(Debug, PartialEq, Eq)]
pub struct Com<T: ComVtable> {
    ptr: NonNull<NonNull<T>>,
}

impl<T: ComVtable> Com<T> {
    /// Creates a new `Com` from an owned pointer. Does not affect the reference count.
    ///
    /// SAFETY: `ptr` must point to a pointer to a valid COM vtable and be safe to dereference.
    pub unsafe fn from_borrowed(ptr: &*mut *mut T) -> &Self {
        unsafe { std::mem::transmute(ptr) }
    }

    /// Creates a new `Com` from an owned pointer. Does not affect the reference count.
    ///
    /// Returns `None` if the pointer (or the inner pointer) is null.
    ///
    /// SAFETY: `ptr` must point to a pointer to a valid COM vtable and be safe to dereference.
    pub unsafe fn from_borrowed_nullable(ptr: &*mut *mut T) -> Option<&Self> {
        unsafe {
            if ptr.is_null() || (*ptr).is_null() {
                None
            } else {
                Some(Self::from_borrowed(ptr))
            }
        }
    }

    /// Creates a new `Com` from an owned pointer. Does not increment the reference count.
    ///
    /// SAFETY: `ptr` must point to a valid COM vtable and be safe to dereference.
    pub unsafe fn from_owned(ptr: *mut *mut T) -> Self {
        unsafe {
            Self {
                ptr: std::mem::transmute(ptr),
            }
        }
    }

    /// Creates a new `Com` from an owned pointer. Does not increment the reference count.
    ///
    /// Returns `None` if the pointer is null.
    ///
    /// SAFETY: `ptr` must point to a valid COM vtable and be safe to dereference.
    pub unsafe fn from_nullable(ptr: *mut *mut T) -> Option<Self> {
        unsafe {
            if ptr.is_null() || (*ptr).is_null() {
                None
            } else {
                Some(Self::from_owned(ptr))
            }
        }
    }

    pub unsafe fn add_ref(&self) {
        unsafe {
            let funknown = self.ptr.as_ptr() as *mut *mut v3_funknown;
            if let Some(add_ref) = (**funknown).add_ref {
                add_ref(funknown as _);
            }
        }
    }

    pub unsafe fn release(&self) {
        unsafe {
            let funknown = self.ptr.as_ptr() as *mut *mut v3_funknown;
            if let Some(release) = (**funknown).release {
                release(funknown as _);
            }
        }
    }

    pub fn as_unknown(&self) -> &Com<v3_funknown> {
        unsafe { std::mem::transmute(&self) }
    }

    pub fn as_super(&self) -> &Com<T::Super> {
        unsafe { std::mem::transmute(&self) }
    }

    pub fn cast<I: ComVtable>(&self) -> Option<Com<I>> {
        unsafe {
            let funknown = self.ptr.as_ptr() as *mut *mut v3_funknown;
            if let Some(query_interface) = (**funknown).query_interface {
                let mut obj = null_mut();
                if query_interface(funknown as _, &I::IID as *const u8, &mut obj) == V3_RESULT_OK
                    && !obj.is_null()
                {
                    Some(Com::from_owned(obj as *mut _))
                } else {
                    None
                }
            } else {
                None
            }
        }
    }

    pub fn as_ptr(&self) -> *mut *mut T {
        self.ptr.as_ptr().cast()
    }

    pub fn into_raw(self) -> *mut *mut T {
        let ptr = self.ptr.as_ptr();
        forget(self);
        ptr.cast()
    }
}

impl<T: ComVtable> Deref for Com<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { self.ptr.as_ref().as_ref() }
    }
}

impl<T: ComVtable> Clone for Com<T> {
    fn clone(&self) -> Self {
        unsafe {
            self.add_ref();
        }

        Self {
            ptr: self.ptr.clone(),
        }
    }
}

impl<T: ComVtable> Drop for Com<T> {
    fn drop(&mut self) {
        unsafe {
            self.release();
        }
    }
}
