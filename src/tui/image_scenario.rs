
use magick_rust::{CompositeOperator, MagickWand};
use rfd::FileDialog;

use crate::channel::split_vector::SplitVector;

use super::shared::*;

pub fn image_scenario_screen(terminal: &mut Cursive, gen_matrix: Arc<Mutex<GenMatrix>>, hadamards: Arc<Mutex<Hadamards>>, channel: Arc<Mutex<Channel>>) {
    let channel2 = Arc::new(Mutex::new(channel.lock().unwrap().clone()));

    let files = FileDialog::new()
        .add_filter("BMP Files", &["bmp"])
        .set_directory("/")
        .pick_file();

    if let Some(mut image_path) = files {
        let mut raw_pixels_image = MagickWand::new();
        match raw_pixels_image.read_image(&String::from_utf8_lossy(image_path.as_os_str().as_encoded_bytes())) {
            Ok(_) => {},
            Err(err) => {
                error_popup(terminal, "Paveikslėlio nuskaitymo klaida", err.0);
                return;
            },
        };
        let mut encoded_pixels_image = raw_pixels_image.clone();

        let (width, height) = (raw_pixels_image.get_image_width(), raw_pixels_image.get_image_height());
        let pixel_data = match raw_pixels_image.export_image_pixels(0, 0, width, height, "RGBA") {
            Some(x) => x,
            None => {
                error_popup(terminal, "Paveikslėlio nuskaitymo klaida", "Nepavyko gauti pikselių");
                return;                
            },
        };

        let gm = gen_matrix.lock().unwrap();

        let bits = BinaryVector::from_bits(pixel_data);
        let mut raw_pixels = match bits {
            Some(x) => SplitVector::new(&x, gm.muller()),
            None => {
                error_popup(terminal, 
                    "Įvesties klaida", 
                    format!("Paveikslėlyje nėra pikselių")
                );
                return;
            },
        };

        let mut encoded_pixels = raw_pixels.clone();
        encoded_pixels.encode(&gm);

        channel.lock().unwrap().send_multiple(&mut raw_pixels);
        
        channel2.lock().unwrap().send_multiple(&mut encoded_pixels);
        encoded_pixels.decode(&hadamards.lock().unwrap());

        let raw_pixel_bytes = raw_pixels.to_bytes();
        let encoded_pixel_bytes = encoded_pixels.to_bytes();
        
        match raw_pixels_image.import_image_pixels(0, 0, width, height, &raw_pixel_bytes, "RGBA") {
            Ok(_) => { },
            Err(err) => {
                error_popup(terminal, "Paveikslėlio sudarymo klaida", err.0);
                return;
            },
        }

        match encoded_pixels_image.import_image_pixels(0, 0, width, height, &encoded_pixel_bytes, "RGBA") {
            Ok(_) => { },
            Err(err) => {
                error_popup(terminal, "Paveikslėlio sudarymo klaida", err.0);
                return;
            },
        }
        
        let (image_width, image_height) = (raw_pixels_image.get_image_width(), raw_pixels_image.get_image_height());
        match raw_pixels_image.extend_image(image_width << 1, image_height, 0, 0) {
            Ok(_) => { },
            Err(err) => {
                error_popup(terminal, "Paveikslėlio sudarymo klaida", err.0);
                return;
            },
        }

        let compose_res = raw_pixels_image.compose_images(
            &encoded_pixels_image, 
            CompositeOperator::Copy, 
            true, 
            image_width as isize,
            0
        );
        match compose_res {
            Ok(_) => { },
            Err(err) => {
                error_popup(terminal, "Paveikslėlio sudarymo klaida", err.0);
                return;
            },
        }

        let mut new_filename = image_path.file_stem().unwrap().to_owned();
        new_filename.push("_split_view.bmp");
        image_path.set_file_name(new_filename);

        match raw_pixels_image.write_image(&String::from_utf8_lossy(image_path.as_os_str().as_encoded_bytes())) {
            Ok(_) => { },
            Err(err) => {
                error_popup(terminal, "Paveikslėlio išsaugojimo klaida", err.0);
                return;
            },
        }

        match open::that_detached(&image_path) {
            Ok(_) => { },
            Err(err) => {
                error_popup(terminal, "Paveikslėlio atidarymo klaida", err.to_string());
                return;
            },
        }
    }
}