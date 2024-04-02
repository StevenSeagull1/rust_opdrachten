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


// // Functie voor het detecteren van randen met Sobel-operator
#[wasm_bindgen]
pub fn sobel_edge_detection(image_data: Vec<u8>, width: u32, height: u32) -> Vec<u8> {
    let mut edge_image_data = vec![0; (width * height * 4) as usize];

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
                    let pixel_index = (((y + j - 1) * width + (x + i - 1)) * 4) as usize;
                    let gray_value = image_data[pixel_index] as i32;
                    pixel_x += gray_value * sobel_x[j as usize][i as usize];
                    pixel_y += gray_value * sobel_y[j as usize][i as usize];
                }
            }

            let magnitude = ((pixel_x.pow(2) + pixel_y.pow(2)) as f64).sqrt() as u8;
            let index = ((y * width + x) * 4) as usize; 
            edge_image_data[index] = magnitude;
            edge_image_data[index + 1] = magnitude;
            edge_image_data[index + 2] = magnitude;
            edge_image_data[index + 3] = 255; 
        }
    }

    edge_image_data
}
