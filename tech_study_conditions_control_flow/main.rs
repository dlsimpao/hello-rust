fn main() {
    /* Six main operators
    
    <
    >
    <=
    >=
    !=
    ==

    */

    //let cond = 2 > 3.0;

    let cond = 2 > 3;

    /* Compound operators
    &&
    ||
    !
    */

    let cond2 = !false || !cond;

    println!("{}", cond2);

    // control flow
    let food = "cookie";

    if food == "cookie" {
        println!("I like {} too", food);
    }else{
        println!("I don't like {}", food);
    }
}
