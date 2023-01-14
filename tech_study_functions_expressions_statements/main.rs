fn test_one() {
    println!("Test has been called...");
}

fn main() {
    println!("Hello, world!");
    test_one();
    add_numbers(54, 45); // <- this is an expression; returns something

    let _x = add_numbers(54,45); // <- this is a statement; does not return anything

    let number = {// <- this is an expression; returns something
        _x + 1
    };

    println!("The number is {}", number);

    println!("Difference is {}", subtract_numbers(100,1));
}

fn add_numbers(x: i32, y: i32) -> i32 { //returns an i32
    println!("The sum is {}", x + y);
    x + y //returns x + y
}

fn subtract_numbers(x: i32, y: i32) -> i32 { //returns an i32
    return x - y;
}
