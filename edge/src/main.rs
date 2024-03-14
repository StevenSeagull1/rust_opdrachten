use image::{RgbImage, Rgb};

fn main() {
    // afbeelding openen
    let img = image::open("images/rust.png").unwrap().to_rgb8();

    // afbeelding grijs maken
    let gray_img = rgb_to_gray(&img);

    // de afbeelding randen geven
    let edges = sobel_edge_detection(&gray_img);

    // afbeelding opslaan
    edges.save("output_image.jpg").unwrap();
}

// de fucntie om alles te veranderen in grijs
fn rgb_to_gray(image: &RgbImage) -> RgbImage {
    //dit pakt de hoogte en breedte van de afbeelding
    let (width, height) = image.dimensions();
    let mut gray_image = RgbImage::new(width, height);
    // dit itereert over elke pixel en maakt het grijs
    for y in 0..height {
        for x in 0..width {
            let pixel = image.get_pixel(x, y);
            //de formule om alles de juiste kleur grijs te geven
            let gray_value = (0.2989 * pixel[0] as f32 + 0.5870 * pixel[1] as f32 + 0.1140 * pixel[2] as f32) as u8;
            gray_image.put_pixel(x, y, Rgb([gray_value, gray_value, gray_value]));
        }
    }

    gray_image
}

// Apply Sobel operator for edge detection
fn sobel_edge_detection(image: &RgbImage) -> RgbImage {
    let (width, height) = image.dimensions();
    let mut edges = RgbImage::new(width, height);

    let sobel_x: [[i32; 3]; 3] = [[-1, 0, 1],
                                    [-2, 0, 2],
                                    [-1, 0, 1]];

    let sobel_y: [[i32; 3]; 3] = [[-1, -2, -1],
                                    [ 0,  0,  0],
                                    [ 1,  2,  1]];

    for y in 1..height-1 {
        for x in 1..width-1 {
            let mut pixel_x: i32 = 0;                       
            let mut pixel_y: i32 = 0;

            for j in 0..3 {
                for i in 0..3 {
                    let neighbor = image.get_pixel(x + i as u32 - 1, y + j as u32 - 1); // Cast i and j to u32
                    let gray_value = neighbor[0] as i32; // Assuming grayscale

                    pixel_x += gray_value * sobel_x[j][i];
                    pixel_y += gray_value * sobel_y[j][i];
                }
            }


           let magnitude = ((pixel_x.pow(2) + pixel_y.pow(2)) as f64).sqrt() as u8;
            edges.put_pixel(x, y, Rgb([magnitude, magnitude, magnitude]));
        }
    }

    edges
}
