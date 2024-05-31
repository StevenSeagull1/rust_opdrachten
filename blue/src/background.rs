use image::{GenericImageView, DynamicImage, Rgba};
use crate::image::GenericImage;
use std::collections::HashMap;
use std::path::Path;

//deze functie verdeelt de afbeedling in hokjes
pub fn marker(input_image: &DynamicImage) -> DynamicImage {
    let (width, height) = input_image.dimensions();
    let mut output_image = input_image.clone();

    let square_width = width / 6;
    let square_height = height / 6;
    let sub_square_width = square_width / 5;
    let sub_square_height = square_height / 5;

    process_squares(&mut output_image, width, height, square_width, 
        square_height, sub_square_width, sub_square_height);

    output_image
}

fn process_squares(output_image: &mut DynamicImage, width: u32, height: u32, 
    square_width: u32, square_height: u32, sub_square_width: u32, 
    sub_square_height: u32) {
    for i in 0..6 {
        for j in 0..6 {
            let x_start = i * square_width;
            let y_start = j * square_height;

            process_square(output_image, width, height, x_start, y_start, 
                square_width, square_height, sub_square_width, 
                sub_square_height);
        }
    }
}

fn process_square(output_image: &mut DynamicImage, width: u32, height: u32, 
    x_start: u32, y_start: u32, square_width: u32, square_height: u32, 
    sub_square_width: u32, sub_square_height: u32) {
    let tolerance = 10;
    if let Some(most_common_pixels) = most_common_pixels_from_memory(
        output_image, x_start, y_start, square_width, square_height) {
        let total_pixels = square_width * square_height;
        let mut similar_pixel_count: u32 = 0;

        for x in x_start..(x_start + square_width) {
            for y in y_start..(y_start + square_height) {
                if x < width && y < height {
                    let pixel = output_image.get_pixel(x, y).0;
                    if most_common_pixels.iter().any(|&common_pixel| 
                        pixels_are_similar(pixel, common_pixel, tolerance)) {
                        similar_pixel_count += 1;
                    }
                }
            }
        }

        let ratio = similar_pixel_count as f32 / total_pixels as f32;
        if ratio > 0.5 {
            for x in x_start..(x_start + square_width) {
                for y in y_start..(y_start + square_height) {
                    if x < width && y < height {
                        output_image.put_pixel(x, y, Rgba([255, 255, 255, 255]));
                    }
                }
            }
        } else {
            process_sub_squares(output_image, width, height, x_start, y_start, 
                sub_square_width, sub_square_height, &most_common_pixels, 
                tolerance);
        }
    }
}

fn process_sub_squares(output_image: &mut DynamicImage, width: u32, height: u32,
     x_start: u32, y_start: u32, sub_square_width: u32, sub_square_height: u32, 
     most_common_pixels: &[[u8; 4]], tolerance: u8) {
    for sub_i in 0..5 {
        for sub_j in 0..5 {
            let sub_x_start = x_start + sub_i * sub_square_width;
            let sub_y_start = y_start + sub_j * sub_square_height;

            let mut sub_pixel_count = 0;

            for x in sub_x_start..(sub_x_start + sub_square_width) {
                for y in sub_y_start..(sub_y_start + sub_square_height) {
                    if x < width && y < height {
                        let pixel = output_image.get_pixel(x, y).0;
                        if most_common_pixels.iter().any(|&common_pixel| 
                            pixels_are_similar(pixel, common_pixel, tolerance)) {
                            sub_pixel_count += 1;
                        }
                    }
                }
            }

            let sub_total_pixels = sub_square_width * sub_square_height;
            let sub_ratio = sub_pixel_count as f32 / sub_total_pixels as f32;

            if sub_ratio > 0.99 {
                for x in sub_x_start..(sub_x_start + sub_square_width) {
                    for y in sub_y_start..(sub_y_start + sub_square_height) {
                        if x < width && y < height {
                            output_image.put_pixel(x, y, Rgba([255, 255, 255, 255]));
                        }
                    }
                }
            }
        }
    }
}

fn most_common_pixels_from_memory(output_image: &DynamicImage, x_start: u32, 
    y_start: u32, square_width: u32, square_height: u32) -> 
    Option<Vec<[u8; 4]>> {
    let mut pixel_count = HashMap::new();

    for x in x_start..(x_start + square_width) {
        for y in y_start..(y_start + square_height) {
            if x < output_image.width() && y < output_image.height() {
                let pixel = output_image.get_pixel(x, y).0;

                if pixel.iter().all(|&channel| channel < 10) {
                    continue;
                }

                *pixel_count.entry(pixel).or_insert(0) += 1;
            }
        }
    }

    let mut pixel_count_vec: Vec<([u8; 4], u32)> = pixel_count.into_iter().collect();
    pixel_count_vec.sort_by(|a, b| b.1.cmp(&a.1));

    let top_3_pixels: Vec<[u8; 4]> = pixel_count_vec.into_iter().take(7).map(
        |(pixel, _)| pixel).collect();

    if !top_3_pixels.is_empty() {
        println!("Top 3 most common pixels: {:?}", top_3_pixels);
        Some(top_3_pixels)
    } else {
        None
    }
}

