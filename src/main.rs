
use three_d::*;
mod plotter;
mod operator_descr;
mod parser;
mod expression;

fn main() {

    let mut window = Window::new_default("Plasm").unwrap();
    let (screen_width, screen_height) = window.framebuffer_size();

    let gl = window.gl();

    let mut camera = Camera::new_perspective(&gl, vec3(0.0, 0.0, 2.0), vec3(0.0, 0.0, 0.0), vec3(0.0, 1.0, 0.0),
                                                degrees(45.0), screen_width as f32/screen_height as f32, 0.1, 10.0);

    let mut plotter = plotter::Plotter::new(&gl, "sin(x)");

    // main loop
    let mut moving = false;
    window.render_loop(move |frame_input|
    {
        camera.set_size(frame_input.screen_width as f32, frame_input.screen_height as f32);

        for event in frame_input.events.iter() {
            match event {
                Event::MouseClick {state, button, ..} => {
                    moving = *button == MouseButton::Left && *state == State::Pressed;
                },
                Event::MouseMotion {delta} => {
                    if moving {
                        let delta_x = -delta.0 as f32 / 200.0;
                        let delta_y = delta.1 as f32 / 200.0;
                        camera.translate(&vec3(delta_x, delta_y, 0.0));
                    }
                },
                Event::MouseWheel {delta} => {
                    plotter.zoom(&gl, *delta as f32);
                },
                _ => ()
            }
        }

        Screen::write(&gl, 0, 0, screen_width, screen_height, Some(&vec4(0.9, 0.9, 0.9, 1.0)), Some(1.0), &|| {

            plotter.plot(&camera);

        }).unwrap();
    }).unwrap();
}