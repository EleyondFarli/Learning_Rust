const DAYS : [&str; 12]= [
    "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eight", "ninth", "tenth",
    "eleventh", "twelfth"
];
const VERSES: [&str; 12] = [
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
    "Twelve drummers drumming"
];
pub fn sing_twelve_days() {
    for i in 0..12 {
        println!("\n[Verse {}]", i+1);
        println!("On the {} day of Christmas, my true love sent to me", DAYS[i]);
        for j in (0..i+1).rev() {
            println!("{}", VERSES[j]);
        }
    }
}
