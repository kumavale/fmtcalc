# fmtcalc

[![Actions Status](https://github.com/kumavale/fmtcalc/workflows/CI/badge.svg)](https://github.com/kumavale/fmtcalc/actions)
[![Crate](https://img.shields.io/crates/v/fmtcalc.svg)](https://crates.io/crates/fmtcalc)
[![license](https://img.shields.io/badge/license-MIT-blue.svg?style=flat)](LICENSE)  

This is a simple calculator written in Rust.  
It is an alternative to printf for shell command.  

## Examples

```.sh
$ fmtcalc 4+2
6

$ fmtcalc "1+2*3 = {}" "1+2*3"
1+2*3 = 7

$ fmtcalc "0xFF - 654.321"
-399.321

$ fmtcalc "3²+4²={}, 5²={}" "3*3+4*4" "5*5"
3² +4² =25, 5² =25
```

For better ergonomics I often abbreviate `fmtcalc` as `println` in your shell startup files:

```.sh
alias println='fmtcalc'
```

## Installation

```.sh
$ cargo install fmtcalc
```

## License

MIT

