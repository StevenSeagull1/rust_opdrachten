use wasm_bindgen::prelude::wasm_bindgen;

mod utils;

// Functie om een afbeelding in grijswaarden om te zetten
#[wasm_bindgen]
pub fn rgb_to_gray(image_data: Vec<u8>, width: u32, height: u32) -> Vec<u8> {
    let mut gray_image_data = vec![0; (width * height * 4) as usize];
    
    for y in 0..height {
        for x in 0..width {
            let index = (y * width + x) as usize;
            let pixel_index = index * 4;
            let gray_value = (0.299 * image_data[pixel_index] as f32 + 
                              0.587 * image_data[pixel_index + 1] as f32 + 
                              0.114 * image_data[pixel_index + 2] as f32) as u8;

            // Set the gray value for each component and keep alpha as it is
            gray_image_data[pixel_index] = gray_value;
            gray_image_data[pixel_index + 1] = gray_value;
            gray_image_data[pixel_index + 2] = gray_value;
            gray_image_data[pixel_index + 3] = image_data[pixel_index + 3];
        }
    }
    gray_image_data
}


// Functie voor het detecteren van randen met Sobel-operator
#[wasm_bindgen]
pub fn marker(image_data: Vec<u8>, width: u32, height: u32) -> Vec<u8> {
    let mut output_image_data = image_data.clone();

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
                        let pixel_index = (((y * width + x) * 4) as usize);
                        let blue = output_image_data[pixel_index + 2] as u32;
                        let green = output_image_data[pixel_index + 1] as u32;
                        let red = output_image_data[pixel_index] as u32;
                        if blue > red && blue > green {
                            blue_pixel_count += 1;
                            total_blue += blue;
                        }
                    }
                }
            }
            let average_blue = total_blue / (square_width * square_height);

            let total_pixels = square_width * square_height;
            let blue_ratio = blue_pixel_count as f32 / total_pixels as f32;
            if blue_ratio > 0.98 {
                for x in x_start..(x_start + square_width) {
                    for y in y_start..(y_start + square_height) {
                        if x < width && y < height {
                            let pixel_index = (((y * width + x) * 4) as usize);
                            output_image_data[pixel_index] = 255;
                            output_image_data[pixel_index + 1] = 255;
                            output_image_data[pixel_index + 2] = 255;
                            output_image_data[pixel_index + 3] = 255;
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
                                    let pixel_index = (((y * width + x) * 4) as usize);
                                    let blue = output_image_data[pixel_index + 2] as u32;
                                    let green = output_image_data[pixel_index + 1] as u32;
                                    let red = output_image_data[pixel_index] as u32;
                                    if blue > red && blue > green {
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
                                        let pixel_index = (((y * width + x) * 4) as usize);
                                        output_image_data[pixel_index] = 255;
                                        output_image_data[pixel_index + 1] = 255;
                                        output_image_data[pixel_index + 2] = 255;
                                        output_image_data[pixel_index + 3] = 255;
                                    }
                                }
                            }
                        } else if sub_blue_ratio > 0.04 {
                            for x in sub_x_start..(sub_x_start + sub_square_width) {
                                for y in sub_y_start..(sub_y_start + sub_square_height) {
                                    if x < width && y < height {
                                        let pixel_index = (((y * width + x) * 4) as usize);
                                        let blue = output_image_data[pixel_index + 2] as u32;
                                        let green = output_image_data[pixel_index + 1] as u32;
                                        let red = output_image_data[pixel_index] as u32;
                                        if blue > red && blue > green {
                                            output_image_data[pixel_index] = 255;
                                            output_image_data[pixel_index + 1] = 255;
                                            output_image_data[pixel_index + 2] = 255;
                                            output_image_data[pixel_index + 3] = 255;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    output_image_data
}
