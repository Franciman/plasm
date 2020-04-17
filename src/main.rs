
use three_d::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let screenshot_path = if args.len() > 1 { Some(args[1].clone()) } else {None};

    let mut window = Window::new_default("Hello, world!").unwrap();
    let (screen_width, screen_height) = window.framebuffer_size();

    let gl = window.gl();

    // Camera
    let mut camera = Camera::new_perspective(&gl, vec3(0.0, 0.0, 2.0), vec3(0.0, 0.0, 0.0), vec3(0.0, 1.0, 0.0),
                                                degrees(45.0), screen_width as f32/screen_height as f32, 0.1, 10.0);

    let positions: Vec<f32> = vec![
        0.5, -0.5, 0.0, // bottom right
        -0.5, -0.5, 0.0,// bottom left
        0.0,  0.5, 0.0 // top
    ];
    let position_buffer = VertexBuffer::new_with_static_f32(&gl, &positions).unwrap();
    let colors: Vec<f32> = vec![
        1.0, 0.0, 0.0,   // bottom right
        0.0, 1.0, 0.0,   // bottom left
        0.0, 0.0, 1.0    // top
    ];
    let color_buffer = VertexBuffer::new_with_static_f32(&gl, &colors).unwrap();

    let program = Program::from_source(&gl,
                                       include_str!("../assets/shaders/color.vert"),
                                       include_str!("../assets/shaders/color.frag")).unwrap();

    // main loop
    let mut time = 0.0;
    window.render_loop(move |frame_input|
    {
        time += frame_input.elapsed_time as f32;
        camera.set_size(frame_input.screen_width as f32, frame_input.screen_height as f32);

        Screen::write(&gl, 0, 0, screen_width, screen_height, Some(&vec4(0.8, 0.8, 0.8, 1.0)), Some(1.0), &|| {
            program.use_attribute_vec3_float(&position_buffer, "position").unwrap();
            program.use_attribute_vec3_float(&color_buffer, "color").unwrap();

            let world_view_projection = camera.get_projection() * camera.get_view() * Mat4::from_angle_y(radians(time * 0.005));
            program.add_uniform_mat4("worldViewProjectionMatrix", &world_view_projection).unwrap();

            program.draw_arrays(3);
        }).unwrap();

        if let Some(ref path) = screenshot_path {
            #[cfg(target_arch = "x86_64")]
            Screen::save_color(path, &gl, 0, 0, screen_width, screen_height).unwrap();
            std::process::exit(1);
        }
    }).unwrap();
}