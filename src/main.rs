//quize

// pub mod foo {

//     fn a() { println!("a"); }
//     mod bar {
//         pub fn b() {
//             println!("b"); 
//         }
//     }
// }

// fn main() {
//     foo::bar::b();
// }

// pub mod a {

//     pub mod b {
//         pub fn f() { println!("b1"); }

//         pub mod c {
//             pub fn f() { println!("c1"); }
//         }
//     }
//     pub fn entry() { super::b::c::f(); }
// }

// pub mod b {
//     pub fn f() { println!("b2"); }
//     pub mod c {
//         pub fn f() { println!("c2"); }
//     }
// }

// fn main() {
//     crate::a::entry();
// }


pub mod point {

    #[derive(Debug)]
    pub struct Point(i32, i32);

    impl Point {
        pub fn origin() -> Self { Point(0, 0) }
    }
}


fn main() {

    let mut p = point::Point::origin();

    p.0 += 1;

    println!("{p:?}");

}

// pub mod parent {

//     pub fn a() {}
  
//     fn b() {}
  
//     pub mod child {
  
//       pub fn c() {}
  
//     }
  
//   }
  
  
//   fn main() {
  
//     use parent::{*, child as alias};
  
//     // ...
  
//   }
  
//   Inside main, what is the total number of paths that can refer to a, b, or c (not including those that use self, super, or crate)? Write your answer as a digit such as 0 or 1. For example, if the only two valid paths were a and parent::b, then the answer would be 2.
//   You answered:
//   5
  
//   Context: There are two paths to a: parent::a and a. There are no paths to b, because it is private. There are three paths to c: parent::child::c, child::c, alias::c.