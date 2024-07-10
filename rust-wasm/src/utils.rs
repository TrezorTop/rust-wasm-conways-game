pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

/// A timer struct that measures the duration of a block of code.
///
/// The `Timer` struct is used to measure the duration of a block of code by
/// calling `web_sys::console::time_with_label()` when the `Timer` is created,
/// and `web_sys::console::time_end_with_label()` when the `Timer` is dropped.
/// This allows you to easily measure the performance of your code in the
/// browser's developer console.
///
/// # Example
/// 
/// use utils::Timer;
///
/// fn do_something() {
///     let _timer = Timer::new("do_something");
///     // do something that you want to time
/// }
/// 
pub struct Timer<'a> {
    name: &'a str,
}

impl<'a> Timer<'a> {
    pub fn new(name: &'a str) -> Timer<'a> {
        web_sys::console::time_with_label(name);

        Timer { name }
    }
}

impl<'a> Drop for Timer<'a> {
    fn drop(&mut self) {
        web_sys::console::time_end_with_label(self.name);
    }
}
