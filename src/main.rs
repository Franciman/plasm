mod plotter3d;
mod operator_descr;
mod parser;
mod expression;
use three_d::*;

fn main() {
    let get_input = || {String::from("x^2/5 + y^2/5")};
    start_main(get_input);
}

fn start_main<F: 'static>(get_input: F) where
    F: Fn() -> String {

    let mut window = Window::new_default("Plasm").unwrap();
    let (screen_width, screen_height) = window.framebuffer_size();

    let gl = window.gl();

    let input = get_input();
    let mut plotter = plotter3d::Plotter3d::new(&gl, input.as_str(), (screen_width as u32, screen_height as u32));

    // main loop
    let mut moving = false;
    let mut old_input = input;
    window.render_loop(move |frame_input|
    {
        let input = get_input();
        if input != old_input {
            plotter.set_expression(&gl, input.as_str());
            old_input = input;
        }

        for event in frame_input.events.iter() {
            match event {
                Event::MouseClick {state, button, ..} => {
                    moving = *button == MouseButton::Left && *state == State::Pressed;
                },
                Event::MouseMotion {delta} => {
                    if moving {
                        let delta_x = -delta.0 as f32;
                        let delta_y = delta.1 as f32;
                        plotter.translate(&gl, delta_x, delta_y);
                    }
                },
                Event::MouseWheel {delta} => {
                    plotter.zoom(&gl, *delta as f32);
                },
                _ => ()
            }
        }

        Screen::write(&gl, 0, 0, screen_width, screen_height, Some(&vec4(0.9, 0.9, 0.9, 1.0)), Some(1.0), &|| {

            plotter.draw();

        }).unwrap();
    }).unwrap();
}