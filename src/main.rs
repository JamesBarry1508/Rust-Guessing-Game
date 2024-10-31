use std::{cmp::Ordering, io};
use rand::Rng;

fn main(){
    let correct = rand::thread_rng().gen_range(1..=10);
    //println!("The correct number is: {correct}");
    
    println!("Enter a number: ");
    loop{
        let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");


    let input: u32 = match input.trim().parse(){
        Ok(num) => num,
        Err(e) =>{
            println!("Error try again {e}");
            continue;
        }
    };
    //Method 1
    // let mut message = if correct <input{
    //     String:: from("You guessed too high")
    // }else if correct > input{
    //     String:: from("You guessed too low")
    // }else{
    //     String:: from("You guessed correctly")
    // };

    //Method 2
    let message = match input.cmp(&correct){
        Ordering::Less => "You guessed too low",
        Ordering::Greater => "You guessed too high",
        Ordering::Equal => {
            println!("You guessed correctly");
            break
        }
    };

        println!("{message}")
    }
}
