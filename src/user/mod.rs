pub mod graphics;
pub mod math;
pub mod time;
pub mod input;
// pub mod fs;
pub mod sys;
pub mod io;


use core::{{fmt::Pointer}, ops::{Index, IndexMut}};

#[cfg(feature = "liballoc")]
pub use alloc as data;

use data::{string::*, vec::{*}};


pub struct Arguments {
    parent : String,
    args   : Option<Vec<String>>,
}

impl Arguments {
    pub fn empty() -> Self {
        Self {
            parent : String::from("none"),
            args : None
        }
    }

    pub fn new(parent : String, args : Option<Vec<String>>) -> Self {
        Self {
            parent,
            args
        }
    }

    pub fn parent(&self) -> &String {
        &self.parent
    }

    pub fn args(&self) -> &Option<Vec<String>> {
        &self.args
    }

    pub fn arg_count(&self) -> usize {
        if self.args.is_some() {
            let v = self.args.as_ref().unwrap();
            return v.len();
        } else {
            0
        }
    }


}

pub struct ConstPointer<T> {
    size : usize,
    const_ptr : *const T,
}

pub struct MutPointer<T> {
    size : usize,
    mut_ptr : *mut T 
}

impl<T> ConstPointer<T> {
    pub fn from(item : &T, size : usize) -> Self {
        Self {
            size,
            const_ptr : item
        }
    }

    pub fn deref(&self) -> &T {
        unsafe {&*self.const_ptr}
    }

    pub fn cast<C>(&self) -> &C {
        unsafe {&*(self.const_ptr as *const C)}
    }

    pub fn copy_bytes(&self, buffer : &mut [u8]) {
        assert!(self.size >= buffer.len());
        for i in 0..buffer.len() {
            buffer[i] = self[i]
        }
    }

    pub fn size(&self) -> usize {
        self.size
    }
}

impl<T> MutPointer<T> {
    pub fn from(item : &mut T, size : usize) -> Self {
        Self {
            size,
            mut_ptr : item
        }
    }

    pub fn deref(&self) -> &mut T {
        unsafe {&mut *self.mut_ptr}
    }

    pub fn cast<C>(&self) -> &mut C {
        unsafe {&mut *(self.mut_ptr as *mut C)}
    }

    pub fn as_slice_ref(&self) -> &mut [u8] {
        self.cast::<&mut [u8]>()
    } 

    pub fn copy_bytes(&self, buffer : &mut [u8]) {
        assert!(self.size >= buffer.len());
        for i in 0..buffer.len() {
            buffer[i] = self[i]
        }
    }

    pub fn set_bytes(&mut self, buffer : &[u8]) {
        assert!(self.size >= buffer.len());
        for i in 0..buffer.len() {
            self[i] = buffer[i];
        }
    }

    pub fn size(&self) -> usize {
        self.size
    }
}

impl<T : Default> MutPointer<T> {
    pub fn new(size : usize) -> Self {
        Self {
            size,
            mut_ptr : &mut T::default()
        }
    }
}

impl<T> Index<usize> for ConstPointer<T> {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        assert!(index <= self.size);
        let raw : *const u8 = self.cast();
        unsafe {&*raw.add(index as usize)}
    }
} 

impl<T> Index<usize> for MutPointer<T> {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        assert!(index <= self.size);
        let raw : *const u8 = self.cast();
        unsafe {&*raw.add(index as usize)}
    }
}

impl<T> IndexMut<usize> for MutPointer<T> {

    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        assert!(index <= self.size);
        let raw : *mut u8 = self.cast();
        unsafe {&mut *raw.add(index as usize)}
    }
} 

impl<T> Pointer for ConstPointer<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "ConstPointer<{}> @ {:p}, ",core::any::type_name::<T>(), self.const_ptr)
    }
}
