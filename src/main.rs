use std::fs;

struct User{
    name: String,
    age: i32,
    active: bool
}

// Implementing Structs

struct Rect{
    height: u32, 
    width: u32
}

// Implementing Enums

enum Direction{
    North,
    East,
    West,
    South
}

// Enums with values
enum Shape{
    Circle(f64),
    Square(f64),
    Rectangle(f64, f64)
}

impl Rect{
    fn area(&self) -> u32{
        self.height*self.width // If there is no ; in the last then it automatically returns the
                               // statement no need to write return keyword.
    }

    fn peri(&self) -> u32{
        2*(self.width+self.height)
    }
}

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

    //OWNERSHIP
    
    let s1: String = String::from("hello");
    let s2: String = s1;

    // print!("{}", s1); // This will generate error because the ownership has been moved
    
    println!("{}", s2);

    let my_string: String = String::from("Hi there");
    take_ownership(my_string.clone());
    println!("My String: {}", my_string);


    // BORROWING AND REFERENCES
    
    let s1: String = String::from("hello this is sameer");
    let s2: &String = &s1;
    println!("s2: {}", s2);
    println!("s1: {}", s1);

    let new_str: String = String::from("Hello this is my new string");
    borrowing(&new_str);
    println!("After borrowing completed: {}", new_str);


    // Mutable References
    
    let mut mut_s1: String = String::from("Hello");
    update_word(&mut mut_s1);
    println!("{}", mut_s1);


    // Structs
    
    println!("\nUsing Structs");

    let user = User{
        name: String::from("Sameer"),
        age: 20,
        active: false
    };

    println!("User name: {}\nUser age: {}\nIs active: {}", user.name, user.age, user.active);

    let rect = Rect{
        height: 12,
        width: 10
    };

    println!("\nArea of the rectangle with height: {} and width: {} is {}", rect.height, rect.width, rect.area());
    println!("Perimeter of the rectangle with height: {} and width: {} is {}", rect.height, rect.width, rect.peri());


    // Enums

    let my_direction: Direction = Direction::North;
    move_around(my_direction);

    // Enums with values

    let circle: Shape = Shape::Circle(6.0);
    let square: Shape = Shape::Square(13.9);
    let rectangle: Shape = Shape::Rectangle(29.0, 12.9);

    let area: f64 = calculate_area(circle);
    println!("\nArea {}", area);

    let res = fs::read_to_string("example.txt");
    match res{
        Ok(content) => println!("Content of the file are: {}", content),
        Err(err) => println!("Error reading file: {}", err)
    }

    let str_option: String = String::from("Smeer");
    let res = find_first_a(str_option.clone());
    let res2 = find_first_a_result(str_option);

    match res{
        Some(index) => println!("'a' is present at index: {}", index),
        None => println!("'a' is not present in the give string"),
    }

    match res2 {
        Ok(index) => println!("'a' is present at index: {}", index),
        Err(error) => println!("{}", error)
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

fn take_ownership(new_str: String){
    println!("New String: {}", new_str)
}

fn borrowing(my_string: &String){
    println!("Borrowed String: {}", my_string);
}

fn update_word(s: &mut String){
    s.push_str(" World!");
}

fn move_around(direction: Direction){
    println!("\n");
    match direction {
        Direction::North=>println!("Moving North"),
        Direction::East=>println!("Moving East"),
        Direction::West=>println!("Moving West"),
        Direction::South=>println!("Moving South")
    }
}

fn calculate_area(shape:Shape) -> f64{
    match shape{
        Shape::Circle(radius) => std::f64::consts::PI*radius*radius,
        Shape::Square(side_len) => side_len*side_len,
        Shape::Rectangle(height, width) => height*width
    }
}

fn find_first_a(str: String) -> Option<i32>{
    for(index, chars) in str.chars().enumerate(){
        if chars=='a'{
            return Some(index as i32);
        }
    }
    return None;
}

fn find_first_a_result(str: String) -> Result<i32, String>{
    for(index, chars) in str.chars().enumerate(){
        if chars=='a'{
            return Ok(index as i32);
        }
    }
    return Err("'a' is not present in the given string".to_string());
}
