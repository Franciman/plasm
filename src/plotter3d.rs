use three_d::*;
use crate::expression::Expression;
use crate::plotter::Plotter;

const RESOLUTION: usize = 50;

pub struct Plotter3d {
    plot: Plot,
    expression: Expression,
    camera: Camera,
    screen_size: (usize, usize),
    projection: three_d::Camera,
    ambient_light: AmbientLight,
    directional_light: DirectionalLight,
    axis: Axis
}

impl Plotter3d {
    pub fn new(gl: &Gl, expression: Expression, screen_size: (usize, usize)) -> Plotter3d {

        let camera = Camera {position: (0.0, 0.0, 0.0), size: 10.0};
        let plot = Plot::new(gl, &expression, RESOLUTION, &camera);
        let projection = three_d::Camera::new_perspective(gl, vec3(1.5, 1.5, 1.5), vec3(0.0, 0.0, 0.0), vec3(0.0, 1.0, 0.0),
                                                        degrees(45.0), screen_size.0 as f32/screen_size.1 as f32, 0.1, 10.0);

        let ambient_light = AmbientLight::new(&gl, 0.6, &vec3(1.0, 1.0, 1.0)).unwrap();
        let directional_light = DirectionalLight::new(&gl, 1.0, &vec3(0.5, 1.0, 1.0), &vec3(3.0, 0.0, 3.0)).unwrap();
        let axis = Axis::new(gl);

        let plotter = Plotter3d {
            plot,
            expression,
            camera,
            screen_size,
            projection,
            ambient_light,
            directional_light,
            axis
        };

        plotter
    }

    pub fn rotate(&mut self, delta: f32) {
        self.projection.rotate(delta, 0.0);
    }

}

impl Plotter for Plotter3d {

    fn update_view(&mut self) {
        self.plot.update_positions(&self.expression, RESOLUTION, &self.camera);
    }

    fn set_expression(&mut self, expression: Expression) {
        self.expression = expression;
        self.update_view();
    }

    fn zoom(&mut self, delta: f32) {
        self.camera.size *= (1.01 as f32).powf(delta);
        self.update_view();
    }

    fn translate(&mut self, delta_x: f32, delta_y: f32) {
        self.camera.position.0 += delta_x * self.camera.size / self.screen_size.0 as f32;
        self.camera.position.1 += delta_y * self.camera.size / self.screen_size.1 as f32;
        self.update_view();
    }

    fn render(&self, gl: &Gl, renderer: &mut DeferredPipeline) {

        renderer.geometry_pass(self.screen_size.0, self.screen_size.1, &|| {
            self.plot.render(&self.projection);
            self.axis.render(&self.projection);
        }).unwrap();

        Screen::write(&gl, 0, 0, self.screen_size.0, self.screen_size.1, Some(&vec4(0.9, 0.9, 0.9, 1.0)), None, &|| {
            renderer.light_pass(&self.projection, Some(&self.ambient_light), &[&self.directional_light], &[], &[]).unwrap();
        }).unwrap();

    }
}

struct Camera {
    position: (f32, f32, f32),
    size: f32,
}

impl Camera { 
    // project a point to normalized coordinates [-1,1]
    fn to_normalized_coordinates(&self, point: (f32, f32, f32)) -> (f32, f32, f32) {
        let x_proj = 2.0*(point.0 - self.position.0)/self.size;
        let y_proj = 2.0*(point.1 - self.position.1)/self.size;
        let z_proj = 2.0*(point.2 - self.position.2)/self.size;
        (x_proj, y_proj, z_proj)
    }
}

struct Plot {
    plot_mesh: Mesh,
    grid: Edges
}

impl Plot {

