fn main() {
    let ith = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelveth",
    ];

    let verses = [
        "A partridge in a pear tree",
        "Two turtle doves, and",
        "Three french hens",
        "Four calling birds",
        "Five golden rings",
        "Six geese a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming",
    ];

    let mut n = 0;

    println!("The Twelve Days of Christmas\n");

    for d in ith.iter() {
        println!("On the {} day of Christmas, my true love sent to me", d);

        for m in 0..n + 1 {
            println!("{}", verses[m]);
        }
        n += 1;

        println!();
    }
}
