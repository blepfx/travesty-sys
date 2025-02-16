use super::*;
use std::ptr::{addr_of, null_mut, NonNull};

pub unsafe trait ComVtable: 'static + Copy {
    const IID: v3_iid;
    type Super: ComVtable;
}

#[repr(transparent)]
#[derive(Debug, PartialEq, Eq)]
pub struct Com<T: ComVtable> {
    ptr: NonNull<T>,
}

impl<T: ComVtable> Com<T> {
    pub unsafe fn from_owned(ptr: *mut T) -> Option<Self> {
        Some(Self {
            ptr: NonNull::new(ptr)?,
        })
    }

    pub unsafe fn from_shared(ptr: *mut T) -> Option<Self> {
        let comrc = Self {
            ptr: NonNull::new(ptr)?,
        };

        comrc.add_ref();
        Some(comrc)
    }

    pub unsafe fn add_ref(&self) {
        let funknown = self.ptr.as_ptr() as *mut v3_funknown;
        if let Some(add_ref) = *addr_of!((*funknown).add_ref) {
            add_ref(funknown as _);
        }
    }

    pub unsafe fn release(&self) {
        let funknown = self.ptr.as_ptr() as *mut v3_funknown;
        if let Some(release) = *addr_of!((*funknown).release) {
            release(funknown as _);
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
            let funknown = self.ptr.as_ptr() as *mut v3_funknown;
            if let Some(query_interface) = *addr_of!((*funknown).query_interface) {
                let mut obj = null_mut();
                if query_interface(funknown as _, &I::IID as *const u8, &mut obj) == V3_RESULT_OK {
                    Com::from_owned(obj as *mut _)
                } else {
                    None
                }
            } else {
                None
            }
        }
    }

    pub fn as_ptr(&self) -> *mut T {
        self.ptr.as_ptr()
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
