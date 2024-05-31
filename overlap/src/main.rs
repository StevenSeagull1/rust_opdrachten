use image::{DynamicImage, GenericImageView, Rgba};
use std::path::Path;
use image::GenericImage;
use image::RgbaImage;
const TOLERANCE:u8 = 25;
const TOLERANCE_BLACK:i16 = 50;
//alle foto's openen en verwerken door de functies
//tot slot de bewerkte foto opslaan
// fn main() {
//     let input_path = Path::new("selfies/selfie02.png");
//     let reference_path = Path::new("selfies/leeg01.png");
//     let output_path = Path::new("ja.png");

//     let input_image = 
//     image::open(&input_path).expect("Failed to open input image");
//     let reference_image =
//      image::open(&reference_path).expect("Failed to open reference image");

//     let output_image = marker(&input_image, &reference_image);

//     output_image.save(&output_path).expect("Failed to save output image");
// }

// //de eertse 70 y-as regels wit maken omdat daar toch niemand zit
// //vervolgens wordt alles netjes opgedeeld in vierkanten en die vierkanten ook
// //weer in subvierkanten
// pub fn marker(input_image: &DynamicImage, reference_image: &DynamicImage) -> 
// DynamicImage {
//     let (width, height) = input_image.dimensions();
//     let mut output_image = input_image.clone();

//     // Maak de eerste 70 rijen van de Y-as wit
//     for y in 0..80 {
//         for x in 0..width {
//             if y < height {
//                 output_image.put_pixel(x, y, Rgba([255, 255, 255, 255]));
//             }
//         }
//     }

//     let square_width = width / 6;
//     let square_height = height / 6;
//     let sub_square_width = square_width / 5;
//     let sub_square_height = square_height / 5;

//     process_squares(&mut output_image, reference_image, width, height, square_width, 
//         square_height, sub_square_width, sub_square_height);

//     output_image
// }

// fn process_squares(output_image: &mut DynamicImage, reference_image: 
//     &DynamicImage, width: u32, height: u32, 
//     square_width: u32, square_height: u32, sub_square_width: u32, 
//     sub_square_height: u32) {
//     for i in 0..6 {
//         for j in 0..6 {
//             let x_start = i * square_width;
//             let y_start = j * square_height;

//             process_square(output_image, reference_image, width, height,
//                  x_start, y_start, 
//                 square_width, square_height, sub_square_width, 
//                 sub_square_height);
//         }
//     }
// }
// //vanaf hier gaat hij per blok kijken
// fn process_square(output_image: &mut DynamicImage, reference_image: 
//     &DynamicImage, width: u32, height: u32, 
//     x_start: u32, y_start: u32, square_width: u32, square_height: u32, 
//     sub_square_width: u32, sub_square_height: u32) {
//     let tolerance = 0;
    
//     let total_pixels = square_width * square_height;
//     let mut similar_pixel_count: u32 = 0;
//         //hier word gekeken naar hoeveel pixels hetzelfde zijn op 2 de foto's
//         // als een pixel hetzelfde ongeveer is op foto 1 en 2 wordt similar_pixel_count
//         //verhoogt
//     for x in x_start..(x_start + square_width) {
//         for y in y_start..(y_start + square_height) {
//             if x < width && y < height {
//                 let pixel = output_image.get_pixel(x, y).0;
//                 let reference_pixel = reference_image.get_pixel(x, y).0;
//                 if pixels_are_similar(pixel, reference_pixel, tolerance) {
//                     similar_pixel_count += 1;
//                 }
//             }
//         }
//     }
// //als de helft van een blok ongeveer hetzeflde is wordt deze helemaal wit
// //zo niet gaat er per sub square binnen in gekeken worden
//     let ratio = similar_pixel_count as f32 / total_pixels as f32;
//     if ratio > 0.5 {
//         for x in x_start..(x_start + square_width) {
//             for y in y_start..(y_start + square_height) {
//                 if x < width && y < height {
//                     output_image.put_pixel(x, y, Rgba([255, 255, 255, 255]));
//                 }
//             }
//         }
//     } else {
//         process_sub_squares(output_image, reference_image, width, height, 
//             x_start, y_start, 
//             sub_square_width, sub_square_height);
//     }
// }
// //hier word hetzelfde gedaan als hierboven alle pixels die op elkaar lijken
// //worden geteld.
// fn process_sub_squares(output_image: &mut DynamicImage, reference_image:
//      &DynamicImage, width: u32, height: u32,
//      x_start: u32, y_start: u32, sub_square_width: u32, sub_square_height: u32 
//      ) {
//             let tolerance = 3;
    
//     let total_pixels = sub_square_width * sub_square_height;
//     let mut similar_pixel_count: u32 = 0;

