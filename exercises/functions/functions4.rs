// functions4.rs
// Make me compile! Execute `rustlings hint functions4` for hints :)

// This store is having a sale where if the price is an even number, you get
// 10 Rustbucks off, but if it's an odd number, it's 3 Rustbucks off.


fn main() {
    let original_price = 51;
    //函数需要输出值（返回值）用于打印
    println!("Your sale price is {}", sale_price(original_price));
}

//该函数的逻辑是如果数为偶数即减去10，如果为奇数即减去3，减法会导致为负数，那么可返回i32
//采用-> i32表示返回值类型
//price-10、price-3 没有采用；号结束，表示为值，rust函数体 隐式返回最后的值(不需要使用return关键)
//也可以采用return显示地返回，那么price-10、price-3需要采用分号结束，表示为一个表达式
fn sale_price(price: i32) -> i32{
    if is_even(price) {
        price - 10
    } else {
        price - 3
    }
}

fn is_even(num: i32) -> bool {
    num % 2 == 0
}
