use ggez::graphics;
use ggez::mint::Point2;
use ggez::mint::Vector2;
use ggez::{Context, GameResult};
use std::path::Path;

const CARD_WIDTH: u32 = 125;

#[derive(Debug)]
enum CardTurning {
    TurnStart,
    TurnMiddle,
    TurnEnd,
}

#[derive(Debug)]
pub struct Card {
    card_back: graphics::Image,
    card_front: graphics::Image,
    pub match_id: u32,
    pub is_clicked: bool,
    pub turning: bool,
    turning_state: CardTurning,
    pub is_matched: bool,
    dest: Point2<f32>,
}

impl Card {
    pub fn new(
        ctx: &mut Context,
        card_back_path: &Path,
        card_front_path: &Path,
        dest: Point2<f32>,
        match_id: u32,
    ) -> GameResult<Card> {
        let card_back = graphics::Image::from_path(ctx, card_back_path)?;
        let card_front = graphics::Image::from_path(ctx, card_front_path)?;

        Ok(Card {
            card_back,
            card_front,
            match_id,
            is_clicked: false,
            turning: false,
            turning_state: CardTurning::TurnEnd,
            is_matched: false,
            dest,
        })
    }

    pub fn update(&mut self, _ctx: &mut Context) -> GameResult {
        match self.turning_state {
            CardTurning::TurnStart => {
                if self.is_matched {
                    self.turning_state = CardTurning::TurnEnd;
                } else {
                    self.turning_state = CardTurning::TurnMiddle
                }
            }
            CardTurning::TurnMiddle => {
                self.turning_state = CardTurning::TurnEnd;
                self.is_clicked = !self.is_clicked;
                self.turning = false;
            }
            CardTurning::TurnEnd => {
                if self.turning || self.is_matched {
                    self.turning_state = CardTurning::TurnStart;
                }
            }
        }

        Ok(())
    }

    pub fn draw(&self, canvas: &mut graphics::Canvas) -> GameResult {
        let mut dest: Point2<f32> = self.dest;
        if self.turning {
            match self.turning_state {
                CardTurning::TurnStart => {
                    dest.x += (CARD_WIDTH / 2) as f32;
                    if self.is_clicked {
                        let draw_params = graphics::DrawParam::default()
                            .dest(dest)
                            .scale(Vector2 { x: 0.20, y: 0.50 })
                            .offset(Vector2 { x: 0.50, y: 0.0 });

                        canvas.draw(&self.card_front, draw_params);
                    } else {
                        let draw_params = graphics::DrawParam::default()
                            .dest(dest)
                            .scale(Vector2 { x: 0.20, y: 0.50 })
                            .offset(Vector2 { x: 0.50, y: 0.0 });

                        canvas.draw(&self.card_back, draw_params);
                    }
                }
                CardTurning::TurnMiddle => {
                    dest.x += (CARD_WIDTH / 2) as f32;
                    if self.is_clicked {
                        let draw_params = graphics::DrawParam::default()
                            .dest(dest)
                            .scale(Vector2 { x: 0.20, y: 0.50 })
                            .offset(Vector2 { x: 0.50, y: 0.0 });

                        canvas.draw(&self.card_back, draw_params);
                    } else {
                        let draw_params = graphics::DrawParam::default()
                            .dest(dest)
                            .scale(Vector2 { x: 0.20, y: 0.50 })
                            .offset(Vector2 { x: 0.50, y: 0.0 });
                        canvas.draw(&self.card_front, draw_params);
                    }
                }
                CardTurning::TurnEnd => {
                    if self.is_matched {
                        return Ok(());
                    }
                    if self.is_clicked {
                        let draw_params = graphics::DrawParam::default()
                            .dest(dest)
                            .scale(Vector2 { x: 0.50, y: 0.50 });

                        canvas.draw(&self.card_front, draw_params);
                    } else {
                        let draw_params = graphics::DrawParam::default()
                            .dest(dest)
                            .scale(Vector2 { x: 0.50, y: 0.50 });

                        canvas.draw(&self.card_back, draw_params);
                    }
                }
            }
        } else {
            if self.is_clicked {
                let draw_params = graphics::DrawParam::default()
                    .dest(self.dest)
                    .scale(Vector2 { x: 0.50, y: 0.50 });

                canvas.draw(&self.card_front, draw_params);
            } else {
                let draw_params = graphics::DrawParam::default()
                    .dest(self.dest)
                    .scale(Vector2 { x: 0.50, y: 0.50 });

                canvas.draw(&self.card_back, draw_params);
            }
        }

        Ok(())
    }

    pub fn click(&mut self) {
        self.turning = true
    }
}
