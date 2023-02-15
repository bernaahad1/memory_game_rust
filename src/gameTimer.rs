use ggez::graphics;
use ggez::graphics::Color;
use ggez::mint::Point2;
use ggez::{Context, GameResult};
use std::time::Duration;
use std::time::Instant;

pub struct GameTimer {
    pub text: graphics::Text,
    pub start: Instant,
    pub duration: Duration,
    pub color: Color,
}

impl GameTimer {
    pub fn new(_ctx: &mut Context, start: Instant, duration: Duration) -> GameResult<GameTimer> {
        // let font = graphics::Font::default();
        let text = graphics::Text::new("02:00");

        Ok(GameTimer {
            text,
            start,
            duration,
            color: Color::WHITE,
        })
    }

    pub fn update(&mut self, ctx: &mut Context) -> GameResult {
        let elapsed = self.start.elapsed();
        if elapsed >= self.duration {
            // End the game if the timer has run out.
            ctx.request_quit();
        } else {
            let remaining = self.duration - elapsed;
            let minutes = remaining.as_secs() / 60;
            let seconds = remaining.as_secs() % 60;
            let timer_string = format!("{:02}:{:02}", minutes, seconds);
            self.text = graphics::Text::new(&timer_string);

            if remaining.as_secs() % 2 == 0 {
                self.color = Color::RED;
                self.text.set_scale(40.0);
            } else {
                self.color = Color::WHITE;
                self.text.set_scale(36.0);
            }
        }
        Ok(())
    }

    pub fn give_additional_time(&mut self, additional_time: Duration) {
        self.duration += additional_time;
    }

    pub fn take_time(&mut self, take: Duration) {
        self.duration -= take;
    }

    pub fn draw(&self, canvas: &mut graphics::Canvas) -> GameResult {
        let dest = Point2 { x: 100.0, y: 40.0 };

        let draw_params = graphics::DrawParam::default()
            .dest(dest)
            .offset(Point2 { x: 0.5, y: 0.5 })
            .color(self.color);

        canvas.draw(&self.text, draw_params);
        Ok(())
    }
}
