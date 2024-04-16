use image::{RgbImage, Rgb, imageops};
use imageproc::edges::canny;

fn canny_edge_detection(image: &RgbImage) -> RgbImage {
    let gray_image = imageops::colorops::grayscale(image);

    let edges = canny(&gray_image, 25.0, 50.0);

    let mut result_image = RgbImage::new(edges.width(), edges.height());
    for y in 0..edges.height() {
        for x in 0..edges.width() {
            let pixel_value = edges.get_pixel(x, y)[0];
            let rgb_pixel = Rgb([pixel_value, pixel_value, pixel_value]);
            result_image.put_pixel(x, y, rgb_pixel);
        }
    }
    result_image
}

fn hair_edge_detection(image: &RgbImage) -> RgbImage {
    let lower_hair_color = Rgb([50, 30, 30]);
    let upper_hair_color = Rgb([150, 130, 130]);

    let mut hair_mask = RgbImage::new(image.width(), image.height());

    for y in 0..image.height() {
        for x in 0..image.width() {
            let pixel = image.get_pixel(x, y);
            if pixel[0] >= lower_hair_color[0] && pixel[0] <= upper_hair_color[0] &&
               pixel[1] >= lower_hair_color[1] && pixel[1] <= upper_hair_color[1] &&
               pixel[2] >= lower_hair_color[2] && pixel[2] <= upper_hair_color[2] {
                hair_mask.put_pixel(x, y, Rgb([255, 255, 255]));
            } else {
                hair_mask.put_pixel(x, y, Rgb([0, 0, 0]));
            }
        }
    }

    hair_mask
}

fn main() {
    // Load your input image
    let input_image = image::open("blue_with_marker.jpg").expect("Failed to open image").to_rgb8();

    // Apply hair edge detection
    let hair_mask = hair_edge_detection(&input_image);

    // Apply Canny edge detection on the hair edges
    let output_image = canny_edge_detection(&hair_mask);

    // Save the output image
    output_image.save("hair_edges.jpg").expect("Failed to save image");
}
