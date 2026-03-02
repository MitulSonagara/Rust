// =======================================================
// 🦀 Twelve Days of Christmas
// Print the full song using loops
// =======================================================

fn main() {
    let days = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];

    let gifts = [
        "A partridge in a pear tree.",
        "Two turtle doves,",
        "Three French hens,",
        "Four calling birds,",
        "Five golden rings,",
        "Six geese a laying,",
        "Seven swans a swimming,",
        "Eight maids a milking,",
        "Nine ladies dancing,",
        "Ten lords a leaping,",
        "Eleven pipers piping,",
        "Twelve drummers drumming,",
    ];

    for day in 0..12 {
        println!(
            "\nOn the {} day of Christmas my true love sent to me:",
            days[day]
        );

        for gift_index in (0..=day).rev() {
            if day > 0 && gift_index == 0 {
                println!("And {}", gifts[gift_index]);
            } else {
                println!("{}", gifts[gift_index]);
            }
        }
    }
}
