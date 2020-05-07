use three_d::*;

pub trait Plotter {
    fn zoom(&mut self, delta: f32);
    fn translate(&mut self, delta_x: f32, delta_y: f32);
    fn render(&self, gl: &Gl, renderer: &mut DeferredPipeline);
    fn update_view(&mut self);
}
