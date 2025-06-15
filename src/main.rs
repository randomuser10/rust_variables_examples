fn main() {
    //Scalar types - integer, float, boolean, character
    let x = 5;
    println!("The value of x: {x}");
    let x = 8;
    println!("The value of x now is:{x}");

    // Adding the mut
    let mut y = 32;
    println!("The value of y is: {y}");
    y = 12;
    println!("The value of y now is using mut is:{y}");

    let divi: f32 = 45.2 / 3.0;
    let remaind: u8 = 50 % 3;
    println!("The result of the division is: {divi}");
    println!("The remainder is:{remaind}");

    let boolt: bool = true;
    let boolf: bool = false;
    println!("These are the boolean values:{boolt} & {boolf}");

    let char_new: char = 'S';
    println!("This is the character value: {char_new}");
}
