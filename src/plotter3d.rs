use three_d::*;
use crate::operator_descr::{OperatorTable, default_operator_table};
use crate::parser::Parser;
use crate::expression::Expression;
use three_d::Program;

const RESOLUTION: usize = 50;

pub struct Plotter3d {
    plot: Plot,
    program: Program,
    operator_table: OperatorTable,
    expression: Expression,
    camera: Camera,
    screen_size: (u32, u32),
    projection: three_d::Camera
}

struct Camera {
    position: (f32, f32, f32),
    size: f32,
}

struct Plot {
    position_buffer: VertexBuffer,
    position_buffer_size: u32,
    axis_buffer: VertexBuffer,
}

impl Plotter3d {
    pub fn new(gl: &Gl, input: &str, screen_size: (u32, u32)) -> Plotter3d {

        let program = Program::from_source(gl,
            include_str!("../assets/shaders/3d.vert"),
            include_str!("../assets/shaders/color.frag")).unwrap();

        let operator_table = default_operator_table();
        let expression = Parser::new(input, &operator_table).parse().unwrap();
        let camera = Camera {position: (0.0, 0.0, 0.0), size: 10.0};
        let plot = Plot::new(gl, &expression, RESOLUTION, &camera);
        let projection = three_d::Camera::new_perspective(gl, vec3(10.0, 10.0, 10.0), vec3(0.0, 0.0, 0.0), vec3(0.0, 0.0, 1.0),
                                                        degrees(45.0), screen_size.0 as f32/screen_size.1 as f32, 0.1, 100.0);

        Plotter3d {
            plot,
            program,
            operator_table,
            expression,
            camera,
            screen_size,
            projection
        }
    }

    pub fn set_expression(&mut self, gl: &Gl, input: &str) {
        let expr = Parser::new(input, &self.operator_table).parse();
        match expr {
            Ok(expr) => self.expression = expr,
            Err(_) => ()
        }
        self.update_view(gl);
    }

    pub fn zoom(&mut self, delta: f32) {
        self.projection.zoom(delta as f32);
    }

    pub fn rotate(&mut self, delta_x: f32, delta_y: f32) {
        self.projection.rotate(delta_x, delta_y);
    }

    pub fn translate(&mut self, gl: &Gl, delta_x: f32, delta_y: f32) {
        self.camera.position.0 += delta_x * self.camera.size / self.screen_size.0 as f32;
        self.camera.position.1 += delta_y * self.camera.size / self.screen_size.1 as f32;
        self.update_view(gl);
    }

    pub fn draw(&self) {
        self.plot.draw(&self.program, &self.projection);
    }

    fn update_view(&mut self, gl: &Gl) {
        self.plot = Plot::new(gl, &self.expression, RESOLUTION, &self.camera);
    }
}

impl Plot {
    fn new(gl: &Gl, expression: &Expression, count: usize, camera: &Camera) -> Plot {
        let points = Plot::generate_points(expression, count, camera);
        let axis_points = Plot::generate_axis_lines(camera);

        let position_buffer = VertexBuffer::new_with_static_f32(&gl, &points).unwrap();
        let position_buffer_size = ((count-1)*(count-1)*2*3) as u32;
        let axis_buffer = VertexBuffer::new_with_static_f32(&gl, &axis_points).unwrap();

        Plot{
            position_buffer,
            position_buffer_size,
            axis_buffer
        }
    }

    pub fn draw(&self, program: &Program, projection: &three_d::Camera) {
        let world_view_projection = projection.get_projection() * projection.get_view();
        program.add_uniform_mat4("worldViewProjectionMatrix", &world_view_projection).unwrap();

        program.use_attribute_vec3_float(&self.position_buffer, "position").unwrap();
        program.add_uniform_vec4("color", &vec4(0.3, 0.6, 0.3, 0.7)).unwrap();
        program.draw_arrays_mode(self.position_buffer_size, consts::TRIANGLES);

        program.use_attribute_vec3_float(&self.position_buffer, "position").unwrap();
        program.add_uniform_vec4("color", &vec4(0.2, 0.2, 0.2, 1.0)).unwrap();
        program.draw_arrays_mode(self.position_buffer_size, consts::LINES);

        // draw axis
        program.use_attribute_vec3_float(&self.axis_buffer, "position").unwrap();
        program.add_uniform_vec4("color", &vec4(0.5, 0.5, 0.5, 1.0)).unwrap();
        program.draw_arrays_mode(6, consts::LINES);
    }

    fn generate_points(expression: &Expression, count: usize, camera: &Camera) -> Vec<f32> {
    
        let mut grid = vec![vec![(0.0, 0.0, 0.0); count]; count];
        let step = camera.size / count as f32;
        let mut x = camera.position.0 - camera.size/2.0;
        for i in 0..count {
            let mut y = camera.position.1 - camera.size/2.0;
            for j in 0..count {
                let z = expression.eval((x, y));
                grid[i][j] = (x - camera.position.0, y - camera.position.1, z);
                y += step;
            }
            x += step;
        }
    
        // each square in the grid has 2 triangles with 3 vertices in 3D coordinates
        let len = (count-1)*(count-1)*2*3*3; 
        let mut points: Vec<f32> = Vec::with_capacity(len);
    
        for i in 0..count-1 {
            for j in 0..count-1 {
                let p1 = grid[i][j];
                let p2 = grid[i][j+1];
                let p3 = grid[i+1][j+1];
                let p4 = grid[i+1][j];
    
                // first triangle
                points.push(p1.0); points.push(p1.1); points.push(p1.2);
                points.push(p2.0); points.push(p2.1); points.push(p2.2);
                points.push(p3.0); points.push(p3.1); points.push(p3.2);
    
                // second triangle
                points.push(p1.0); points.push(p1.1); points.push(p1.2);
                points.push(p3.0); points.push(p3.1); points.push(p3.2);
                points.push(p4.0); points.push(p4.1); points.push(p4.2);
            }
        }
    
        points
    }
    
    fn generate_axis_lines(camera: &Camera) -> Vec<f32> {
        vec![-10.0, 0.0, 0.0,
            10.0, 0.0, 0.0,
            0.0, -10.0, 0.0,
            0.0, 10.0, 0.0,
            0.0, 0.0, -10.0,
            0.0, 0.0, 10.0]
    }
}