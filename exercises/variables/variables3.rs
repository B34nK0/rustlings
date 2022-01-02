// variables3.rs
// Make me compile! Execute the command `rustlings hint variables3` if you want a hint :)

fn main() {
    //x是不可变，需要声明为mut
    //不可变变量与常量有区别， 不可变变量可以被声明覆盖，而常量不能被覆盖
    let mut x = 3;
    println!("Number {}", x);
    x = 5; // don't change this line
    println!("Number {}", x);
}
