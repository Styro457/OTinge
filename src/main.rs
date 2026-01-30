use crate::ui::canvas::app::run;

mod engine;
mod gpu_render;
mod ui;

fn main() {
    run().expect("TODO: panic message");
}