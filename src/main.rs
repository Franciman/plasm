mod plotter;
mod operator_descr;
mod parser;
mod expression;
use three_d::*;

fn main() {
    let get_input = || {String::from("sin(x*10)")};
    start_main(get_input);
}

fn start_main<F: 'static>(get_input: F) where
    F: Fn() -> String {

    let mut window = Window::new_default("Plasm").unwrap();
    let (screen_width, screen_height) = window.framebuffer_size();

    let gl = window.gl();

    let mut plotter = plotter::Plotter::new(&gl, "sin(x)");

    // main loop
    let mut moving = false;
    let mut old_input = String::from("sin(x)");
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
                        let delta_x = -delta.0 as f32 / 200.0;
                        let delta_y = delta.1 as f32 / 200.0;
                        // todo move plotter
                    }
                },
                Event::MouseWheel {delta} => {
                    plotter.zoom(&gl, *delta as f32);
                },
                _ => ()
            }
        }

        Screen::write(&gl, 0, 0, screen_width, screen_height, Some(&vec4(0.9, 0.9, 0.9, 1.0)), Some(1.0), &|| {

            plotter.plot();

        }).unwrap();
    }).unwrap();
}