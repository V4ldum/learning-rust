fn main() {
    // Convert Fahrenheit to Celsius and the other way around
    let fahrenheit = 60.0;
    let celsius = 16.0;

    println!("Fahrenheit to Celsius : {:.2} (should be ~16)", fahrenheit_to_celsius(fahrenheit));
    println!("Celsius to Fahrenheit : {:.2} (should be ~60)", celsius_to_fahrenheit(celsius));

    // Print "The twelve Days of Christmas
    println!("\nThe Twelve Days of Christmas\n--------------\n");

    print_the_twelve_days_of_christmas();
}

fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) / 1.8
}

fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    (celsius * 1.8) + 32.0
}

fn print_the_twelve_days_of_christmas() {
    let days = ["first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth", "tenth", "eleventh", "twelfth"];
    let gifts = ["A partridge in a pear tree", "Two turtle doves and", "Three french hens", "Four calling birds", "Five golden rings", "Six geese a-laying", "Seven swans a-swimming", "Eight maids a-milking", "Nine ladies dancing", "Ten lords a-leaping", "Eleven pipers piping", "Twelve drummers drumming"];

    for day in 0..12 {
        println!("On the {} day of Christmas", days[day]);
        println!("My True love sent me");

        for gift in (0..(day + 1)).rev() {
            println!("{}", gifts[gift]);
        }

        println!();
    }
}