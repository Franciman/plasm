mod plotter3d;
mod plotter2d;
mod plotter;
mod operator_descr;
mod parser;
mod expression;
use three_d::*;
use plotter::Plotter;
use log::info;

const DEFAULT_EXPR: &str = "sin(x)+sin(y)";

enum DrawingMode {
    Mode2d,
    Mode3d,
}

fn main() {
    let get_input = || {String::from(DEFAULT_EXPR)};
    start_main(get_input);
}

fn start_main<F: 'static>(get_input: F) where
    F: Fn() -> String {

    let mut window = Window::new_default("Plasm").unwrap();
    let (screen_width, screen_height) = window.framebuffer_size();
    let gl = window.gl();

    let operator_table = operator_descr::default_operator_table();
    let expression = parser::parse(DEFAULT_EXPR, &operator_table).unwrap();
    let mut plotter2d = plotter2d::Plotter2d::new(&gl, expression, (screen_width, screen_height));
    let expression = parser::parse(DEFAULT_EXPR, &operator_table).unwrap();
    let mut plotter3d = plotter3d::Plotter3d::new(&gl, expression, (screen_width, screen_height));

    // main loop
    let mut dragging = false;
    let mut old_input = String::from(DEFAULT_EXPR);
    let mut drawing_mode = DrawingMode::Mode3d;
    window.render_loop(move |frame_input|
    {
        // read input
        let input = get_input();
        if input != old_input {
            let expression = parser::parse(&input, &operator_table);
            match expression {
                Ok(expr) => {
                    drawing_mode = if expr.is_3d() {DrawingMode::Mode3d} else {DrawingMode::Mode2d};
                    match &drawing_mode {
                        DrawingMode::Mode2d => {
                            plotter2d.set_expression(expr);
                            plotter2d.update_view(&gl);
                            info!("Draw 2d function");
                        },
                        DrawingMode::Mode3d => {
                            plotter3d.set_expression(expr);
                            plotter3d.update_view(&gl);
                            info!("Draw 3d function");
                        }
                    }
                }
                Err(_) => {
                    info!("Could not parse input function");
                }
            }
            old_input = input;
        }

        // mouse events handling
        for event in frame_input.events.iter() {
            match event {
                Event::MouseClick {state, button, ..} => {
                    dragging = *button == MouseButton::Left && *state == State::Pressed;
                },
                Event::MouseMotion {delta} => {
                    if dragging {
                        let delta_x = -delta.0 as f32;
                        let delta_y = delta.1 as f32;

                        match &drawing_mode {
                            DrawingMode::Mode2d => {
                                plotter2d.translate(delta_x, delta_y);
                                plotter2d.update_view(&gl);
                            },
                            DrawingMode::Mode3d => {
                                plotter3d.translate(delta_x, delta_y);
                                plotter3d.update_view(&gl);
                            }
                        }
                    }
                },
                Event::MouseWheel {delta} => {
                    match &drawing_mode {
                        DrawingMode::Mode2d => {
                            plotter2d.zoom(*delta as f32);
                            plotter2d.update_view(&gl);
                        },
                        DrawingMode::Mode3d => {
                            plotter3d.zoom(*delta as f32);
                            plotter3d.update_view(&gl);
                        }
                    }
                },
                _ => ()
            }
        }

        // draw
        match &drawing_mode {
            DrawingMode::Mode2d => {
                plotter2d.draw(&gl);
            },
            DrawingMode::Mode3d => {
                plotter3d.draw(&gl);

                // rotate
                let delta_rotation = frame_input.elapsed_time as f32 / 200.0;
                plotter3d.rotate(delta_rotation);
            }
        }


    }).unwrap();
}
