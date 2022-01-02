// functions3.rs
// Make me compile! Execute `rustlings hint functions3` for hints :)


fn main() {
    //函数需要传递一个u32的参数
    call_me(20);
}

fn call_me(num: u32) {
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}
