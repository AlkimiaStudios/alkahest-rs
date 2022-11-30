/// Appends a null terminator to &str
#[macro_export]
macro_rules! c_str {
    ($str:ident) => {(
        [$str, "\0"].concat()
    )}
}
