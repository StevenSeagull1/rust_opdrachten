extern crate image;
mod background;
use background::{marker};

fn main() {
    // Laad de afbeelding
    let img = image::open("selfies/selfie02.png").unwrap();

    // Vervang het blauw door wit
    let processed_img = marker(&img);

    // Bewaar de bewerkte afbeelding
    processed_img.save("ja1.png").unwrap();

}