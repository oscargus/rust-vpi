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

#[macro_export]
macro_rules! count_idents {
    () => {0};
    ($_head:expr $(, $tail:expr)*) => {1 + $crate::count_idents!($($tail),*)};
}
