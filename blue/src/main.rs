extern crate image;

use image::{GenericImageView, DynamicImage, Rgba};
use crate::image::GenericImage;

fn marker(input_image: &DynamicImage) -> DynamicImage {
    let (width, height) = input_image.dimensions();

    let mut output_image = input_image.clone();

    let square_width = width / 6;
    let square_height = height / 6;
    let sub_square_width = square_width / 5;
    let sub_square_height = square_height / 5;

    for i in 0..6 {
        for j in 0..6 {
            let x_start = i * square_width;
            let y_start = j * square_height;

            let mut blue_pixel_count: u32 = 0;
            let mut total_blue: u32 = 0;
            for x in x_start..(x_start + square_width) {
                for y in y_start..(y_start + square_height) {
                    if x < width && y < height {
                        let pixel = output_image.get_pixel(x, y);
                        if pixel[2] > pixel[0] && pixel[2] > pixel[1] {
                            blue_pixel_count += 1;
                            total_blue += pixel[2] as u32;
                        }
                    }
                }
            }
            let gemiddelde = total_blue / (square_width * square_height);

            let total_pixels = square_width * square_height;
            let blue_ratio = blue_pixel_count as f32 / total_pixels as f32;
            if blue_ratio > 0.98 {
                for x in x_start..(x_start + square_width) {
                    for y in y_start..(y_start + square_height) {
                        if x < width && y < height {
                            output_image.put_pixel(x, y, Rgba([gemiddelde as u8, gemiddelde as u8, gemiddelde as u8, 255]));
                        }
                    }
                }
            } else {
                 for sub_i in 0..5 {
                    for sub_j in 0..5 {
                        let sub_x_start = x_start + sub_i * sub_square_width;
                        let sub_y_start = y_start + sub_j * sub_square_height;

                        let mut sub_blue_pixel_count = 0;

                        for x in sub_x_start..(sub_x_start + sub_square_width) {
                            for y in sub_y_start..(sub_y_start + sub_square_height) {
                                if x < width && y < height {
                                    let pixel = output_image.get_pixel(x, y);
                                    if pixel[2] > pixel[0] && pixel[2] > pixel[1] {
                                        sub_blue_pixel_count += 1;
                                    }
                                }
                            }
                        }

                        let sub_total_pixels = sub_square_width * sub_square_height;
                        let sub_blue_ratio = sub_blue_pixel_count as f32 / sub_total_pixels as f32;
                        if sub_blue_ratio > 0.99 {
                            // Als het subvierkant meer dan 98% blauw is, maak het volledig wit
                            for x in sub_x_start..(sub_x_start + sub_square_width) {
                                for y in sub_y_start..(sub_y_start + sub_square_height) {
                                    if x < width && y < height {
                                        output_image.put_pixel(x, y, Rgba([255, 255, 255, 255]));
                                    }
                                }
                            }
                        }else if sub_blue_ratio > 0.04{
                            for x in sub_x_start..(sub_x_start + sub_square_width) {
                                for y in sub_y_start..(sub_y_start + sub_square_height) {
                                    let pixel = output_image.get_pixel(x, y);
                                    if pixel[2] > pixel[0] && pixel[2] > pixel[1] {
                                        output_image.put_pixel(x, y, Rgba([255, 255, 255, 255]));
                                    }
                                }
                            }
                        } 
                    }
                }
            }
        }
    }
    output_image
}


fn main() {
    // Laad de afbeelding
    let img = image::open("output.jpg").unwrap();

    // Vervang het blauw door wit
    let processed_img = marker(&img);

    // Bewaar de bewerkte afbeelding
    processed_img.save("blue_with_marker.jpg").unwrap();
}