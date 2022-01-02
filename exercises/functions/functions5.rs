// functions5.rs
// Make me compile! Execute `rustlings hint functions5` for hints :)

fn main() {
    let answer = square(3);
    println!("The answer is {}", answer);
}

//这里可以采用 return num*num; 返回一个表达式的值
//也可以去掉分号 return，隐式返回一个值
fn square(num: i32) -> i32 {
    num * num
}
