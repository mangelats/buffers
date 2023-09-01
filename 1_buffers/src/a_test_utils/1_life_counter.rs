use std::sync::atomic::{AtomicI64, Ordering};

/// Objects that counts how many instances of this type exists.
///
/// Useful to check that the containers properly drop all values.
pub struct LifeCounter<'a> {
    counter: &'a AtomicI64,
}
impl<'a> LifeCounter<'a> {
    pub fn new(counter: &'a AtomicI64) -> Self {
        counter.fetch_add(1, Ordering::SeqCst);
        Self { counter }
    }
}
impl Drop for LifeCounter<'_> {
    fn drop(&mut self) {
        self.counter.fetch_sub(1, Ordering::SeqCst);
    }
}

#[cfg(test)]
mod tests {
    use super::LifeCounter;
    use std::sync::atomic::{AtomicI64, Ordering};

    #[test]
    fn test_counter() {
        let counter = AtomicI64::new(0);
        {
            let _l = LifeCounter::new(&counter);
            assert_eq!(counter.load(Ordering::SeqCst), 1);
        }
        assert_eq!(counter.load(Ordering::SeqCst), 0);
    }
}
