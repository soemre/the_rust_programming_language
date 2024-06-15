// fn main() {
//     let mut s = String::from("12345");

//     let r = &s;
//     let l = str::len(r);

//     let r = &*s;
//     let l2 = str::len(r);
//     println!("{l} {l2}");

//     let mut x = Box::new(2);
//     let r = &mut *x;
//     *r = 90;
//     println!("{r}");
// }

// fn main() {
//     let mut v: Vec<i32> = vec![1, 2, 3];
//     let num = &mut v[2];
//     // let num2: &&mut i32 = &num;
//     let num2 = &*num;
//     let _num3 = &*num;
//     // let num2 = &mut *num; // Inner type is being borrowed and since its being done trough the num variable it's being borrowed as well
//     println!("{} {}", *num, *num2);
// }

// fn main() {
//     let v: Vec<i32> = vec![1, 2, 3];
//     let num = &v[2];

//     println!("{} {}", *num, v[2]);
// }

// fn get_first(vr: &Vec<i32>) -> i32 {
//     // let vr = &&vec![]; // This one works as well
//     // vr[0]
//     let v = &**vr;
//     let _: &[i32] = &[5; 10];
//     v[0]
// }

// fn main() {
//     let mut x = 5;
//     let num = &mut x;

//     multiply((num, 2));
//     println!("{}", *num);
// }

// fn multiply(d: (&mut i32, i32)) {
//     *(d.0) *= d.1;
// }

// fn main() {
//     let mut v: Vec<i32> = vec![1, 2, 3];
//     let num: &mut i32 = &mut v[2];
//     let num2 = &num;
//     // *num = 4; // This is an invalid operation because the `*num` has been already borrowed as immutable reference.
//     let _ = *num; // This is the first proof, we are accessing the heap value from the mutable references owner, of that when we borrow a variable we are also borrowing the inner part of that values, which might be a reference.
//     let num3 = &num; // And this is the second proof, we are assigning a immutable reference.
//     println!("{} {} {}", *num, *num2, num3);
// }

// fn main() {
//     let mut v: Vec<i32> = vec![1, 2, 3];
//     let mut num: &mut i32 = &mut v[2];
//     let num2 = &mut num;
//     *num = 4;
//     println!("{} {} {}", *num, *num2, 3);
// }

// fn return_a_string() -> &'static str {
//     "Hello world"
// }

fn main() {
    let mut name = (String::from("Ferris"), String::from("Rustacean"));
    let first = &mut name.0;
    name.1.push_str(", Esq.");
    let rf = &name.1; // cannot borrow &name because it is already occupied by mut
    println!("{first} {} {}", name.1, rf);
}
