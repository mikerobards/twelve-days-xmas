// A program in Rust that prints the lyrics to the Christmas carol "The Twelve Days of Christmas"

// A function that returns the ordinal number as a string
fn ordinal(n: u32) -> String {
    match n {
        1 => "first".to_string(),
        2 => "second".to_string(),
        3 => "third".to_string(),
        4 => "fourth".to_string(),
        5 => "fifth".to_string(),
        6 => "sixth".to_string(),
        7 => "seventh".to_string(),
        8 => "eighth".to_string(),
        9 => "ninth".to_string(),
        10 => "tenth".to_string(),
        11 => "eleventh".to_string(),
        12 => "twelfth".to_string(),
        _ => panic!("Invalid number"),
    }
}

// A function that returns the gift for each day as a string
fn gift(n: u32) -> String {
    match n {
        1 => "a partridge in a pear tree".to_string(),
        2 => "two turtle doves".to_string(),
        3 => "three French hens".to_string(),
        4 => "four calling birds".to_string(),
        5 => "five gold rings".to_string(),
        6 => "six geese a-laying".to_string(),
        7 => "seven swans a-swimming".to_string(),
        8 => "eight maids a-milking".to_string(),
        9 => "nine ladies dancing".to_string(),
        10 => "ten lords a-leaping".to_string(),
        11 => "eleven pipers piping".to_string(),
        12 => "twelve drummers drumming".to_string(),
        _ => panic!("Invalid number"),
    }
}

// The main function that prints the lyrics
fn main() {
    // A loop for each day from 1 to 12
    for day in 1..=12 {
        // Print the first line with the ordinal number
        println!(
            "On the {} day of Christmas, my true love gave to me",
            ordinal(day)
        );

        // A loop for each gift from the current day to 1
        for gifts in (1..=day).rev() {
            // Print the gift with a comma or a period at the end
            if gifts == 1 {
                // If it is the first day, print "and" before the gift
                if day == 1 {
                    println!("{}", gift(gifts));
                    println!();
                } else {
                    println!("and {}", gift(gifts));
                    println!();
                }
            } else {
                println!("{},", gift(gifts));
            }
        }
    }
}
