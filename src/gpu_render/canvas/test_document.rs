use crate::engine::document::Document;
use crate::engine::image::buffer::ImageBuffer;
use crate::engine::image::buffer::rgba_f32_buffer::RGBAf32Buffer;
use crate::engine::image::buffer::rgba_u8_buffer::RGBAu8Buffer;
use crate::engine::image::color::Color;
use crate::engine::layer::Layer;
use crate::engine::painters::solid::SolidPainter;
use crate::engine::utils::math::pos2::Pos2;

pub fn create_fake_document() -> Document {
    let mut document = Document::new(500, 500, Box::new(RGBAu8Buffer::new(Pos2::ZERO)));
    let mut layer1 = Layer::new("Layer 1", None);
    let solid_red_painter = Box::new(SolidPainter::new(&mut document,
                                                       Pos2::new(500, 500),
                                                       Color::RGBAu8(255, 0, 0, 255),
    ));
    layer1.painter = Some(solid_red_painter);
    let mut layer2 = Layer::new("Layer 2", None);
    let solid_green_painter = Box::new(SolidPainter::new(&mut document,
                                                         Pos2::new(500, 500),
                                                         Color::RGBAu8(255, 255, 0, 255),
    ));
    layer2.painter = Some(solid_green_painter);
    document.base_layer.add_child(layer1);
    document.base_layer.add_child(layer2);
    document
}