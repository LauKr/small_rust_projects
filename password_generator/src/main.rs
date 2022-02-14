use rand::Rng;
use std::io;


fn main() {
    let password_length: u64 = initialize();
    let password: String = generate_password(password_length);
    println!("Your generated password is:\n{}", password);
}


fn generate_char_set() -> Vec<&'static str>{
    let chars: Vec<&str> = vec!["1", "2", "3", "4", "5", "6", "7", "8", "9", "0"];
    return chars
}


fn generate_char<'a>(char_set: &std::vec::Vec<&'a str>) -> &'a str{
    let mut rng = rand::thread_rng();
    let random_int: usize = rng.gen_range(0..10);
    let rnd_char = char_set[random_int];
    //println!("Random number is {}", random_int);
    //println!("Random character from char_set is {}", rnd_char);
    return rnd_char
}


fn generate_password(passwd_length: u64) -> String {
    let mut password: String = "".to_owned();
    let char_set: Vec<&str> = generate_char_set();
    for _i in 0..passwd_length {
        password.push_str(generate_char(&char_set));
    }
    return password
}


fn initialize() -> u64{
    println!("Please insert the number of characters your password should have:");
    let mut passwd_length_string = String::new();
    io::stdin()
        .read_line(&mut passwd_length_string)
        .expect("Failed to read line");
    let passwd_length = passwd_length_string.trim().parse::<i32>().expect("invalid input");
    /*let passwd_length = passwd_length_string.trim();
    match passwd_length.parse::<u64>() {
        Ok(i) => println!("your integer input: {}", i),
        Err(..) => println!("this was not an integer: {}", passwd_length),
    };*/
    
    println!("Selected length is {}", passwd_length);
    return passwd_length.try_into().unwrap()
}