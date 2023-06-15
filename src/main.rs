//quize

pub mod foo {

    fn a() { println!("a"); }
    mod bar {
        pub fn b() {
            println!("b"); 
        }
    }
}

fn main() {
    foo::bar::b();
}