//     for x in x_start..(x_start + sub_square_width) {
//         for y in y_start..(y_start + sub_square_height) {
//             if x < width && y < height {
//                 let pixel = output_image.get_pixel(x, y).0;
//                 let reference_pixel = reference_image.get_pixel(x, y).0;
//                 if pixels_are_similar(pixel, reference_pixel, tolerance) {
//                     similar_pixel_count += 1;
//                 }
//             }
//         }
//     }
//     //als het blok 50% hetzelfde is dan wordt het wit gemaakt
//     let ratio = similar_pixel_count as f32 / total_pixels as f32;
//     if ratio > 0.5 {
//         for x in x_start..(x_start + sub_square_width) {
//             for y in y_start..(y_start + sub_square_height) {
//                 if x < width && y < height {
//                     output_image.put_pixel(x, y, Rgba([255, 255, 255, 255]));
//                 }
//             }
//         }
//     }
//     for sub_i in 0..5 {
//         for sub_j in 0..5 {
//             let sub_x_start = x_start + sub_i * sub_square_width;
//             let sub_y_start = y_start + sub_j * sub_square_height;

//             let mut sub_pixel_count = 0;

//             for x in sub_x_start..(sub_x_start + sub_square_width) {
//                 for y in sub_y_start..(sub_y_start + sub_square_height) {
//                     if x < width && y < height {
//                         let blue = output_image.get_pixel(x, y).0[2] as u32;
//                         let red = output_image.get_pixel(x, y).0[0] as u32;
//                         if blue > 100 && red < 50 {
//                             sub_pixel_count += 1;
//                         }
//                     }
//                 }
//             }

//             let sub_total_pixels = sub_square_width * sub_square_height;
//             let sub_ratio = sub_pixel_count as f32 / sub_total_pixels as f32;
//             //als een sub vierkant 95% hetzelfde is moet alles wit worden
//             if sub_ratio > 0.95 {
//                 for x in sub_x_start..(sub_x_start + sub_square_width) {
//                     for y in sub_y_start..(sub_y_start + sub_square_height) {
//                         if x < width && y < height {
//                             output_image.put_pixel(x, y, Rgba([255, 255, 
//                                 255, 255]));
//                         }
//                     }
//                 }
//                 //als er maar een klein gedeelte hetzelfde is moeten alleen de 
//                 // blauwe pixels weg
//             } else if sub_ratio > 0.02 {
//                 for x in sub_x_start..(sub_x_start + sub_square_width) {
//                     for y in sub_y_start..(sub_y_start + sub_square_height) {
//                         if x < width && y < height {
//                             let blue = output_image.get_pixel(x, y).0[2] as u32;
//                             let red = output_image.get_pixel(x, y).0[0] as u32;
//                             if blue > 100 && red < 50 {
//                                 output_image.put_pixel(x, y, Rgba([255, 255,
//                                      255, 255]));
//                             }
//                         }
//                     }
//                 }
//             }
//         }
//     }
// }
// //hier wordt berekend wat het verschil is tussen de pixels tussen de 2 foto's 
// //als het verschil is dan de tolerance dan wordt er een false teruggestuurd
fn pixels_are_similar1(pixel1: [u8; 4], pixel2: [u8; 4], tolerance: u8) -> bool {
    let r_diff = (pixel1[0] as i16 - pixel2[0] as i16).abs();
    let g_diff = (pixel1[1] as i16 - pixel2[1] as i16).abs();
    let b_diff = (pixel1[2] as i16 - pixel2[2] as i16).abs();

    r_diff <= tolerance as i16 && g_diff <= tolerance as i16 && b_diff <= 
    tolerance as i16
}

fn pixels_are_similar2(pixel1: [u8; 4], pixel2: [u8; 4]) -> bool {
    let r1 = pixel1[0]  as f32 + 10.0;
    let g1 = pixel1[1]  as f32 + 10.0;
    let b1 = pixel1[2]  as f32 + 10.0;
 
    let r2 = pixel2[0]  as f32 + 10.0;
    let g2 = pixel2[1]  as f32 + 10.0;
    let b2 = pixel2[2]  as f32 + 10.0;

    let t: f32 = 25.0;

    if (r1 < t && g1 < t && b1 < t) || (r2 < t && g2 < t && b2 < t){
        let same = pixels_are_similar1(pixel1, pixel2, TOLERANCE);
        return same
    }
     
    let Av = 1.0;
    let x = ((r1 / r2 - Av).abs() + (g1 / g2 - Av).abs() + (b1 / b2 - Av).abs()) / 3.0;
    let tolerance = 0.5;
    let x = ((r1 / r2 - Av).abs() + (g1 / g2 - Av).abs() + (b1 / b2 - Av).abs()) / 3.0;

     x <= tolerance
}

fn remove_background(base_image: &DynamicImage, comparison_image: 
    &DynamicImage, tolerance: f32) -> RgbaImage {
    let (width, height) = base_image.dimensions();
    let mut output = RgbaImage::new(width, height);

    for y in 0..height {
        for x in 0..width {
            let base_pixel = base_image.get_pixel(x, y).0;
            let comparison_pixel = comparison_image.get_pixel(x, y).0;
            // let result = pixels_are_similar2(base_pixel, comparison_pixel);
            // println!("{}", result);
            if pixels_are_similar2(base_pixel, comparison_pixel) {

                output.put_pixel(x, y, base_image.get_pixel(x, y));
            }
        }
    }

    output
}

fn main() {
    let base_image = image::open("selfies/selfie02.png").unwrap();
    let comparison_image = image::open("selfies/leeg01.png").unwrap();
    
    let tolerance = 0.1; 
    let output_image = remove_background(&base_image, &comparison_image,
        tolerance);

    output_image.save("ja.png").unwrap();
}