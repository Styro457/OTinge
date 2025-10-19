use super::image::pixel::Pixel;
use super::utils::composition::BlendMode;
use super::utils::position::Transform;
use super::image::buffer::ImageBuffer;

pub trait LayerPainter<P: Pixel> {
    fn paint(&self, buffer: &mut ImageBuffer<P>);
}

pub struct Layer<P: Pixel> {
    pub enabled: bool,
    pub name: String,
    pub width: usize,
    pub height: usize,
    pub transform: Transform,
    pub opacity: f32,
    pub blend_mode: BlendMode,
    pub mask_index: usize,
    pub painter: Option<Box<dyn LayerPainter<P>>>,
    pub effects: Vec<Box<dyn LayerPainter<P>>>,
    pub children: Vec<Layer<P>>
}

impl<P: Pixel> Layer<P> {
    pub fn new(name: &str, width: usize, height: usize, painter: Option<Box<dyn LayerPainter<P>>>) -> Self {
        Self {
            enabled: true,
            name: name.to_string(),
            width,
            height,
            transform: Transform::zero(),
            opacity: 1.0,
            blend_mode: BlendMode::Normal,
            mask_index: 0,
            children: Vec::new(),
            effects: Vec::new(),
            painter
        }
    }

    pub fn add_child(&mut self, child: Layer<P>) {
        self.children.push(child);
    }

    pub fn get_buffer(&mut self) -> ImageBuffer<P> {
        let mut buffer = ImageBuffer::<P>::new(self.width, self.height);
        self.painter.as_ref().map(|p| p.paint(&mut buffer));
        buffer
    }

    pub fn render(&self) -> ImageBuffer<P> {
        println!("Rendering layer '{}'", self.name);
        let mut buffer = ImageBuffer::<P>::new(self.width, self.height);

        for child in &self.children {
            let child_buffer = child.render();
            for y in 0..self.height {
                for x in 0..self.width {
                    let background_pixel = buffer.get_pixel(x, y);
                    if x < child.transform.x || x >= child.width+child.transform.x || y < child.transform.y || y >= child.height+child.transform.y {
                        continue;
                    }
                    let child_pixel = child_buffer.get_pixel(x-child.transform.x, y-child.transform.y);
                    buffer.set_pixel(x, y, child_pixel.blend(background_pixel));
                }
            }
        }
        self.painter.as_ref().map(|p| p.paint(&mut buffer));
        buffer
    }

    pub fn get_layer_count(&self) -> usize {
        let mut count = 1;
        for child in &self.children {
            count += child.get_layer_count();
        }
        count
    }

}
