// move_semantics3.rs
// Make me compile without adding new lines-- just changing existing lines!
// (no lines with multiple semicolons necessary!)
// Execute `rustlings hint move_semantics3` for hints :)

fn main() {
    let mut vec0 = Vec::new();
    //参数类型为&mut 那么vec0在声明时也需要声明为mut
    let mut vec1 = fill_vec(&mut vec0);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);

    vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}
//函数对参数vec数据进行修改，需要声明为&mut 可变引用
fn fill_vec(vec: &mut Vec<i32>) -> Vec<i32> {
    vec.push(22);
    vec.push(44);
    vec.push(66);
//需要返回一个Vec
    vec.to_vec()
}
