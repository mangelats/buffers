use super::Buffer;

pub trait ContinuousMemoryBuffer: Buffer {
    fn ptr(&self, index: usize) -> *const Self::Element;
    fn mut_ptr(&mut self, index: usize) -> *mut Self::Element;
}
