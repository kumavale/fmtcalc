extern crate fmtcalc;
use fmtcalc::expr;

use std::env;
use dynfmt::{Format, SimpleCurlyFormat};

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.is_empty() {
        println!();
    } else if args.len() == 1 {
        println!("{}", expr(&args[0]));
    } else {
        let fmt = &args[0];
        let args: Vec<String> = env::args().skip(2).map(|argc| expr(&argc)).collect();
        let formatted = SimpleCurlyFormat.format(&fmt, &args);
        println!("{}", formatted.unwrap_or(std::borrow::Cow::Borrowed("")));
    }
}

