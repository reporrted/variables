fn main() {
    let x: i32 = 5;

    let x: i32 = x + 1;

    {
        let x: i32 = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    another_function(x)
}

fn another_function(x: i32) {
    println!("Secret number: {}", x);
}