use crate::expression::Expression;
use three_d::*;

pub trait Plotter {
    fn set_expression(&mut self, expression: Expression<f64>);
    fn zoom(&mut self, delta: f32);
    fn translate(&mut self, delta_x: f32, delta_y: f32);
    fn render(&self, gl: &Gl, renderer: &mut DeferredPipeline);
    fn update_view(&mut self);
}
