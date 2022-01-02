// move_semantics2.rs
// Make me compile without changing line 13!
// Execute `rustlings hint move_semantics2` for hints :)

fn main() {
    let mut vec0 = Vec::new();

    //vec0的所有权已转移，因为函数参数类型为Vec<i32> ,而Vec没有实现copy trait
    //需要变更函数参数接收一个引用，因为变量默认为不可变，所以引用参数也需要mut声明为可变的，函数体内才能对变量进行修改
    //传参时需要显示使用&mut
    fill_vec(&mut vec0);

    // Do not change the following line!
    println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);

    vec0.push(88);

    //println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}

fn fill_vec(vec: &mut Vec<i32>) {

    vec.push(22);
    vec.push(44);
    vec.push(66);

}
