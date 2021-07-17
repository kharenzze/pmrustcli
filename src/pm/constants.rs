#[macro_export]
macro_rules! PM_BASE {
    ($path:expr) => {
        concat!("https://sync.appfluence.com", $path);
    };
}
