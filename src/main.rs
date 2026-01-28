mod v1;
mod engine;
mod gpu_render;

use crate::v1::ui::canvas::app::run;


fn main() {
    run().expect("TODO: panic message");
}