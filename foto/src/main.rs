use image::{DynamicImage, GenericImageView};

fn main() {
    //dit zoekt de foto op
    let img = image::open("images/Unknown.png").unwrap();
    // stuur de foto naar de converter
    let grayscale_img = convert_to_grayscale(&img);

    // hier wordt de zwart wit foto opgeslagen
    grayscale_img.save("output_image.jpg").expect("Failed to save grayscale image");
}

fn convert_to_grayscale(img: &DynamicImage) -> DynamicImage {
    // dit maakt de img zwart wit
    img.grayscale()
}