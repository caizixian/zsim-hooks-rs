// Ported from zsim/misc/hooks/test.c
use zsim_hooks::*;

fn main() {
    println!("Rust test");
    zsim_roi_begin();
    zsim_heartbeat();
    zsim_roi_end();
    println!("Rust test done");
}
