pub fn main() {
    print_examples();

    let heap_string = String::from("example of a string on heap");
    let stack_string = " Example of a string slice on stack";
    string_vs_str(heap_string.clone(), stack_string); // without clone(), the original String is moved and cannot be used again
    string_vs_str(heap_string, stack_string);
    // string_vs_str(heap_string, stack_string); // => not possible, as heap_string has been moved

    string_operations(stack_string);

    println!("[compare_to_ten] {} ten", compare_to_ten(50));
    println!("[compare_to_ten] {} ten", compare_to_ten(10));
    println!("[compare_to_ten] {} ten", compare_to_ten(-6666666));

    let to_be_incremented_1 = 41;
    println!("[pass_by_value] Before function: {}", to_be_incremented_1);
    let result1 = pass_by_value(to_be_incremented_1);
    println!("[pass_by_value] After function: {}, return value {}", to_be_incremented_1, result1);

    let mut to_be_incremented_2 = 41;
    println!("[pass_by_reference] Before function: {}", to_be_incremented_2);
    let result2 = pass_by_reference(&mut to_be_incremented_2);
    println!("[pass_by_reference] After function: {}, return value {}", to_be_incremented_2, result2);

    let to_be_incremented_3 = 41;
    println!("[pass_by_immutable_value] Before function: {}", to_be_incremented_3);
    let result3 = pass_by_immutable_value(to_be_incremented_3);
    println!("[pass_by_immutable_value] After function: {}, return value {}", to_be_incremented_3, result3);

    let to_be_incremented_4 = 41;
    println!("[pass_by_immutable_reference] Before function: {}", to_be_incremented_4);
    let result4 = pass_by_immutable_reference(&to_be_incremented_4);
    println!("[pass_by_immutable_reference] After function: {}, return value {}", to_be_incremented_4, result4);

    while_example();
    loop_example();
    for_example();
    match_example(-55);
    match_example(100);
    
    datatypes();
    cast_example();
    println!("[uniqueptr_example] Value of box after function: {}", uniqueptr_example());

    reference_scope();

    read_from_console();
}

fn print_examples() {
    println!("[print_examples] Hi mum!");
    let gibberish = "some gibberish";
    let some_number = -554.4;
    println!("[print_examples] I'm printing {} and {} from rust", gibberish, some_number);
    println!("[print_examples] Bye!");
}

fn string_vs_str(heap_string: String, stack_string: &str) {
    println!("[string_vs_str] {},{}", heap_string, stack_string);
}

fn string_operations(stack_string: &str) {
    let reversed: String = stack_string.chars().rev().collect();
    println!("[string_operations] reversed string: {}", reversed);
    let substring: &str = &stack_string[0..6]; // take first 6 characters
    println!("[string_operations] substring: {}", substring);
    let delimited: Vec<&str> = stack_string.split(" ").collect(); // split returns an iterator, collect creates a vector out of the iterator
    println!("[string_operations] delimited: {:?}", delimited);
    let trimmed: &str = stack_string.trim(); // trims leading and trailing whitespaces
    println!("[string_operations] trimmed: {:?}", trimmed);
    let upper: String = stack_string.to_uppercase(); // create a new heap string
    println!("[string_operations] upper: {:?}", upper);
    let lower: String = stack_string.to_lowercase(); // create a new heap string
    println!("[string_operations] lower: {:?}", lower);
    let noWhitespaces: String = stack_string.chars().filter(|c| !c.is_whitespace()).collect(); // create a new heap string
    println!("[string_operations] noWhitespaces: {:?}", noWhitespaces);
    let removedLetterS: String = stack_string.chars().filter(|&c| c != 's').collect();
    println!("[string_operations] removedLetterS: {:?}", removedLetterS);
    let replacedLetterAwithE: String = stack_string.replace("a", "e");
    println!("[string_operations] replacedLetterAwithE: {:?}", replacedLetterAwithE);
    let occurencesOfLetterR = stack_string.chars().filter(|&c| c == 'r').count();
    println!("[string_operations] occurencesOfLetterR: {:?}", occurencesOfLetterR);
}

fn compare_to_ten(number: i32) -> &'static str {
    if number > 10 {
        "more than"
    } else if number < 10 {
        "less than"
    } else {
        "equal to"
    }
}

fn pass_by_value(mut value: i32) -> i32 {
    value += 1;
    println!("[pass_by_value] Inside function: {}", value);
    value
}

fn pass_by_reference(value: &mut i32) -> i32 {
    *value += 1;
    println!("[pass_by_reference] Inside function: {}", value);
    *value
}

