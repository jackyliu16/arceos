//! A useless macro files
//! Made BC the fraud by ChatGPT
//! Save BC reduce some kinds of workload (doesn't make much sense)

macro_rules! not_implemented {
    // () => {
    //     axlog::debug!("Warn: unimplement function: {}", stringify!(callsite));
    // };
    ($message:expr) => {
       axlog::debug!("Warn: unimplement function: {}", $message);
    };
}
// 想整个能显示函数名称的宏，但是给GPT骗了
macro_rules! dbg {
    () => {
        // axlog::debug!("Call {}", stringify!(callsite));
        axlog::debug!("");
    };
    ($message:ident) => {
        // let callsite = stringify!(callsite);
        // axlog::debug!("{}: {}", stringify!(callsite), $message);
        axlog::debug!("{}", $message);
    };
    ($($arg:tt)*) => {
        axlog::debug!("{}", format_args!("{}", format_args!($($arg)*)));
    }
}