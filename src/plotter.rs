use three_d::*;
use crate::operator_descr::{OperatorTable, default_operator_table};
use crate::parser::Parser;
use crate::expression::Expression;
use three_d::Program;

const SAMPLES: u32 = 2000;

pub struct Plotter {
    position_buffer: VertexBuffer,
    axis_buffer: VertexBuffer,
    program: Program,
    operator_table: OperatorTable,
    expression: Expression,
    zoom: f32
}

impl Plotter {
    pub fn new(gl: &Gl, input: &str) -> Plotter{

        let program = Program::from_source(gl,
            include_str!("../assets/shaders/color.vert"),
            include_str!("../assets/shaders/color.frag")).unwrap();

        let operator_table = default_operator_table();
        let expression = Parser::new(input, &operator_table).parse().unwrap();

        let points = generate_points(&expression, SAMPLES, 100.0);
        let axis_points = generate_axis_lines();

        let position_buffer = VertexBuffer::new_with_static_f32(&gl, &points).unwrap();
        let axis_buffer = VertexBuffer::new_with_static_f32(&gl, &axis_points).unwrap();

        Plotter{
            position_buffer,
            axis_buffer,
            program,
            operator_table,
            expression,
            zoom: 100.0
        }
    }

    pub fn set_expression(&mut self, gl: &Gl, input: &str) {
        self.expression = Parser::new(input, &self.operator_table).parse().unwrap();
        self.update_view(gl)
    }

    pub fn zoom(&mut self, gl: &Gl, delta: f32) {
        self.zoom += delta;
        self.update_view(gl);
    }

    pub fn plot(&self, camera: &Camera) {
        let world_view_projection = camera.get_projection() * camera.get_view();

        self.program.use_attribute_vec3_float(&self.position_buffer, "position").unwrap();
        self.program.add_uniform_vec3("color", &vec3(0.3, 0.3, 0.3)).unwrap();
        self.program.add_uniform_mat4("worldViewProjectionMatrix", &world_view_projection).unwrap();
        self.program.draw_arrays_line_strip(SAMPLES);

        self.program.use_attribute_vec3_float(&self.axis_buffer, "position").unwrap();
        self.program.add_uniform_vec3("color", &vec3(0.5, 0.5, 0.5)).unwrap();
        self.program.add_uniform_mat4("worldViewProjectionMatrix", &world_view_projection).unwrap();
        self.program.draw_arrays_line_strip(4);
    }

    fn update_view(&mut self, gl: &Gl) {
        let points = generate_points(&self.expression, SAMPLES, self.zoom);
        let axis_points = generate_axis_lines();

        self.position_buffer = VertexBuffer::new_with_static_f32(&gl, &points).unwrap();
        self.axis_buffer = VertexBuffer::new_with_static_f32(&gl, &axis_points).unwrap();
    }
}


fn generate_points(expression: &Expression, count: u32, zoom: f32) -> Vec<f32> {
    
    let mut points: Vec<f32> = Vec::with_capacity((count*3) as usize);

    for i in 0..count {
        let x: f32 = (i as f32 - 1000.0) / 100.0;
        let y = expression.eval(x);

        let zoom = (if zoom > 0.0 {zoom} else {-1.0 / zoom}) / 1000.0;

        points.push(x * zoom);
        points.push(y * zoom);
        points.push(0.0);
    }

    points
}

fn generate_axis_lines() -> Vec<f32> {
    vec![-100.0, 0.0, 0.0,
        100.0, 0.0, 0.0,
        0.0, -100.0, 0.0,
        0.0, 100.0, 0.0]
}