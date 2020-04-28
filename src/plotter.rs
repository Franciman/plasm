use three_d::*;
use crate::operator_descr::{OperatorTable, default_operator_table};
use crate::parser::Parser;
use crate::expression::Expression;
use three_d::Program;

pub struct Plotter {
    position_buffer: VertexBuffer,
    axis_buffer: VertexBuffer,
    program: Program,
    operator_table: OperatorTable,
    expression: Expression,
    camera: Camera,
    screen_size: (u32, u32)
}

struct Camera {
    position: (f32, f32),
    size: f32
}

impl Plotter {
    pub fn new(gl: &Gl, input: &str, screen_size: (u32, u32)) -> Plotter{

        let program = Program::from_source(gl,
            include_str!("../assets/shaders/2d.vert"),
            include_str!("../assets/shaders/color.frag")).unwrap();

        let operator_table = default_operator_table();
        let expression = Parser::new(input, &operator_table).parse().unwrap();

        let camera = Camera {position: (0.0, 0.0), size: 10.0};
        let points = generate_points(&expression, screen_size.0, &camera);
        let axis_points = generate_axis_lines(&camera);

        let position_buffer = VertexBuffer::new_with_static_f32(&gl, &points).unwrap();
        let axis_buffer = VertexBuffer::new_with_static_f32(&gl, &axis_points).unwrap();

        Plotter{
            position_buffer,
            axis_buffer,
            program,
            operator_table,
            expression,
            camera,
            screen_size
        }
    }

    pub fn set_expression(&mut self, gl: &Gl, input: &str) {
        let expr = Parser::new(input, &self.operator_table).parse();
        match expr {
            Ok(expr) => self.expression = expr,
            Err(_) => ()
        }
        self.update_view(gl)
    }

    pub fn zoom(&mut self, gl: &Gl, delta: f32) {
        self.camera.size *= (1.1 as f32).powf(delta);
        self.update_view(gl);
    }

    pub fn translate(&mut self, gl: &Gl, delta_x: f32, delta_y: f32) {
        self.camera.position.0 += delta_x * self.camera.size / self.screen_size.0 as f32;
        self.camera.position.1 += delta_y * self.camera.size / self.screen_size.1 as f32;
        self.update_view(gl);
    }

    pub fn plot(&self) {
        self.program.use_attribute_vec3_float(&self.position_buffer, "position").unwrap();
        self.program.add_uniform_vec3("color", &vec3(0.3, 0.3, 0.3)).unwrap();
        self.program.draw_arrays_mode(self.screen_size.0, consts::LINE_STRIP);

        // draw axis
        self.program.use_attribute_vec3_float(&self.axis_buffer, "position").unwrap();
        self.program.add_uniform_vec3("color", &vec3(0.5, 0.5, 0.5)).unwrap();
        self.program.draw_arrays_mode(4, consts::LINES);
    }

    fn update_view(&mut self, gl: &Gl) {
        let points = generate_points(&self.expression, self.screen_size.0, &self.camera);
        let axis_points = generate_axis_lines(&self.camera);

        self.position_buffer = VertexBuffer::new_with_static_f32(&gl, &points).unwrap();
        self.axis_buffer = VertexBuffer::new_with_static_f32(&gl, &axis_points).unwrap();
    }
}

impl Camera { 
    // project a point to screen coordinate [-1,1]
    fn to_screen_coordinate(&self, point: (f32, f32)) -> (f32, f32) {
        let x_proj = 2.0*(point.0 - self.position.0)/self.size;
        let y_proj = 2.0*(point.1 - self.position.1)/self.size;
        (x_proj, y_proj)
    }
}


fn generate_points(expression: &Expression, count: u32, camera: &Camera) -> Vec<f32> {
    
    let mut points: Vec<f32> = Vec::with_capacity((count*3) as usize);

    let x_start = camera.position.0 - camera.size/2.0;

    for i in 0..count {

        // divide camera width into count segments and evaluate y
        let i = i as f32;
        let count = count as f32;
        let x = x_start + i * camera.size/count;
        let y = expression.eval((x, 0.0)); // y=0 as there is no y value
        let (x_screen, y_screen) = camera.to_screen_coordinate((x, y));

        points.push(x_screen);
        points.push(y_screen);
        points.push(0.0);
    }

    points
}

fn generate_axis_lines(camera: &Camera) -> Vec<f32> {
    let (x_zero, y_zero) = camera.to_screen_coordinate((0.0, 0.0));

    vec![-1.0, y_zero, 0.0,
        1.0, y_zero, 0.0,
        x_zero, -1.0, 0.0,
        x_zero, 1.0, 0.0]
}