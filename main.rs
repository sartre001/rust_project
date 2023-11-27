use std::{io, usize};



fn main() {
     println!("Hello, world!");
     println!(" Lets start with variables and constants.................................");
    let x = 4;
    println!("x is: {}", x);
    {
        let x = 14;
        println!("x is: {}", x);

    }
    let x = x + 1;

    println!("x is: {}", x);
    println!(" Now Lets do constants ...............................................");
    const SECONDS_IN_MINTURES: u32 = 60; // define a constant value and set it to an unlisted integer 32 and set a value of 60
   // SECONDS_IN_MINTURES: u32 = 60;
    println!("seconds in a minute are :{} " , SECONDS_IN_MINTURES); 



    println!(" Now lets Move to tuples.............................................. ");

    println!(" Type declaration .............................................. ");

    { 
        let x: u64 = 100000; //when the type is specified, then the program expects the runtime to have the same type unless otherwise. 
        // avoid type mismatch
        let y = x; 
        println!("{},{}", x, y);
    }


    {

         let mut tup: (i32, bool, char) = (1, true, 's');// defined a tuple as mutable with 3 elements as data types
         println!("{}", tup.0);
         println!("{}", tup.1);
         println!("{}", tup.2);

        tup.0 = 10; // reassigned the first element to contain the value 10 

        println!("Here we have declared a mutable tuple usign the (let mut tup:) and changed 1 to : {}, {}, {}", tup.0,tup.1,tup.2);



    }


    println!(" Now lets Move to arrays.............................................. ");

    {
        let  mut arr = [1, 2, 3, 4, 5]; // declared 4 elements to an array 
        arr[3] = 7; // reassigned the array 3rd value with 7
        // you can print multiple items by adding ("{}, {}", arr[1], arr[4])
        println!("The element (arr[3]) has been mutated to contain then value 7 hence it is called with: {}, {}", arr[3], arr[1]); 
    
    }
    println!(" Now lets Move to Console input and output.............................................. ");

    {
        println!("Type in your Name Please. :");
        let mut input = String::new();// defined a mutable input as String and a function new()
        
        io::stdin().read_line(&mut input).expect("Nothing to read in line");
        // the library or crate io::stdin() is used to read the input and create a copy of the actual line or string
        // the .expect is used to catch any errors that arise. 
        println!("Hello :{}", input);
    }
    println!(" Now lets Move to Type Conversion .............................................. ");

    {
        let x = 200.1f64;
        let y = 100.2f64; 

        let z = x + y; 

        println!("The Result for implicit declaration is  :{}", z )
    }
    println!(" Now lets Move to Type Conversion with implicit values  .............................................. ");

    {
        let x = 200.3_f64;
        let y = 100.2_f64;

        let z = x * y;
        println!("The Result for implicit declaration is :{}", z )

    }
    println!(" Now lets Move to Type Conversion with explicit values .............................................. ");

    {

        let x = 200.3 as f64;
        let y = 100.2_f32;

        let z = x * (y as f64);
        println!("The Result for explicit declaration is :{}", z )


    }
    println!(" Now lets Move to Type Conversion with an overflow .............................................. ");

    {
        let x =  (i128::MAX as f64) + 1.2;
        let y = 100.2_f32;

        let z = x * (y as f64);
        println!("The Result for explicit declaration is :{}", z )
    }

    println!(" Now lets Move to Type Conversion from String to integer .............................................. ");

    {
        let mut input = String::new();
    
        // Read a line of input from the user, and handle any potential errors
        io::stdin().read_line(&mut input).expect("Failed to read line");
    
        // Parse the input string into a 64-bit integer, and handle the result using a match statement
        match input.trim().parse::<i64>() {
            Ok(int_input) => {
                // Conversion was successful, proceed with the calculation
                let result = int_input + 12;
                println!("{}", result);
            }
            Err(err) => {
                // Conversion failed, handle the error (print it for now)
                println!("Wangu is nhamba kwete izwi: {}", err);
            }
        }
    }
    
    println!(" Now lets Move to conditional statements  .............................................. ");

    {
        let cond = 2 < 2; 
        println!("This condition is :{}", cond);
    }
    println!(" Now lets Move to conditional statements  .............................................. ");

    {
        let cond = (10 as f64) < 20.23; 
        println!("This Floating condition is :{}", cond);
    }
    println!(" Arithmetic AND :true or false .............................................. ");

    {
        let cond = (10 as i128) > 9;
        let _cond1 = true && cond; 

        println!("Checking if the first condition and the second condition are both true or false :{}", cond)
    }
    println!(" Arithmetic OR :true or false .............................................. ");

    {

        let cond = (10 as i128) > 9;
        let _cond1 = false || cond; 

        println!("Checking the first condition and the second condition :{}", cond)
    

    }
    println!(" Arithmetic NOT or Negation :true or false .............................................. ");

    {

        let cond = (101 as i128) > 129;
        let _cond1 = !(true || cond); 

        println!("Checking the first condition and the second condition :{}", cond)
    

    }
    println!(" Conditional if else statement  ........................ ");

    {
        let final_project = "in progress!" ;
        if final_project == "Break-Time! "
        {
            println!("Break is over. Start Working! ");
        }
        else if final_project == "Lunch-Time!"
        {
            println!("You are spending too much time..!");
        }
        else if final_project == "Refresh-Time!"
        {
            println!("Make you Proud! "); 

        }
         else 
         {
            println!("Make progress!!!!!"); 
         }
        

    }
    println!("The difference between Stack and Heap !!!!!!!!!!!!!!!!!"); 

    {
        // Stack: Variables with known size at compile time
    let stack_variable = 42; // This value is stored on the stack

    // Heap: Variables with dynamic size or unknown size at compile time
    let heap_variable = Box::new(42); // This value is stored on the heap

    println!("Stack Variable is: {}", stack_variable);

    // Accessing the heap variable requires dereferencing the Box
    println!("Heap Variable is: {}", *heap_variable);

    // When the Box goes out of scope, the memory it owns is freed
    // This is automatically handled by Rust, preventing memory leaks
    }
    println!("Ownership, stack and Heap  !!!!!!!!!!!!!!!!!"); 
    //this block contains the stack and heap and variable clone
    {
        let x: i64 = 10; 
        let _y: i64 = x; 

        let st1: String = String::from("This is a String using string type");
        let st2: String = st1.clone();

        println!("{}, This is a value of st1 ", st1);
    }
   


    println!(" Lets move to Functions   ........................ ");

        // Call the multiply_numbers function
        multiply_numbers(12, 100);
        let result = add_numbers(12, 24);
        println!("The sum of a and b - 19 if greater than 10 is: {}", result);




        println!("Ownership, stack and Heap  !!!!!!!!!!!!!!!!!"); 
        // this block contains the ownership function 
        { 
            let a: String = gives_ownership(); // `a` takes ownership of the value returned by gives_ownership
            let b: String = String::from("Shiengy");
            let c: String = takes_and_gives_back(b); // `c` takes ownership of the value passed to takes_and_gives_back
    
            println!("a = {}, c = {}", a, c);
        }
        println!("Reference in ownership   !!!!!!!!!!!!!!!!!"); 
        // this block contains the reference function 
        {
            let rf1: String = String::from("Shingie");
            let len: usize = calculate_length(&rf1); // store the length of the value in rf1 
            println!("The length of '{}' is {}. ", rf1, len); 

        }

        println!("mutable variable that change without taking ownership   !!!!!!!!!!!!!!!!!"); 



        {
            let mut vr1: String = String::from("MAZIOFA");
            change_var(&mut vr1);
            println!("The Mutated value is : {}", vr1);
        }
    


        

    }

// Function to multiply two numbers and print the result
fn multiply_numbers(x: i32, y: i32) {
    println!("The Answer after multiplication is : {}", x * y)
}

fn add_numbers(a: i32, b: i32) -> i32 {
    let result = a + b;
    if result > 10 {  
        return result - 19;
    }
    result
    
}
    // Once this block is over, `a` and `c` go out of scope, and the memory they owned is freed.

fn gives_ownership() -> String {
    let some_string: String = String::from("Shingy");
    some_string // The ownership of `some_string` is transferred to the calling function
}
fn takes_and_gives_back(a_string: String) -> String {
    a_string // The ownership of the passed `a_string` is transferred to the calling function
}

fn calculate_length(s: &String) -> usize {
    let length:usize = s.len(); // len() returns the length of a String
    length
}

fn change_var(some_string: &mut String) {
    some_string.push_str( ", Shingirayi"); 
 
}






    




