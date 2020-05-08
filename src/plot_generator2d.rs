use crate::expression::Expression;
use honestintervals::IntervalSet;

// This struct contains info
// about which part of the plot to draw and about the zoom
#[derive(Clone)]
pub struct DisplayInfo {
    pub x_start: f64,
    pub x_end: f64,
    pub y_start: f64,
    pub y_end: f64,
    // number of steps
    pub resolution: f64,
}

// Struct representing a rectangle containing the coordinates of 2 opposite corners
#[derive(Debug)]
pub struct Rectangle {
    pub x1: f32,
    pub y1: f32,
    pub x2: f32,
    pub y2: f32,
}

pub fn generate_2dplot_implicit(expression: &Expression<IntervalSet<f64>>, display_info: DisplayInfo) -> Vec<Rectangle> {

    let x_interval = IntervalSet::new(display_info.x_start, display_info.x_end);
    let y_interval = IntervalSet::new(display_info.y_start, display_info.y_end);
    let eval = expression.eval_3d(x_interval, y_interval);
    
    if eval.has_zero() {
        if display_info.y_end - display_info.y_start < display_info.resolution ||
        display_info.x_end - display_info.x_start < display_info.resolution {
            let rect = Rectangle {
                x1: display_info.x_start as f32,
                y1: display_info.y_start as f32,
                x2: display_info.x_end as f32,
                y2: display_info.y_end as f32
            };
            return vec![rect];
        } else {
            let x_half = (display_info.x_start + display_info.x_end) / 2.0;
            let y_half = (display_info.y_start + display_info.y_end) / 2.0;
            let mut plot: Vec<Rectangle> = Vec::new();

            let mut quadrant = display_info.clone();
            quadrant.x_start = x_half;
            quadrant.y_start = y_half;
            plot.append(&mut generate_2dplot_implicit(expression, quadrant));

            let mut quadrant = display_info.clone();
            quadrant.x_end = x_half;
            quadrant.y_start = y_half;
            plot.append(&mut generate_2dplot_implicit(expression, quadrant));

            let mut quadrant = display_info.clone();
            quadrant.x_end = x_half;
            quadrant.y_end = y_half;
            plot.append(&mut generate_2dplot_implicit(expression, quadrant));

            let mut quadrant = display_info.clone();
            quadrant.x_start = x_half;
            quadrant.y_end = y_half;
            plot.append(&mut generate_2dplot_implicit(expression, quadrant));
            
            return plot;
        }
    }
    vec![]
}

// Given the DisplayInfo, it returns an approximation of the plot
// consistings as a list of rectangles that should be displayed
pub fn generate_2dplot(expression: &Expression<IntervalSet<f64>>, display_info: DisplayInfo) -> Vec<Rectangle> {
    let mut rectangles = Vec::new();

    let mut x_0 = display_info.x_start;
    while x_0 < display_info.x_end {

        let x_1 = x_0 + display_info.resolution;
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

        x_0 += display_info.resolution;
    }

    return rectangles
}
