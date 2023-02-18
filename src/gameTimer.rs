use ggez::graphics;
use ggez::graphics::Color;
use ggez::mint::Point2;
use ggez::{Context, GameResult};
use std::time::Duration;
use std::time::Instant;

pub struct GameTimer {
    pub text: graphics::Text,
    pub start: Instant,
    pub color: Color,
    pub remaining: Duration,
    pub duration: Duration,
}

impl GameTimer {
    pub fn new(_ctx: &mut Context, start: Instant, duration: Duration) -> GameResult<GameTimer> {
        let text = graphics::Text::new("");

        Ok(GameTimer {
            text,
            start,
            color: Color::WHITE,
            remaining: duration,
            duration,
        })
    }

    pub fn update(&mut self, _ctx: &mut Context) -> GameResult {
        let elapsed = self.start.elapsed();
        if elapsed >= self.duration {
            self.remaining = Duration::new(0, 0)
        } else {
            self.remaining = self.duration - elapsed;
            let minutes = self.remaining.as_secs() / 60;
            let seconds = self.remaining.as_secs() % 60;
            self.text = graphics::Text::new(&format!("{:02}:{:02}", minutes, seconds));

            // Each second change the size and the color
            if self.remaining.as_secs() % 2 == 0 {
                self.color = Color::RED;
                self.text.set_scale(40.0);
            } else {
                self.color = Color::WHITE;
                self.text.set_scale(36.0);
            }
        }
        Ok(())
    }

    pub fn give_additional_time(&mut self, additional_time: Duration) -> Duration {
        self.duration += additional_time;
        return self.duration;
    }

    pub fn take_time(&mut self, take: Duration) -> Duration {
        self.duration -= take;
        return self.duration;
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
