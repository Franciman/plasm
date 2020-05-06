use crate::expression::Expression;
use honestintervals::IntervalSet;

// This struct contains info
// about which part of the plot to draw and about the zoom
pub struct DisplayInfo {
    pub x_start: f64,
    pub x_end: f64,
    pub y_start: f64,
    pub y_end: f64,
    // number of steps
    pub resolution: u32,
}

// Struct representing a rectangle containing the coordinates of 2 opposite corners
pub struct Rectangle {
    pub x1: f32,
    pub y1: f32,
    pub x2: f32,
    pub y2: f32,
}

// Given the DisplayInfo, it returns an approximation of the plot
// consistings as a list of rectangles that should be displayed
pub fn generate_2dplot(expression: &Expression<IntervalSet<f64>>, display_info: DisplayInfo) -> Vec<Rectangle> {
    let mut rectangles = Vec::new();

    let step = (display_info.x_end - display_info.x_start) / (display_info.resolution as f64);

    let mut x_0 = display_info.x_start;
    while x_0 < display_info.x_end {

        let x_1 = x_0 + step;
        let x_interval = IntervalSet::new(x_0, x_1);
        let y_intervals: Vec<(f64, f64)> = expression.eval_2d(x_interval).into();

        for interval in y_intervals {
            if (interval.0 > display_info.y_end && interval.1 > display_info.y_end) 
                || (interval.0 < display_info.y_start && interval.1 < display_info.y_start) {
                    continue;
                }

            rectangles.push(Rectangle {
                x1: x_0 as f32,
                y1: interval.0.max(display_info.y_start).min(display_info.y_end) as f32,
                x2: x_1 as f32,
                y2: interval.1.max(display_info.y_start).min(display_info.y_end) as f32
            });
        }

        x_0 += step;
    }

    return rectangles
}
