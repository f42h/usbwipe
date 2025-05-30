#[macro_export]
macro_rules! tern {
    ($con:expr, $t:expr, $f:expr) => {
        if $con { $t } else { $f }
    };
}

#[macro_export]
macro_rules! printdec {
    ($c:expr, $len:expr) => {
        for _ in 0..=$len {
            print!("{}", $c);
        }
    
        println!();
    };
}

#[macro_export]
macro_rules! gb {
    ($size:expr) => {
        ($size * 512) as f64 / (1024.0 * 1024.0 * 1024.0)
    };
}