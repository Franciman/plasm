use crate::expression::Expression;

// This struct contains info
// about which part of the plot to draw and about the zoom
pub struct DisplayInfo {
    pub x_start: f32,
    pub x_end: f32,
    // number of steps
    pub resolution: u32,
}

pub struct Segment {
    pub start_point: (f32, f32),
    pub end_point: (f32, f32),
}

// Given the DisplayInfo, it returns an approximation of the plot
// consistings as a list of segments that should be displayed
pub fn generate_2dplot(expression: &Expression<f64>, display_info: DisplayInfo) -> Vec<Segment> {
    let mut segments = Vec::new();

    let step = (display_info.x_end - display_info.x_start) / (display_info.resolution as f32);

    let mut x_0 = display_info.x_start;
    while x_0 < display_info.x_end {
        let y_0 = expression.eval_2d(x_0 as f64);

        let x_1 = x_0 + step;
        let y_1 = expression.eval_2d(x_1 as f64);

        segments.push(Segment {
            start_point: (x_0, y_0 as f32),
            end_point: (x_1, y_1 as f32),
        });

        x_0 += step;
    }

    return segments
}
