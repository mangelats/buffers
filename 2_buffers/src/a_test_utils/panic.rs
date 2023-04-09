use std::panic;

/// Utility to catch panics and assert things about them.
pub fn catch_panic_unwind_silent<F, R>(f: F) -> std::thread::Result<R>
where
    F: FnOnce() -> R + panic::UnwindSafe,
{
    let prev_hook = panic::take_hook();
    panic::set_hook(Box::new(|_| {}));
    let result = panic::catch_unwind(f);
    panic::set_hook(prev_hook);
    result
}

/// Assert that a given callable panics on call.
pub fn assert_panic<F, R>(f: F)
where
    F: FnOnce() -> R + panic::UnwindSafe,
{
    let panicked = catch_panic_unwind_silent(f).is_err();
    assert!(
        panicked,
        "The function didn't panic even though it was expected"
    );
}
