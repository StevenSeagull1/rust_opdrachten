fn main() {
    //arrays met alle dagen en dingen die herhaald worden
    let dagen = ["first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth", "tenth", "eleventh", "twelfth"];
    let extra = ["a partridge in a pear tree", "two turtle doves", "three French hens", "four calling birds", "five golden rings", "six geese a-laying", "seven swans a-swimming", "eight maids a-milking", "nine ladies dancing", "ten lords a-leaping", "eleven pipers piping", "twelve drummers drumming"];

//loop 12 keer want erD zijn 12 dagen in het liedje
    for i in 0..12 {
        println!("On the {} day of Christmas, my true love sent to me:", dagen[i]);
        //loop net zo vaak als dat er dagen zijn geweest.
        // is er bvb 2 keer geloopt dan print hij 2 keer de extra's
        //.rev() zorgt ervoor dat het gerevesed word. anders waren de lyrics verkeerd om geprint.
        for j in (0..=i).rev() {
                println!("{}", extra[j]);
        }
        //wit regel anders zijn er geen witregels tussen elk refrein.
        println!();
    }
}
