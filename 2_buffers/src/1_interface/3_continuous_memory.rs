use super::Buffer;

pub trait ContinuousMemoryBuffer: Buffer {
    unsafe fn ptr(&self, index: usize) -> *const Self::Element;
    unsafe fn mut_ptr(&mut self, index: usize) -> *mut Self::Element;
}
