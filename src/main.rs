extern crate rand;

use std::io;
use std::thread::sleep;
use std::time::Duration;
use rand::Rng;

fn main() {
    let strings: Vec<&str> = vec![
        "On the first day of Christmas, my true love sent to me
        A partridge in a pear tree",
        "On the second day of Christmas, my true love sent to me
        Two turtle doves, and
        A partridge in a pear tree",
        "On the third day of Christmas, my true love sent to me
        Three french hens
        Two turtle doves, and
        A partridge in a pear tree",
        "On the fourth day of Christmas, my true love sent to me
        Four calling birds
        Three french hens
        Two turtle doves, and
        A partridge in a pear tree",
        "On the fifth day of Christmas, my true love sent to me
        Five golden rings
        Four calling birds
        Three french hens
        Two turtle doves, and
        A partridge in a pear tree",
        "On the sixth day of Christmas, my true love sent to me
        Six geese a-laying
        Five golden rings
        Four calling birds
        Three french hens
        Two turtle doves, and
        A partridge in a pear tree",
        "On the seventh day of Christmas, my true love sent to me
        Seven swans a-swimming
        Six geese a-laying
        Five golden rings
        Four calling birds
        Three french hens
        Two turtle doves, and
        A partridge in a pear tree",
        "On the eighth day of Christmas, my true love sent to me
        Eight maids a-milking
        Seven swans a-swimming
        Six geese a-laying
        Five golden rings
        Four calling birds
        Three french hens
        Two turtle doves, and
        A partridge in a pear tree",
        "On the ninth day of Christmas, my true love sent to me
        Nine ladies dancing
        Eight maids a-milking
        Seven swans a-swimming
        Six geese a-laying
        Five golden rings
        Four calling birds
        Three french hens
        Two turtle doves, and
        A partridge in a pear tree",
        "On the tenth day of Christmas, my true love sent to me
        Ten lords a-leaping
        Nine ladies dancing
        Eight maids a-milking
        Seven swans a-swimming
        Six geese a-laying
        Five golden rings
        Four calling birds
        Three french hens
        Two turtle doves, and
        A partridge in a pear tree",
        "On the eleventh day of Christmas, my true love sent to me
        Eleven pipers piping
        Ten lords a-leaping
        Nine ladies dancing
        Eight maids a-milking
        Seven swans a-swimming
        Six geese a-laying
        Five golden rings
        Four calling birds
        Three french hens
        Two turtle doves, and
        A partridge in a pear tree",
        "On the twelfth day of Christmas, my true love sent to me
        Twelve drummers drumming
        Eleven pipers piping
        Ten lords a-leaping
        Nine ladies dancing
        Eight maids a-milking
        Seven swans a-swimming
        Six geese a-laying
        Five golden rings
        Four calling birds
        Three french hens
        Two turtle doves, and
        A partridge in a pear tree",
    ];

    let secret_number = rand::thread_rng().gen_range(1..=10);
    let mut current_string_index = 0;

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if guess == secret_number {
            println!("You guessed it!");
            break;
        } else {
            if current_string_index + 3 > strings.len() {
                println!("Wow you suck at this!");
                break;
            }

            for _ in 0..3 {
                println!("{}", strings[current_string_index]);
                current_string_index += 1;
                sleep(Duration::from_secs(3));
            }
        }
    }
}

