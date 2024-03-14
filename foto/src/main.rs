extern crate image;
use crate::image::GenericImageView;
use image::{GenericImage, Rgba};
use crate::image::Pixel;

fn main() {
    // Open de grotere afbeelding
    let mut grotere_afbeelding = image::open("images/unknown.png").unwrap();
    let (_breedte_groot, hoogte_groot) = grotere_afbeelding.dimensions();

    // Open de kleinere afbeelding
    let kleinere_afbeelding = image::open("images/rust.png").unwrap();
    let (breedte_klein, hoogte_klein) = kleinere_afbeelding.dimensions();

    // Bepaal de positie van de kleinere afbeelding (linksonder)
    let positie_x = 0;
    let positie_y = hoogte_groot - hoogte_klein;

    // Plaats de kleinere afbeelding op de grotere afbeelding
    for x in 0..breedte_klein {
        for y in 0..hoogte_klein {
            let pixel_klein = kleinere_afbeelding.get_pixel(x, y);
            let kleur = pixel_klein.channels(); // Haal de RGBA-waarden op
            let kleur_array: [u8; 4] = [kleur[0], kleur[1], kleur[2], kleur[3]]; // Converteer de slice naar een array
            grotere_afbeelding.put_pixel(x + positie_x, y + positie_y, Rgba(kleur_array)); // Plaats de pixel
        }
    }

    // Sla de resulterende afbeelding op
    grotere_afbeelding.save("images/samengevoegde_afbeelding.png").unwrap();
}