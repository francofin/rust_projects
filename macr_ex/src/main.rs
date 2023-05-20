use proc_macro_item::debug_print;

macro_rules! avg {
    ($(,)*) => {
        {
            0.0
        }

    };

    ($($val: expr), +$(,)*) => {{
        let count = 0usize $(+ {let _ =stringify!($val); 1})*; //0usizeinit count to 0.
        let sum = 0.0 $(+ $val as f64)*;

        sum/count as f64
    }};
}

#[debug_print]
fn main() {
    println!("Hello, world!");
    let v = vec![1,5,45,32,23,54,7];
    // println!("{}", avg!(v));
    println!("{}", avg!(1,5,45,32,23,54,7));
}
