// variables2.rs
// Make me compile! Execute the command `rustlings hint variables2` if you want a hint :)

fn main() {
    //声明变量时需要确认变量类型，在1示例中，通过声明赋值方式确认了x的类型
    //在变量名后使用:type的方式显示声明类型为u32即无符号32位值
    let x:u32 = 10;
    if x == 10 {
        println!("Ten!");
    } else {
        println!("Not ten!");
    }
}
