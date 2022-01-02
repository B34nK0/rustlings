// variables5.rs
// Make me compile! Execute the command `rustlings hint variables5` if you want a hint :)

fn main() {
    let number = "T-H-R-E-E"; // don't change this line
    println!("Spell a Number : {}", number);
    //rust支持shadowing，重影，所以可以重新加上let关键字 覆盖前一个number，重新声明的类型可从赋值值类型推导
    let number = 3;
    println!("Number plus two is : {}", number + 2);
}
