use raylib::Color;
use raylib::Vector2;

struct Point {
    x: i32,
    y: i32,
}

impl Point {
    // Constructor static method
    fn new(x: i32, y: i32) -> Point {
        return Point { x, y };
    }
}

const SCREEN_WIDTH: i32 = 640;
const SCREEN_HEIGHT: i32 = 480;

fn main() {
    let mut ballPosition = Vector2::new((SCREEN_WIDTH / 2) as f32, (SCREEN_HEIGHT / 2) as f32);   
    let rl = raylib::init()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Hello, World")
        .build();

    rl.set_target_fps(60);
    while !rl.window_should_close() {
        //UPDATE
        if rl.is_key_down(raylib::consts::KEY_LEFT as i32) {
            ballPosition.x -= 3.0;
        }
        if rl.is_key_down(raylib::consts::KEY_RIGHT as i32) {
            ballPosition.x += 3.0;
        }
        if rl.is_key_down(raylib::consts::KEY_UP as i32) {
            ballPosition.y -= 3.0;
        }
        if rl.is_key_down(raylib::consts::KEY_DOWN as i32) {
            ballPosition.y += 3.0;
        }

        /* Begin Drawing */
        rl.begin_drawing();

        rl.draw_fps(SCREEN_WIDTH - 85, 2);

        rl.clear_background(Color::BLACK);
        let textSize = rl.measure_text("move the ball with arrow keys", 20);
        rl.draw_text("move the ball with arrow keys", 10, 10, 20, Color::DARKGRAY);

        rl.draw_circle_v(ballPosition, 50.0, Color::MAROON);

        rl.draw_text("move the ball with arrow keys", 10, 10, 20, Color::DARKGRAY);

            rl.draw_circle_v(ballPosition, 50.0, Color::MAROON);
        rl.draw_circle_v(ballPosition, 100.0, Color::MAROON);
        rl.end_drawing();
    }
}