fn pass_by_immutable_value(value: i32) -> i32 {
    let result = value + 1;
    println!("[pass_by_immutable_value] Inside function: {}", result);
    result
}

fn pass_by_immutable_reference(value: &i32) -> i32 {
    println!("[pass_by_immutable_reference] Inside function: {}", value);
    let result = *value + 1;
    result
}

fn while_example() {
    let mut counter = 5;
    while counter > 0 {
        println!("[while_example] counter: {}", counter);
        counter -= 1
    }
}

fn loop_example() {
    let mut counter = 5;
    loop {
        println!("[loop_example] counter: {}", counter);
        counter -= 1;
        if counter == 0 {
            break
        }
    }
}

fn for_example() {
    let arr = [10, 20, 30, 40, 50];

    for element in arr.iter() {
        println!("[for_example] Foreach: {}", element);
    }

    for i in 0..arr.len() {
        println!("[for_example] For index {}: {}", i, arr[i]);
    }

    for (i, a) in arr.iter().enumerate() {
        println!("[for_example] Enumerate {}: {}", i, a);
    }
}

fn match_example(number: i32) {
    match number {
        0 => println!("[match_example] zero"),
        1 | 3 | 5 => println!("[match_example] small odd number"),
        y if y < 0 => println!("[match_example] negative"),
        y => println!("[match_example] something else {}", y)
        //_ => "[match_example] wildcard"  // unreachable, as the previous line covers all the rest
    };

    // variant with variable; note: if I want to format!, message has to be a string
    let message = match number {
        0 => "zero".to_string(),
        1 | 3 | 5 => "small odd number".to_string(),
        y if y < 0 => "negative".to_string(),
        y => format!("something else {}", y),
        //_ => "[match_example] wildcard"  // unreachable, as the previous line covers all the rest
    };
    println!("[match_example] {}", message);
}

fn datatypes() {
    // boolean
    let _x = true;
    let _x = false;

    // signed
    let _x = -154555;
    let _x = 34i8;
    let _x = 34i16;
    let _x = 34i32;
    let _x = 34isize;
    let _x = 34i64;
    let _x = -1115553125464645i128;

    // unsigned
    let _x = 34u8;
    let _x = 34u16;
    let _x = 34u32;
    let _x = 34usize;
    let _x = 34u64;
    let _x = 34u128;

    // floats
    let _x = 34f32;
    let _x = 34f64;

    // binary, octal, hex
    let _x = 12;
    let _x = 0b1100;
    let _x = 0o14;
    let _x = 0xe;
    let _x = 0b_1100_0011_1011_0001;  // underscores are ignored
}

fn cast_example() {
    let _x = 34usize as isize;   // cast usize to isize
    let _x = 10 as f32;      // isize to float
    let _x = 10.45f64 as i8; // float to i8 (loses precision)
    let _x = 4u8 as u64;     // gains precision
    let _x = 400u16 as u8;   // 144, loses precision (and thus changes the value)
    println!("[cast_example]`400u16 as u8` gives {}", _x);
    let _x = -3i8 as u8;     // 253, signed to unsigned (changes sign)
    println!("[cast_example]`-3i8 as u8` gives {}", _x);
    //let x = 45 as bool;  // FAILS! (use 45 != 0 instead)
    let _x = true as usize;  // cast bool to usize (gives a 1)
}

fn uniqueptr_example() -> Box<i32> {
    let mut mybox = Box::new(56465);  // Box is an object wrapper
    println!("[uniqueptr_example] value of box: {}", mybox);
    let mybox2 = Box::new(-56465);
    *mybox += 1;  // we can increment only if the variable is mutable
    println!("[uniqueptr_example] value of box after increment: {}", mybox);
    mybox = mybox2;
    println!("[uniqueptr_example] value of box after reassignment: {}", mybox);
    mybox  // last statement returns
}

fn reference_scope() {
    let mut x = 5;
    let mut xr = &mut x;        // Ok - x and xr have the same lifetime
    *xr = 6;
    {
        let mut y = 6;
        //xr = &mut y;             // Error - xr will outlive y
    }
    println!("[reference_scope] {:?}", xr);   // xr is used here so it outlives y. Try to comment out this line.
}                           // x and xr are released here

use std::io::{self, BufRead, Write};
fn read_from_console() {
    let stdin = io::stdin();
    print!("[read_from_console] Enter your name : "); 
    io::stdout().flush();
    let mut name = String::new();
    stdin.lock().read_line(&mut name);
    println!("[read_from_console] Hello {} !", name.trim());
}
