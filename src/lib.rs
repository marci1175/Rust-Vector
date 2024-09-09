use std::{alloc::{self, Layout}, marker::PhantomData, mem, ops::Index};

struct MVector<T> {
    /// Start addr of the addresses we are stroing data under
    /// An item will be placed when creating a new MVector instance
    start_addr: usize,
    
    /// End addr of the addresses we are stroing data under
    end_addr: usize,

    /// Size of one of the instances we are storing
    sizeof: usize,

    __phantom_data: PhantomData<T>,
}

impl<T> MVector<T> {
    pub fn new() -> Self {
        unsafe {
            let allocated_addr = alloc::alloc(Layout::from_size_align(size_of::<T>() * 4, size_of::<T>() * 4).unwrap());

            Self { start_addr: allocated_addr as usize, end_addr: allocated_addr as usize, sizeof: size_of::<T>(), __phantom_data: PhantomData::default() }
        }
    }

    pub fn len(&self) -> usize {
        (self.end_addr - self.start_addr) / self.sizeof
    }
}

impl<T> Index<usize> for MVector<T> {
    fn index(&self, index: usize) -> &Self::Output {
        unsafe {
            let val_start_addr = (self.start_addr as *mut u8).add(index * self.sizeof);

            let output = std::mem::transmute(val_start_addr.read());

            output
        }
    }
    
    type Output = T;
}