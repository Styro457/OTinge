mod engine;
use engine::layer::Layer;
use crate::engine::document::Document;
use crate::engine::image::pixels::rgba::RGBAPixel;
use crate::engine::painters::solid_painter::SolidPainter;

fn print_color(r: u8, g: u8, b: u8) {
    print!("\x1b[48;2;{};{};{}m  \x1b[0m", r, g, b); // colored block
}

fn main() {
    println!("Hello, world!");
    let mut document : Document<RGBAPixel<u8>> = Document::new(10, 10);

    let red: RGBAPixel<u8> = RGBAPixel::new(255, 0, 0, 255);
    let blue: RGBAPixel<u8> = RGBAPixel::new(0, 0, 255, 80);
    let red_solid: SolidPainter<RGBAPixel<u8>> = SolidPainter::new(red);
    let blue_solid: SolidPainter<RGBAPixel<u8>> = SolidPainter::new(blue);
    let blue_solid2: SolidPainter<RGBAPixel<u8>> = SolidPainter::new(blue);

    let red_layer: Layer<RGBAPixel<u8>> = Layer::new("Red Layer", 10, 10, Some(Box::new(red_solid)));
    let mut blue_layer: Layer<RGBAPixel<u8>> = Layer::new("Blue Layer", 3, 3, Some(Box::new(blue_solid)));
    blue_layer.transform.x = 1;
    blue_layer.transform.y = 1;
    let mut blue_layer2: Layer<RGBAPixel<u8>> = Layer::new("Blue Layer", 2, 2, Some(Box::new(blue_solid2)));
    blue_layer2.transform.x = 1;
    blue_layer2.transform.y = 1;

    document.base_layer.add_child(red_layer);
    document.base_layer.add_child(blue_layer);
    document.base_layer.add_child(blue_layer2);


    let image = document.render();
    for y in 0..image.height {
        for x in 0..image.width {
            let pixel = image.get_pixel(x, y);
            print_color(pixel.r, pixel.g, pixel.b);
        }
        println!();
    }
}
