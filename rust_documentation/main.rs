use rand::Rng;
use std::io; //rand = 0.8.5

fn main() {
    // let letters = vec!['a','b','c','d','e','f','g','h','i','j','k','l','m','n','o','p','q','r','s','t','u','v','w','x','y','z'];
    // let numbers = vec!['1','2','3','4','5','6','7','8','9','0'];
    // let symbols =vec!['!','@','#','$'];

    let tokens = vec![
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
        's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0',
        '!', '@', '#', '$',
    ];

    let tokens_no_symb = vec![
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
        's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0',
    ];

    println!("Hi, I generate passwords!");

    loop {
        let mut choice = String::new();
        println!("Do you want symbols? Type in Y or N.");
        
        io::stdin()
            .read_line(&mut choice)
            .expect("Not valid");

        choice = choice.trim().to_uppercase();

        let token_pool = if choice == "Y" {
            &tokens
        } else if choice == "N" {
            &tokens_no_symb
        }else{
            continue;
        };

        gen_password(7, token_pool);
        break;
    }

}

fn gen_password(pass_length: i32, token_pool: &Vec<char>){
    let mut password = String::from("");
    let pass_length = pass_length;

    for _n in 1..pass_length {
        let value = rand::thread_rng().gen_range(0..token_pool.len());
        password.push(token_pool[value]);
    }

    println!("{}", password.to_uppercase())
}
