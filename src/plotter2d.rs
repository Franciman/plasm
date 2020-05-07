use three_d::*;
use crate::expression::Expression;
use three_d::Program;
use crate::plotter;
use crate::plotter::Plotter;
use crate::plot_generator2d;
use honestintervals::IntervalSet;

pub struct Plotter2d {
    plot: Plot,
    program: Program,
    expression: Expression<IntervalSet<f64>>,
    camera: Camera,
    screen_size: (usize, usize)
}

impl Plotter2d {
    pub fn new(gl: &Gl, expression: Expression<IntervalSet<f64>>, screen_size: (usize, usize)) -> Plotter2d {

        let program = Program::from_source(gl,
            include_str!("../assets/shaders/color.vert"),
            include_str!("../assets/shaders/color.frag")).unwrap();
                
        let camera = Camera {position: (0.0, 0.0), size: 10.0};
        let plot = Plot::new(gl, &expression, screen_size.0 as u32, &camera);

        Plotter2d {
            plot,
            program,
            expression,
            camera,
            screen_size
        }
    }

    pub fn set_expression(&mut self, expression: Expression<IntervalSet<f64>>) {
        self.expression = expression;
        self.update_view();
    }
}

impl plotter::Plotter for Plotter2d {

    fn update_view(&mut self) {
        self.plot.update_positions(&self.expression, self.screen_size.0 as u32, &self.camera)
    }

    fn zoom(&mut self, delta: f32) {
        self.camera.size *= (1.05 as f32).powf(delta);
        self.update_view();
    }

    fn translate(&mut self, delta_x: f32, delta_y: f32) {
        self.camera.position.0 += delta_x * self.camera.size / self.screen_size.0 as f32;
        self.camera.position.1 += delta_y * self.camera.size / self.screen_size.1 as f32;
        self.update_view();
    }

    fn render(&self, gl: &Gl, renderer: &mut DeferredPipeline) {
        Screen::write(gl, 0, 0, self.screen_size.0, self.screen_size.1, Some(&vec4(0.9, 0.9, 0.9, 1.0)), None, &|| {
            self.plot.draw(&self.program);
        }).unwrap();
    }
}

struct Camera {
    position: (f32, f32),
    size: f32
}

impl Camera { 
    // project a point to normalized coordinates [-1,1]
    fn to_normalized_coordinates(&self, point: (f32, f32)) -> (f32, f32) {
        let x_proj = 2.0*(point.0 - self.position.0)/self.size;
        let y_proj = 2.0*(point.1 - self.position.1)/self.size;
        (x_proj, y_proj)
    }
}

struct Plot {
    position_buffer: VertexBuffer,
    position_buffer_size: u32,
    axis_buffer: VertexBuffer,
}

impl Plot {

    fn new(gl: &Gl, expression: &Expression<IntervalSet<f64>>, resolution: u32, camera: &Camera) -> Plot {
        let positions = Plot::generate_positions(expression, resolution, camera);
        let axis_points = Plot::generate_axis_lines(camera);

        let position_buffer = VertexBuffer::new_with_static_f32(&gl, &positions).unwrap();
        let position_buffer_size = (positions.len() / 3) as u32;
        let axis_buffer = VertexBuffer::new_with_static_f32(&gl, &axis_points).unwrap();

        Plot {
            position_buffer,
            position_buffer_size,
            axis_buffer
        }
    }

    fn update_positions(&mut self, expression: &Expression<IntervalSet<f64>>, resolution: u32, camera: &Camera) {
        let positions = Plot::generate_positions(expression, resolution, camera);
        let axis_positions = Plot::generate_axis_lines(camera);
        
        self.position_buffer.fill_with_static_f32(&positions);
        self.position_buffer_size = (positions.len() / 3) as u32;
        self.axis_buffer.fill_with_static_f32(&axis_positions);
    }

    fn draw(&self, program: &Program) {
        program.add_uniform_mat4("worldViewProjectionMatrix", &Mat4::identity()).unwrap();

        program.use_attribute_vec3_float(&self.position_buffer, "position").unwrap();
        program.add_uniform_vec4("color", &vec4(0.5, 0.3, 0.1, 1.0)).unwrap();
        program.draw_arrays(self.position_buffer_size);

        // draw axis
        program.use_attribute_vec3_float(&self.axis_buffer, "position").unwrap();
        program.add_uniform_vec4("color", &vec4(0.2, 0.2, 0.2, 1.0)).unwrap();
        program.draw_arrays_mode(4, consts::LINES);
    }

    fn generate_positions(expression: &Expression<IntervalSet<f64>>, resolution: u32, camera: &Camera) -> Vec<f32> {

        let display_info = plot_generator2d::DisplayInfo {
            x_start: (camera.position.0 - camera.size/2.0) as f64,
            x_end: (camera.position.0 + camera.size/2.0) as f64,
            y_start: (camera.position.1 - camera.size/2.0) as f64,
            y_end: (camera.position.1 + camera.size/2.0) as f64,
            resolution: resolution
        };
        let rectangles = plot_generator2d::generate_2dplot(expression, display_info);

        let mut positions: Vec<f32> = Vec::with_capacity(rectangles.len()*2*3*3);
        
        let mut add_position = |p: (f32, f32), x_offset: f32, y_offset: f32| {
            let (x_screen, y_screen) = camera.to_normalized_coordinates((p.0, p.1));
            positions.push(x_screen + x_offset);
            positions.push(y_screen + y_offset);
            positions.push(0.0);
        };

        let x_width = 0.002;
        let y_width = 0.005;

        for rectangle in rectangles {
            let mut rectangle = rectangle;
            if rectangle.y1 > rectangle.y2 {
                let temp = rectangle.y1;
                rectangle.y1 = rectangle.y2;
                rectangle.y2 = temp;
            }
            add_position((rectangle.x1, rectangle.y1), -x_width, -y_width);
            add_position((rectangle.x2, rectangle.y2), x_width, y_width);
            add_position((rectangle.x1, rectangle.y2), -x_width, y_width);

            add_position((rectangle.x1, rectangle.y1), -x_width, -y_width);
            add_position((rectangle.x2, rectangle.y1), x_width, -y_width);
            add_position((rectangle.x2, rectangle.y2), x_width, y_width);
        }

        positions
    }

    fn generate_axis_lines(camera: &Camera) -> Vec<f32> {
        let (x_zero, y_zero) = camera.to_normalized_coordinates((0.0, 0.0));

        vec![-1.0, y_zero, 0.0,
            1.0, y_zero, 0.0,
            x_zero, -1.0, 0.0,
            x_zero, 1.0, 0.0]
    }

}