fn pixels_are_similar(pixel1: [u8; 4], pixel2: [u8; 4], tolerance: u8) -> bool {

     if pixel1[0] >= 50 || pixel2[0] >= 50 {
        return false; 
    }
    let r_diff = (pixel1[0] as i16 - pixel2[0] as i16).abs();
    let g_diff = (pixel1[1] as i16 - pixel2[1] as i16).abs();
    let b_diff = (pixel1[2] as i16 - pixel2[2] as i16).abs();
    let a_diff = (pixel1[3] as i16 - pixel2[3] as i16).abs();

    r_diff <= tolerance as i16 && g_diff <= tolerance as i16 && b_diff <= 
    tolerance as i16 && a_diff <= tolerance as i16
}
// fn average_rgb(image: &DynamicImage)  -> (f32, f32, f32) {
//     let (width, height) = image.dimensions();
//     let mut total_r = 0;
//     let mut total_g = 0;
//     let mut total_b = 0;

//     for x in 0..width {
//         for y in 0..height {
//             let pixel = image.get_pixel(x, y);

//             total_r += pixel[0] as u32;
//             total_g += pixel[1] as u32;
//             total_b += pixel[2] as u32;
//         }
//     }

//     let num_pixels = (width * height) as f32;
//     let average_r = total_r as f32 / num_pixels;
//     let average_g = total_g as f32 / num_pixels;
//     let average_b = total_b as f32 / num_pixels;

//     (average_r, average_g, average_b)
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_average_rgb() {
        // maakt een foto
        let mut test_image = DynamicImage::new_rgba8(100, 100);
        for x in 0..100 {
            for y in 0..100 {
                let r = 0;
                let g = 255;
                let b = 255;
                test_image.put_pixel(x, y, Rgba([r, g, b, 255]));
            }
        }
        
        let expected_average_r = 0.0; 
        let expected_average_g = 255.0; 
        let expected_average_b = 255.0;

        let (actual_average_r, 
            actual_average_g, 
            actual_average_b) = average_rgb(&test_image);

        assert_eq!(actual_average_r, expected_average_r);
        assert_eq!(actual_average_g, expected_average_g);
        assert_eq!(actual_average_b, expected_average_b);
    }
    #[test]
    fn test_marker() {
        let input_image = DynamicImage::new_rgba8(100, 100);
        let output_image = marker(&input_image);
        assert_eq!(output_image.dimensions(), (100, 100));
    }

#[test]
fn test_process_square() {
    let mut input_image = DynamicImage::new_rgba8(20, 20);
    for x in 0..20 {
        for y in 0..20 {
            if (x >= 4 && x < 8) && (y >= 4 && y < 8) {
                input_image.put_pixel(x, y, Rgba([0, 0, 255, 255]));
            } else {
                input_image.put_pixel(x, y, Rgba([255, 255, 255, 255]));
            }
        }
    }

    let mut output_image = input_image.clone();
    let width = 20;
    let height = 20;
    let x_start = 0;
    let y_start = 0;
    let square_width = 6;
    let square_height = 6;
    let sub_square_width = 2;
    let sub_square_height = 2;

    process_square(&mut output_image, width, height, x_start, y_start, 
        square_width, square_height, sub_square_width, sub_square_height);

    for x in 4..8 {
        for y in 4..8 {
            let pixel = output_image.get_pixel(x, y);
            if pixel != Rgba([255, 255, 255, 255]) {
                println!("Fout bij pixel op ({}, {}): {:?}", x, y, pixel);
            }
            assert_eq!(pixel, Rgba([255, 255, 255, 255]));
        }   
    }
}



   #[test]
    fn test_process_sub_squares() {
        let mut input_image = DynamicImage::new_rgba8(20, 20);
        for x in 0..20 {
            for y in 0..20 {
                if (x >= 4 && x < 8) && (y >= 4 && y < 8) {
                    input_image.put_pixel(x, y, Rgba([0, 0, 255, 255]));
                } else {
                    input_image.put_pixel(x, y, Rgba([255, 255, 255, 255]));
                }
            }
        }

        let mut output_image = input_image.clone();
        let width = 20;
        let height = 20;
        let x_start = 0;
        let y_start = 0;
        let sub_square_width = 4;
        let sub_square_height = 4;

        process_sub_squares(&mut output_image, width, height, x_start, y_start,
            sub_square_width, sub_square_height);

        for x in 4..8 {
            for y in 4..8 {
                let pixel = output_image.get_pixel(x, y);
                assert_eq!(pixel, Rgba([255, 255, 255, 255]));
            }
        }

        for x in 0..20 {
            for y in 0..20 {
                if (x >= 4 && x < 8) && (y >= 4 && y < 8) {
                    continue;
                }
                let pixel = output_image.get_pixel(x, y);
                assert_eq!(pixel, Rgba([255, 255, 255, 255]));
            }
        }
    }
}