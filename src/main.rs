use num::Complex;

type Vec2 = Vec<Vec<usize>>;

struct Coords {
    min: f64,
    max: f64,
}

fn render(vals: Vec2) {
    for row in vals {
        let mut line = String::with_capacity(row.len());

        for col in row {
            let val = match col {
                0..=2 => ' ',
                3..=5 => '.',
                6..=10 => '●',
                11..=30 => '*',
                31..=100 => '+',
                101..=200 => 'x',
                201..=400 => '$',
                401..=700 => '#',
                _ => '%',
            };

            line.push(val);
        }

        println!("{}", line);
    }
}

fn mandelbrot_at_point(cx: f64, cy: f64, max_iters: usize) -> usize {
    let mut z = Complex::new(0., 0.);
    let c = Complex::new(cx, cy);

    for i in 0..max_iters {
        if z.norm() > 2.0 {
            return i;
        }

        z = z * z + c;
    }

    max_iters
}

fn calc_mandelbrot(max_iters: usize, x: Coords, y: Coords, width: usize, height: usize) -> Vec2 {
    let mut rows: Vec2 = Vec::with_capacity(width);

    for point_y in 0..height {
        let mut row: Vec<usize> = Vec::with_capacity(height);

        for point_x in 0..width {
            let x_percent = point_x as f64 / width as f64;
            let y_percent = point_y as f64 / height as f64;
            let cx = x.min + (x.max - x.min) * x_percent;
            let cy = y.min + (y.max - y.min) * y_percent;
            let escaped_at = mandelbrot_at_point(cx, cy, max_iters);

            row.push(escaped_at);
        }

        rows.push(row);
    }

    rows
}

fn main() {
    let mandelbrot = calc_mandelbrot(
        1000,
        Coords { min: -2., max: 1. },
        Coords { min: -1., max: 1. },
        100,
        24,
    );

    render(mandelbrot);
}
