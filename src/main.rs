mod plotter3d;
mod plotter2d;
mod plotter;
mod semantics;
mod operator_descr;
mod parser;
mod expression;
mod plot_generator2d;
mod operator_tables;
use three_d::*;
use plotter::Plotter;
use log::info;

const DEFAULT_EXPR: &str = "sin(x)";
const DEFAULT_MODE: DrawingMode = DrawingMode::Mode3d;

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

    let mut renderer = DeferredPipeline::new(&gl).unwrap();

    let operator_table = operator_tables::default_operator_table();
    let interval_arithmetic_operator_table = operator_tables::interval_arithmetic_operator_table();

    let expression = parser::parse(DEFAULT_EXPR, &interval_arithmetic_operator_table).unwrap();
    let mut plotter2d = plotter2d::Plotter2d::new(&gl, expression, (screen_width, screen_height));
    let expression = parser::parse(DEFAULT_EXPR, &operator_table).unwrap();
    let mut plotter3d = plotter3d::Plotter3d::new(&gl, expression, (screen_width, screen_height));
    

    // main loop
    let mut dragging = false;
    let mut old_input = String::from(DEFAULT_EXPR);
    let mut drawing_mode = DEFAULT_MODE;
    window.render_loop(move |frame_input|
    {
        // read input
        let input = get_input();
        if input != old_input {

            // determine if 2d function or 3d function
            let expression = parser::parse(&input, &operator_table);
            match expression {
                Ok(expr) => {
                    match expr.expr_type() {
                        expression::ExprType::Expr2d | expression::ExprType::ExprImplicit => {
                            // draw as 2d function parse again using interval arithmetic
                            let expression = parser::parse(&input, &interval_arithmetic_operator_table);
                            match expression {
                                Ok(expr) => {
                                    plotter2d.set_expression(expr);
                                    info!("Draw implicit function");
                                }
                                Err(_) => {
                                    info!("Could not parse input function");
                                }
                            }
                            drawing_mode = DrawingMode::Mode2d;
                            renderer.geometry_pass(screen_width, screen_height, &|| {
                            }).unwrap();
                        },
                        expression::ExprType::Expr3d => {
                            plotter3d.set_expression(expr);
                            drawing_mode = DrawingMode::Mode3d;
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
                            },
                            DrawingMode::Mode3d => {
                                plotter3d.translate(delta_x, delta_y);
                            }
                        }
                    }
                },
                Event::MouseWheel {delta} => {
                    match &drawing_mode {
                        DrawingMode::Mode2d => {
                            plotter2d.zoom(*delta as f32);
                        },
                        DrawingMode::Mode3d => {
                            plotter3d.zoom(*delta as f32);
                        }
                    }
                },
                _ => ()
            }
        }

        // draw
        match &drawing_mode {
            DrawingMode::Mode2d => {
                plotter2d.render(&gl, &mut renderer);
            },
            DrawingMode::Mode3d => {

                plotter3d.render(&gl, &mut renderer);

                let delta_rotation = frame_input.elapsed_time as f32 / 200.0;
                plotter3d.rotate(delta_rotation);
            }
        }


    }).unwrap();
}
