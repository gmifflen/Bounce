use ggez::{
    graphics::{self, Color, MeshBuilder, Rect},
    Context, ContextBuilder, GameResult,
};
use ggez::event::{self, EventHandler};
use ggez::timer;
use std::collections::VecDeque;
use core::f32::consts::PI;

struct MainState {
    position: (f32, f32),
    velocity: (f32, f32),
    trail: VecDeque<((f32, f32), Color)>,
    trail_length: usize,
    color_shift: f32,
}

impl MainState {
    fn new(_ctx: &mut Context) -> GameResult<MainState> {
        let state = MainState {
            position: (100.0, 100.0),
            velocity: (2.0, 2.0),
            trail: VecDeque::new(),
            trail_length: 9999999,
            color_shift: 0.0,
        };
        Ok(state)
    }

    fn update_square(&mut self, ctx: &mut Context) {
        let (width, height) = graphics::drawable_size(ctx);
        let square_size = 50.0;

        self.position.0 += self.velocity.0;
        self.position.1 += self.velocity.1;

        if self.position.0 < 0.0 || self.position.0 > width - square_size {
            self.velocity.0 *= -1.0;
        }
        if self.position.1 < 0.0 || self.position.1 > height - square_size {
            self.velocity.1 *= -1.0;
        }

        // Calculate current color
        let color = Color::new(
            self.color_shift.sin().powi(2),
            (self.color_shift + 2.0 * PI / 3.0).sin().powi(2),
            (self.color_shift + 4.0 * PI / 3.0).sin().powi(2),
            1.0,
        );

        // Add current position and color to the trail
        self.trail.push_front((self.position, color));
        while self.trail.len() > self.trail_length {
            self.trail.pop_back();
        }

        // Update the color_shift for the rainbow effect
        self.color_shift += 0.01;
        if self.color_shift > 2.0 * PI {
            self.color_shift = 0.0;
        }
    }
}

impl EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        const DESIRED_FPS: u32 = 60;

        while timer::check_update_time(ctx, DESIRED_FPS) {
            self.update_square(ctx);
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, Color::BLACK);

        let square_size = 50.0;
        // Iterate over the trail in reverse order
        for (_i, &((x, y), color)) in self.trail.iter().rev().enumerate() {
            let alpha = 1.0;
            let faded_color = Color::new(color.r, color.g, color.b, alpha);

            let mut mesh_builder = MeshBuilder::new();
            let _ = mesh_builder.rectangle(
                graphics::DrawMode::fill(),
                Rect::new(x, y, square_size, square_size),
                faded_color,
            );
            let square = mesh_builder.build(ctx)?;
            graphics::draw(ctx, &square, graphics::DrawParam::default())?;
        }

        graphics::present(ctx)?;
        Ok(())
    }
}

fn main() -> GameResult {
    let (mut ctx, event_loop) = ContextBuilder::new("Bounce", "author")
        .build()?;

    graphics::set_window_title(&ctx, "Bounce");

    let state = MainState::new(&mut ctx)?;
    event::run(ctx, event_loop, state)
}
