use gfx_demo;

const TITLE: &'static str = "Plasma";
const WINDOW_WIDTH: usize = 640;
const WINDOW_HEIGHT: usize = 480;
const CANVAS_WIDTH: usize = WINDOW_WIDTH / 2;
const CANVAS_HEIGHT: usize = WINDOW_HEIGHT / 2;
const TICK_MS: u64 = 100;
const PREP_CONST: f64 = 0.02;

fn main() {
    let mut sin_vals = [0.0f64; CANVAS_WIDTH];
    let mut cos_vals = [0.0f64; CANVAS_WIDTH];

    for index in 0..CANVAS_WIDTH {
        sin_vals[index] = (index as f64 * PREP_CONST).sin();
        cos_vals[index] = (index as f64 * PREP_CONST).cos();
    }

    let mut t = 0;
    gfx_demo::gfx_demo(
        TITLE,
        WINDOW_WIDTH, WINDOW_HEIGHT,
        CANVAS_WIDTH, CANVAS_HEIGHT,
        TICK_MS,
        |pixels| {
            for y in 0 .. CANVAS_HEIGHT {
                for x in 0 .. CANVAS_WIDTH {
                    let a = sin_vals[x] + cos_vals[y];
                    let b = sin_vals[(y + t * 5) % CANVAS_WIDTH] + cos_vals[(x + t * 3) % CANVAS_WIDTH];
                    let i = ((a + b) / 2.0).sin();

                    pixels[(y * CANVAS_WIDTH) + x] = 0xff000000 | ((255.0 * i) as u32);
                }
            }

            t = (t + 1) % CANVAS_WIDTH;
        }
    ).unwrap();
}
