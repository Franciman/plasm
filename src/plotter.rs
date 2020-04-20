use three_d::*;
use crate::operator_descr::default_operator_table;
use crate::parser::Parser;
use three_d::Program;

pub struct Plotter {
    position_buffer: VertexBuffer,
    program: Program
}

impl Plotter {
    pub fn new(gl: &Gl, input: &str) -> Plotter{

        let table = default_operator_table();   
        let expr = Parser::new(input, &table).parse().unwrap();
        let mut points: Vec<f32> = Vec::new();

        for i in 0..2000 {
            let x: f32 = (i as f32 - 1000.0) / 100.0;
            let y = expr.eval(x);
            points.push(x/5.0);
            points.push(y/2.0);
            points.push(0.0);
        }

        let position_buffer = VertexBuffer::new_with_static_f32(&gl, &points).unwrap();

        let program = Program::from_source(gl,
            include_str!("../assets/shaders/color.vert"),
            include_str!("../assets/shaders/color.frag")).unwrap();

        Plotter{
            position_buffer,
            program
        }
    }

    pub fn plot(&self, camera: &Camera) {
        self.program.use_attribute_vec3_float(&self.position_buffer, "position").unwrap();
        self.program.add_uniform_vec3("color", &vec3(0.3, 0.3, 0.3)).unwrap();

        let world_view_projection = camera.get_projection() * camera.get_view();
        self.program.add_uniform_mat4("worldViewProjectionMatrix", &world_view_projection).unwrap();

        self.program.draw_arrays_line_strip(2000);
    }
}