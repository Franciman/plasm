use crate::expression::Expression;
use honestintervals::IntervalSet;

pub struct Rectangle {
    pub x_start: f64,
    pub y_start: f64,
    pub x_end: f64,
    pub y_end: f64,
}

struct Implicit2DPlot<'a> {
    expression: &'a Expression<IntervalSet<f64>>,
    plot: Vec<Rectangle>,
    resolution: f64,
}

impl<'a> Implicit2DPlot<'a> {
    fn generate_plot(&mut self, rect: Rectangle) {
        let x_interval = IntervalSet::new(rect.x_start, rect.x_end);
        let y_interval = IntervalSet::new(rect.y_start, rect.y_end);
        let eval = self.expression.eval_3d(x_interval, y_interval);
        
        if eval.has_zero() {
            if rect.y_end - rect.y_start < self.resolution ||
            rect.x_end - rect.x_start < self.resolution {
                let rect = Rectangle {
                    x_start: rect.x_start,
                    y_start: rect.y_start,
                    x_end: rect.x_end,
                    y_end: rect.y_end
                };
                self.plot.push(rect);
            } else {
                let x_half = (rect.x_start + rect.x_end) / 2.0;
                let y_half = (rect.y_start + rect.y_end) / 2.0;
    
                let quadrant = Rectangle {
                    x_start: x_half,
                    y_start: y_half,
                    x_end: rect.x_end,
                    y_end: rect.y_end
                };
                self.generate_plot(quadrant);

                let quadrant = Rectangle {
                    x_start: rect.x_start,
                    y_start: y_half,
                    x_end: x_half,
                    y_end: rect.y_end
                };
                self.generate_plot(quadrant);

                let quadrant = Rectangle {
                    x_start: rect.x_start,
                    y_start: rect.y_start,
                    x_end: x_half,
                    y_end: y_half
                };
                self.generate_plot(quadrant);

                let quadrant = Rectangle {
                    x_start: x_half,
                    y_start: rect.y_start,
                    x_end: rect.x_end,
                    y_end: y_half
                };
                self.generate_plot(quadrant);
            }
        }
    }
}

pub fn generate_2dplot_implicit(expression: &Expression<IntervalSet<f64>>, display_info: Rectangle, resolution: f64) -> Vec<Rectangle> {
    let mut plot = Implicit2DPlot {
        expression: expression,
        plot: Vec::new(),
        resolution: resolution
    };
    plot.generate_plot(display_info);
    return plot.plot;
}

// Given the DisplayInfo, it returns an approximation of the plot
// consistings as a list of rectangles that should be displayed
pub fn generate_2dplot(expression: &Expression<IntervalSet<f64>>, display_info: Rectangle, resolution: f64) -> Vec<Rectangle> {
    let mut rectangles = Vec::new();

    let mut x_0 = display_info.x_start;
    while x_0 < display_info.x_end {

        let x_1 = x_0 + resolution;
        let x_interval = IntervalSet::new(x_0, x_1);
        let y_intervals: Vec<(f64, f64)> = expression.eval_2d(x_interval).into();

        for interval in y_intervals {
            if (interval.0 > display_info.y_end && interval.1 > display_info.y_end) 
                || (interval.0 < display_info.y_start && interval.1 < display_info.y_start) {
                    continue;
                }

            rectangles.push(Rectangle {
                x_start: x_0,
                y_start: interval.0.max(display_info.y_start).min(display_info.y_end),
                x_end: x_1,
                y_end: interval.1.max(display_info.y_start).min(display_info.y_end)
            });
        }

        x_0 += resolution;
    }

    return rectangles
}
