fn main() {
    //primitive data types
    let _x: i32 = -2;// default
    /*
    i8
    i16
    i32
    i64
    i128
    */

    let _y: u32 = 2;// unsigned integer, no negative values

    /* floating point values
    f32
    f64
    */

    let _z: f64 = 2.0;

    /* boolean 
    bool
    */
    let _true_or_false = 0;

    /* char
    */
    let _letter: char = ';';

    /* tuple
    */

    /*
    let tup: (i32, bool, char) = (1, true, 's');
    let mut tup2: (i8, bool, char) = (1, true, 's');
    println!("{:?}", tup);

    println!("{:#?}", tup);

    tup2.0 = 2;

    println!("{}", tup2.0);
    */

    let mut arr: [i32; 5] = [1,2,3,4,5];
    arr[4] = 100;
    println!("{}", arr[4]);
}
