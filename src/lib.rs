#![allow(unused_macros)]
macro_rules! bf_op {
    ($cells:ident, $ptr:ident, $sout:ident, $sin:ident, >) => {
        $ptr = $ptr.wrapping_add(1);
    };
    ($cells:ident, $ptr:ident, $sout:ident, $sin:ident, <) => {
        $ptr = $ptr.wrapping_sub(1);
    };
    ($cells:ident, $ptr:ident, $sout:ident, $sin:ident, +) => {
        $cells[$ptr] = $cells[$ptr].wrapping_add(1);
    };
    ($cells:ident, $ptr:ident, $sout:ident, $sin:ident, -) => {
        $cells[$ptr] = $cells[$ptr].wrapping_sub(1);
    };
    ($cells:ident, $ptr:ident, $sout:ident, $sin:ident, .) => {
        $sout.write(&[$cells[$ptr]]).unwrap();
    };
    ($cells:ident, $ptr:ident, $sout:ident, $sin:ident, ,) => {
        $sin.read(&mut $cells[$ptr..$ptr+1]).unwrap();
    };
    ($cells:ident, $ptr:ident, $sout:ident, $sin:ident, $else: tt) => {
        // Do nothing.
    };
}
macro_rules! get_bf {
    {$cells:ident, $ptr:ident, $sout:ident, $sin:ident, ->} => {
        bf_op! { $cells, $ptr, $sout, $sin, - }
        bf_op! { $cells, $ptr, $sout, $sin, > }
    };
    {$cells:ident, $ptr:ident, $sout:ident, $sin:ident, <-} => {
        bf_op! { $cells, $ptr, $sout, $sin, < }
        bf_op! { $cells, $ptr, $sout, $sin, - }
    };
    {$cells:ident, $ptr:ident, $sout:ident, $sin:ident, <<} => {
        bf_op! { $cells, $ptr, $sout, $sin, < }
        bf_op! { $cells, $ptr, $sout, $sin, < }
    };
    {$cells:ident, $ptr:ident, $sout:ident, $sin:ident, >>} => {
        bf_op! { $cells, $ptr, $sout, $sin, > }
        bf_op! { $cells, $ptr, $sout, $sin, > }
    };
    {$cells:ident, $ptr:ident, $sout:ident, $sin:ident, ..} => {
        bf_op! { $cells, $ptr, $sout, $sin, . }
        bf_op! { $cells, $ptr, $sout, $sin, . }
    };
    {$cells:ident, $ptr:ident, $sout:ident, $sin:ident, ...} => {
        bf_op! { $cells, $ptr, $sout, $sin, . }
        bf_op! { $cells, $ptr, $sout, $sin, . }
        bf_op! { $cells, $ptr, $sout, $sin, . }
    };
    {$cells:ident, $ptr:ident, $sout:ident, $sin:ident, [ $($a:tt)* ]} => {
        while $cells[$ptr] != 0 {
            $( get_bf! { $cells, $ptr, $sout, $sin, $a } )?
        }
    };
    {$cells:ident, $ptr:ident, $sout:ident, $sin:ident, $a:tt} => { bf_op! { $cells, $ptr, $sout, $sin, $a }};
}
#[macro_export]
macro_rules! bf {
    {$($a:tt)+} => {
        #[allow(unused)]
        {
            use std::io::{stdin, stdout};
            $crate::customized_bf! { 20000, 0, stdout(), stdin(), $($a)+ }
        }
    };
}
#[macro_export]
macro_rules! customized_bf {
    { $len_cells: literal, $ptr_start: literal, $stdout: expr, $stdin:expr, $($a:tt)+ } => {
        #[allow(unused)]
        {
            use std::io::{Write, Read};

            let mut cells = [0u8; $len_cells];
            let mut ptr = $ptr_start;
            let mut sout = $stdout;
            let mut sin = $stdin;
            $(get_bf! { cells, ptr, sout, sin, $a })+
        }
    };
}

