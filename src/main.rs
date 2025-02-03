fn main() {
    //    let x;
    //    foobar(x);
    //    x = 42;

    let x;
    x = 42;
    foobar(x);
}

fn foobar(x: i32) {
    println!("x = {}", x);
}
