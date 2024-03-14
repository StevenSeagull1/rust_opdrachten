use image::{DynamicImage, GenericImageView, Rgb, RgbImage};
use image::Pixel;
use image_similarity::metrics::image_similarity::similarity_sad;
fn main() {
    // Load the images
    let image_a = image::open("output_rgb_image.png").unwrap();
    let image_b = image::open("images/test.png").unwrap();

   // Convert images to RGB
    let rgb_image_a = convert_to_rgb(&image_a);
    let rgb_image_b = convert_to_rgb(&image_b);

    // Perform image similarity comparison
    // (Note: image_similarity::metrics::image_similarity::similarity_sad function is used for demonstration)
    let difference = similarity_sad(&rgb_image_a, &rgb_image_b);

    // Print the similarity difference
    println!("Similarity Difference: {:?}", difference);
}

fn convert_to_rgb(img: &DynamicImage) -> RgbImage {
    // Create a new RGB imagea
    <DynamicImage as Clone>::clone(&img).into_rgb8()
}