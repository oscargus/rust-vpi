/// Declares the `vlog_startup_routines` symbol required by VPI simulators.
///
/// Provide one or more `extern "C" fn()` routine names. The macro emits a
/// null-terminated function pointer table as expected by common simulators.
///
/// # Example
/// ```ignore
/// extern "C" fn register() {
///     // Register callbacks/system tasks.
/// }
///
/// vpi::startup_routines!(register);
/// ```
#[macro_export]
macro_rules! startup_routines {
    ($($func:expr),* $(,)?) => {
        #[unsafe(no_mangle)]
        pub static vlog_startup_routines: [Option<extern "C" fn()>; $crate::count_idents!($($func),*) + 1] = [
            $(Some($func),)*
            None,
        ];
    };
}

/// Counts macro expression arguments at compile time.
///
/// This is primarily used internally by [`startup_routines!`].
#[macro_export]
macro_rules! count_idents {
    () => {0};
    ($_head:expr $(, $tail:expr)*) => {1 + $crate::count_idents!($($tail),*)};
}
