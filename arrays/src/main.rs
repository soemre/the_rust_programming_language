fn main() {
    let arr = [1, 5, 3, 54];
    let arr_copy = arr;

    // let func_arr = array_tst();

    println!("{:?}", arr);
    println!("{:?}", arr_copy);
    // println!("{:?}", func_arr);

    let idx_pre: usize = 10;
    let idx: usize = idx_pre;
    println!("{:?}", arr[idx]);
}
// I guess rust destroys everything in the scope when it ends
// Ofcourse if its owner is here
// fn array_tst() -> &'static [i32; 5] {
//     let arr = [3; 5];

//     return &arr;
// }
