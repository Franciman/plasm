
use three_d::*;
mod plotter;
mod operator_descr;
mod parser;
mod expression;

fn main() {

    let mut window = Window::new_default("Plasm").unwrap();
    let (screen_width, screen_height) = window.framebuffer_size();

    let gl = window.gl();

    // Camera
    let mut camera = Camera::new_perspective(&gl, vec3(0.0, 0.0, 2.0), vec3(0.0, 0.0, 0.0), vec3(0.0, 1.0, 0.0),
                                                degrees(45.0), screen_width as f32/screen_height as f32, 0.1, 10.0);

    let plotter = plotter::Plotter::new(&gl, "sin(x)");

    // main loop
    let mut time = 0.0;
    window.render_loop(move |frame_input|
    {
        time += frame_input.elapsed_time as f32;
        camera.set_size(frame_input.screen_width as f32, frame_input.screen_height as f32);

        Screen::write(&gl, 0, 0, screen_width, screen_height, Some(&vec4(0.9, 0.9, 0.9, 1.0)), Some(1.0), &|| {

            plotter.plot(&camera);

        }).unwrap();
    }).unwrap();
}