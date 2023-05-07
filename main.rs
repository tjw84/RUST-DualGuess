use rand::prelude::*;
use std::{io};

fn main() {
    // set up two random numbers
    let mut replay = 1;
    let mut rng = thread_rng();
    let mut num1 = rng.gen_range(1..101);
    // println!("{}", num1);
    let mut num2 = rng.gen_range(1..101);
    // println!("{}", num2);
    println!("Enter 2 numbers between 1 and 100.");
    while replay != 0 {
        // loop to have it keep asking for their numbers until they get it right
        let game_result = get_numbers(num1, num2);
        println!("{}", game_result);
    
        // If they wish to paly again then regenerate the random numbers
        if game_result == 1 {
            replay = 1;
            num1 = rng.gen_range(1..101);
        //  println!("{}", num1);
            num2 = rng.gen_range(1..101);
        //  println!("{}", num2);
            
        }

        // create end condition if they say they do not want to play again
        if game_result == 0 {
            replay = 0;
        }
        
   
    }
    
}

fn get_numbers(num1: i32, num2: i32) -> i32 {
    // get two numbers from the user to compare to the previously made random numbers
    println!("Enter your first number: ");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Failed to read line");
    let number1: i32 = input1.trim().parse().unwrap();
    // debugging check for initial number1 value
    // println!("{}", number1);
    
    println!("Enter your second number: ");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Failed to read line");
    let number2: i32 = input2.trim().parse().unwrap();
    // debugging check for initial number2 value 
    // println!("{}", number2);
    
    // call the check function to see if any of the numbers from the user match the RNG numbers
    let result = check(number1, number2, num1, num2); 
    // println!("{}", result);
    return result;
}

fn check(number1: i32, number2: i32, num1: i32, num2: i32) -> i32 {
    
    
    // check if they made their numbers out of bounds 
    if number1 < 1 || number1 > 100 || number2 < 1 || number2 > 100 {
        println!("please enter a valid whole number between 1 and 100");
        return 2; 
    }
    
    // if both numbers are wrong, check if they are high or low
    if number1 != num1 || number2 != num2 {
        // if a number is high check if just one is or if both are
        if number1 > num1 || number2 > num2 {
            if number1 <= num1 || number2 <= num2 {
                println!(" one is more");
            } else {
                println!(" both are high");
            }
        }
        // if a number is low check if just one is or if both are (thus recieving only one line of "one is more" or "one is less" informs them that one is correct) 
        if number1 < num1 || number2 < num2 {
            if number1 >= num1 || number2 >= num2 {
                println!( " one is less");
                
            } else 
            {
                println!( " both are low");
            
            }
        }
        // if both are correct inform the user and ask if they would like to play again with the answers set to numeric values
    } else {
        println!("You are correct!");
        println!("Would you like to play again? (enter 1 for yes or 0 for no)");
        // take in user input as a integer to return to previous function
        let mut play = String::new();
        io::stdin().read_line(&mut play).expect("Failed to read line");
        let replay: i32 = play.trim().parse().unwrap();   

        // exit program if they do not wish to play again by returning 0 back to main through get_numbers 
        // println!("{}", replay);
        if replay == 0{
            
            return replay;
        }
        
        // if they wish to play again it passes 1 up to main through get numbers which activates the conditional to reset the two RNG numbers and continues looping
        if replay == 1 {  
        return replay; 
        } else {
            // if neither correct input is given loop through again with the same two numbers
            let result = 2;
            return result;
             }
    }
// if all else fails, loop through again with the same two numbers    
return 2;   
}