    fn new(gl: &Gl, expression: &Expression, count: usize, camera: &Camera) -> Plot {

        let positions = Plot::generate_grid_positions(expression, count, camera);
    
        // generate triangles: each square in the grid has 2 triangles
        let n_triangles = (count-1)*(count-1)*2;
        let n_vertices = n_triangles * 3; // 3 vertices per triangle
        let mut indices: Vec<u32> = Vec::with_capacity(n_vertices);

        let to_vec_index = |pos: (usize, usize)| {
            (count * pos.0 + pos.1) as u32
        };
    
        for i in 0..count-1 {
            for j in 0..count-1 {
                // first triangle
                indices.push(to_vec_index((i, j)));
                indices.push(to_vec_index((i, j+1)));
                indices.push(to_vec_index((i+1, j+1)));

                // second triangle
                indices.push(to_vec_index((i+1, j)));
                indices.push(to_vec_index((i, j)));
                indices.push(to_vec_index((i+1, j+1)));
            }
        }

        let cpu_mesh = CPUMesh::new_with_computed_normals(&indices, &positions).unwrap();
        let mut plot_mesh = cpu_mesh.to_mesh(gl).unwrap();
        plot_mesh.diffuse_intensity = 0.2;
        plot_mesh.specular_intensity = 0.4;
        plot_mesh.specular_power = 20.0;
        plot_mesh.color = vec3(0.6, 1.0, 0.6);


        // generate grid wireframe
        let n_lines = 2*count*(count-1);
        let n_vertices = n_lines * 3;
        let mut indices: Vec<u32> = Vec::with_capacity(n_vertices);
        let step = 1;
        for i in (0..count).step_by(step) {
            for j in (0..count-2).step_by(step) {
                indices.push(to_vec_index((i, j)));
                indices.push(to_vec_index((i, j+step)));
                indices.push(to_vec_index((i, j+step)));

                indices.push(to_vec_index((j, i)));
                indices.push(to_vec_index((j+step, i)));
                indices.push(to_vec_index((j+step, i)));
            }
        }

        let mut grid = Edges::new(gl, &indices, &positions, 0.001);
        grid.color = vec3(0.6, 0.6, 0.6);

        Plot {
            plot_mesh,
            grid
        }
    }

    fn update_positions(&mut self, expression: &Expression, count: usize, camera: &Camera) {
        let positions = Plot::generate_grid_positions(expression, count, camera);
        self.plot_mesh.update_positions(&positions).unwrap();
        self.grid.update_positions(&positions);
    }

    fn render(&self, projection: &three_d::Camera) {
        let transformation = Mat4::identity();
        self.plot_mesh.render(&transformation, projection);
        self.grid.render(&transformation, projection);
    }

    fn generate_grid_positions(expression: &Expression, count: usize, camera: &Camera) ->  Vec<f32> {
        // generate grid positions
        let mut positions: Vec<f32> = Vec::with_capacity(count * count * 3);
        let step = camera.size / count as f32;
        let mut x = camera.position.0 - camera.size/2.0;
        for _ in 0..count {
            let mut y = camera.position.1 - camera.size/2.0;
            for _ in 0..count {
                let z = expression.eval((x, y));
                let point = camera.to_normalized_coordinates((x,y,z));
                positions.push(point.0);
                positions.push(point.2);
                positions.push(-point.1);
                y += step;
            }
            x += step;
        }

        positions
    }
}

struct Axis {
    axis: Edges
}

impl Axis {
    fn new(gl: &Gl) -> Axis {
        let positions = vec![-1.0, 0.001, 0.0,
                            1.0, 0.0, 0.0,
                            0.0, -1.0, 0.0,
                            0.0, 1.0, 0.0,
                            0.0, 0.0, -1.0,
                            0.0, 0.0, 1.0];

        let indices = vec![2,3,3,4,5,5,0,1,1];
        let mut axis = Edges::new(gl, &indices, &positions, 0.007);
        axis.color = vec3(0.6, 0.6, 0.6);

        Axis {
            axis
        }
    }

    fn render(&self, camera: &three_d::Camera) {
        let transformation = Mat4::identity();
        self.axis.render(&transformation, camera);
    }
}