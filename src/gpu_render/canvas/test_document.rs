use crate::engine::document::Document;
use crate::engine::image::buffer::ImageBuffer;
use crate::engine::image::buffer::rgba_u8_buffer::RGBAu8Buffer;
use crate::engine::image::color::Color;
use crate::engine::layer::Layer;
use crate::engine::painters::image::ImagePainter;
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
    layer1.transform.position = Pos2::new(250, 250);
    document.base_layer.add_child(layer1);

    let mut layer2 = Layer::new("Layer 2", None);
    let solid_blue_painter = Box::new(SolidPainter::new(&mut document,
                                                       Pos2::new(50, 50),
                                                       Color::RGBAu8(0, 0, 255, 100),
    ));
    layer2.painter = Some(solid_blue_painter);
    layer2.transform.position = Pos2::new(250, 250);
    layer2.transform.rotation = 3.14/4.0;
    document.base_layer.add_child(layer2);

    let mut layer3 = Layer::new("Layer 3", None);
    let solid_blue_painter = Box::new(SolidPainter::new(&mut document,
                                                        Pos2::new(50, 50),
                                                        Color::RGBAu8(0, 0, 255, 255),
    ));
    layer3.painter = Some(solid_blue_painter);
    layer3.transform.position = Pos2::new(240, 250);
    layer3.transform.rotation = 3.14/4.0;
    document.base_layer.add_child(layer3);

    let mut layer4 = Layer::new("Layer 4", None);
    let image_painter = Box::new(ImagePainter::new(&mut document,
                                                        "image.png",
    ));
    layer4.painter = Some(image_painter);
    layer4.transform.position = Pos2::new(250, 250);
    layer4.opacity = 0.5;
    document.base_layer.add_child(layer4);

    document
}