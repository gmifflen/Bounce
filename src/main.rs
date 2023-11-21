// Import necessary modules from the ggez library for graphics and events.
use ggez::{
    graphics::{self, Color, MeshBuilder, Rect},
    Context, ContextBuilder, GameResult,
};
use ggez::event::{self, EventHandler};
use ggez::timer;
use std::collections::VecDeque;
use core::f32::consts::PI;

// Define the MainState struct to hold the game state.
struct MainState {
    position: (f32, f32),             // Position of the square.
    velocity: (f32, f32),             // Velocity of the square.
    trail: VecDeque<((f32, f32), Color)>, // Trail of positions and colors.
    trail_length: usize,              // Maximum length of the trail.
    color_shift: f32,                 // Value to create color shifting effect.
}

impl MainState {
    // Constructor for MainState.
    fn new(_ctx: &mut Context) -> GameResult<MainState> {
        let state = MainState {
            position: (100.0, 100.0),  // Initial position.
            velocity: (2.0, 2.0),      // Initial velocity.
            trail: VecDeque::new(),    // Initialize an empty trail.
            trail_length: 9999999,     // Set a large trail length.
            color_shift: 0.0,          // Start with no color shift.
        };
        Ok(state)
    }

    // Update the square's position, handle wall collision, and update the trail.
    fn update_square(&mut self, ctx: &mut Context) {
        let (width, height) = graphics::drawable_size(ctx);
        let square_size = 50.0;

        // Update position based on velocity.
        self.position.0 += self.velocity.0;
        self.position.1 += self.velocity.1;

        // Handle collision with the window edges.
        if self.position.0 < 0.0 || self.position.0 > width - square_size {
            self.velocity.0 *= -1.0;
        }
        if self.position.1 < 0.0 || self.position.1 > height - square_size {
            self.velocity.1 *= -1.0;
        }

        // Calculate current color for the rainbow effect.
        let color = Color::new(
            self.color_shift.sin().powi(2),
            (self.color_shift + 2.0 * PI / 3.0).sin().powi(2),
            (self.color_shift + 4.0 * PI / 3.0).sin().powi(2),
            1.0,
        );

        // Add current position and color to the trail.
        self.trail.push_front((self.position, color));
        while self.trail.len() > self.trail_length {
            self.trail.pop_back();
        }

        // Update the color_shift for the next frame.
        self.color_shift += 0.01;
        if self.color_shift > 2.0 * PI {
            self.color_shift = 0.0;
        }
    }
}

// Implement the EventHandler trait for MainState for handling game events.
impl EventHandler for MainState {
    // Update game logic.
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        const DESIRED_FPS: u32 = 60;

        // Update the square's state according to the desired FPS.
        while timer::check_update_time(ctx, DESIRED_FPS) {
            self.update_square(ctx);
        }
        Ok(())
    }

    // Render the game graphics.
    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, Color::BLACK); // Clear the screen.

        let square_size = 50.0;
        // Render the trail of the square.
        for (_i, &((x, y), color)) in self.trail.iter().rev().enumerate() {
            let alpha = 1.0;
            let faded_color = Color::new(color.r, color.g, color.b, alpha);

            // Create and draw each square in the trail.
            let mut mesh_builder = MeshBuilder::new();
            let _ = mesh_builder.rectangle(
                graphics::DrawMode::fill(),
                Rect::new(x, y, square_size, square_size),
                faded_color,
            );
            let square = mesh_builder.build(ctx)?;
            graphics::draw(ctx, &square, graphics::DrawParam::default())?;
        }

        // Present the rendered frame.
        graphics::present(ctx)?;
        Ok(())
    }
}

// Main function to initialize the game and run the event loop.
fn main() -> GameResult {
    // Create the game context and event loop.
    let (mut ctx, event_loop) = ContextBuilder::new("Bounce", "author")
        .build()?;

    // Set the window title.
    graphics::set_window_title(&ctx, "Bounce");

    // Create the initial game state.
    let state = MainState::new(&mut ctx)?;
    // Start the event loop.
    event::run(ctx, event_loop, state)
}
