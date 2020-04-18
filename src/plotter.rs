use three_d::*;

pub struct Plotter {
    position_buffer: VertexBuffer,
    program: Program
}

impl Plotter {
    pub fn new(gl: &Gl) -> Plotter{
        let positions: Vec<f32> = vec![
            0.5, -0.5, 0.0, // bottom right
            -0.5, -0.5, 0.0,// bottom left
            0.0,  0.5, 0.0 // top
        ];
        let position_buffer = VertexBuffer::new_with_static_f32(&gl, &positions).unwrap();

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

        self.program.draw_arrays(3);
    }
}