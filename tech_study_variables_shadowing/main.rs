fn main() {
    // by default variables are immutable
    // let mut allows variables to change

    //println! - the exclamation means to run the macro
    let mut x = 2000;
    println!("I was born in {}.", x);
    
    x = 1999;
    println!("I was born in {}.", x);

    // name shadowing
    {//different interior scope, grabs x from outside scope; leaves it unchanged
        let x = x + 1;
        println!("I was born in {}.", x)
    }

    let x = x - 1;
    println!("I was born in {}.", x);

    let x = "1998..";
    println!("I was born in {}.", x);

    const BIRTH_YEAR : &str = "1998";
    println!("I was born in {}.", BIRTH_YEAR);
}
