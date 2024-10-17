fn main(){
    // integers
    let x: i8 = -23;
    let y: u32 = 499;
    let z: f32 = 299.829;

    print!("x: {}\ny: {}\nz: {}\n", x, y, z);

    let mut a: i32 = 9;

    for i in 0..100{
        a+=i;
    }

    print!("Value of 'a' after increment: {}\n", a);

    let is_male: bool = true;
    let is_above_18: bool = true;

    if is_male{
        print!("YOU ARE A MALE\n");
    }
    else{
        print!("YOU ARE A FEMALE");
    }
    if is_male && is_above_18{
        print!("YOU ARE A LEGAL MALE\n");
    }

    let greeting: String = String::from("Hello world!");
    print!("{}\n", greeting);

    let char1 = greeting.chars().nth(2);

    //Rust will not simply allow me to do print!("{}", greeting[0]); because it does not know that
    //if there is any character at the given index. So to prevent this we do pattern matching
    match char1{
        Some(c)=>println!("Char at index 0 is: {}", c),
        None => println!("NOTHING AT INDEX 0"),
    }

    //We can do this but not recomended because there can be an overflow and error will be shown
    println!("{}", char1.unwrap());

    let sentence: String = String::from("Hello this is a sentence");
    let first_word: String = first_word(sentence);
    println!("The first word of the given sentence is: {}", first_word);

    let n: i128 = 1;
    for i in 0..n{ // We are using this _ here is because we are not using i anywhere and to avoid
                    // getting any kind of warning we use _
        println!("Hello, World! {}", i);
    }
}

// Defining a function with a return type in rust, return type can be anything
fn first_word(sentence: String) -> String{
    let mut ans: String = String::new();
    for char in sentence.chars(){
        ans.push_str(char.to_string().as_str());
        if char == ' '{
            break;
        }
    }
    return ans;
}
