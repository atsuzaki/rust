trait T {
    fn foo() {}
    fn bar() {}
}

struct S;
impl T for S {}

fn main() {
<<<<<<< HEAD //~ ERROR encountered git conflict marker
    S::foo();
=======
    S::bar();
>>>>>>> branch
}
