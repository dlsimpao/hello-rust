use std::io; //using std library


fn main() {
    let mut input = String::new();

    //standard input in
    io::stdin().read_line(&mut input).expect("failed to read line");


    println!("{}", input);

    //read_line
    //expect - checks to see if input is valid
}
