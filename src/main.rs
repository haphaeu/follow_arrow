use nannou::prelude::*;

static INTRO_TEXT: &str =
    "Click on an arrow to move it. Arrows only move in the direction they are pointing, if
there is either an empty space next to it, or if it can jump over another arrow into
an empty space.
The goal is to move all left-pointing arrows to the left, and all right-pointing arrows
to the right.
Click the mouse to start.
";

static GAMEOVER_TEXT: &str = "No more valid moves. Game over.

Click the mouse to try again.
";

static SOLVED_TEXT: &str = "You did it! Congratulations!

Click the mouse to re-start.
";

#[derive(PartialEq)]
enum State {
    Intro,
    Playing,
    Solved,
    GameOver,
}
use State::*;

#[derive(PartialEq, Clone, Copy, Debug)]
enum Arrow {
    Left,
    Right,
    Empty,
}
use Arrow::*;

struct Model {
    state: State,
    boards: Vec<Vec<Arrow>>,
}
impl Model {
    fn new() -> Self {
        Self {
            state: Intro,
            boards: vec![vec![
                Right, Right, Right, Right, Empty, Left, Left, Left, Left,
            ]],
        }
    }
    /// Reset board
    fn reset(&mut self) {
        self.state = Playing;
        self.boards = vec![vec![
            Right, Right, Right, Right, Empty, Left, Left, Left, Left,
        ]];
    }
    // undo last move
    fn undo(&mut self) {
        if self.boards.len() > 1 {
            self.boards.pop();
        }
    }
    /// Returns the index of the empty space.
    fn index_empty(&self) -> usize {
        self.boards
            .last()
            .unwrap()
            .iter()
            .position(|&x| x == Empty)
            .unwrap()
    }
    /// When the user clicks on an arrow, this function checks
    /// if that arrow can be moved and returns `true` if the arrow
    // can be moved, and `false` otherwise.
    fn is_move_valid(&self, index: usize) -> bool {
        let empty = self.index_empty();
        match (self.boards.last().unwrap()[index], index, empty) {
            (Left, i, e) if i == e + 1 || i == e + 2 => true,
            (Right, i, e) if (e > 0 && i == e - 1) || (e > 1 && i == e - 2) => true,
            _ => false,
        }
    }
    /// Check if the current board has any valid moves left.
    fn is_game_over(&self) -> bool {
        for (i, &a) in self.boards.last().unwrap().iter().enumerate() {
            if a != Empty && self.is_move_valid(i) {
                return false;
            }
        }
        true
    }
    /// Move the arrow at `index` to the empty space.
    /// Check if the move is valid.
    fn try_move(&mut self, index: usize) {
        println!("Trying to move arrow at index {}", index);
        match self.is_move_valid(index) {
            true => {
                println!("Move is valid");
                let empty = self.index_empty();
                self.boards.push(self.boards.last().unwrap().clone());
                self.boards.last_mut().unwrap().swap(index, empty);
                println!("{:?}", self.boards.last().unwrap());
                if *self.boards.last().unwrap()
                    == vec![Left, Left, Left, Left, Empty, Right, Right, Right, Right]
                {
                    println!("Solved!");
                    self.state = Solved;
                } else if self.is_game_over() {
                    println!("Game over!");
                    self.state = GameOver;
                }
            }
            false => {
                println!("Move is invalid");
                ()
            }
        }
    }
}

fn main() {
    nannou::app(model).loop_mode(LoopMode::Wait).run();
}

fn model(app: &App) -> Model {
    let _window = app
        .new_window()
        .size(900, 100)
        .title("Follow the arrows")
        .view(view)
        .event(event)
        .build()
        .unwrap();
    Model::new()
}

fn event(app: &App, model: &mut Model, event: WindowEvent) {
    match event {
        MousePressed(_button) => match model.state {
            Intro => {
                model.state = Playing;
            }
            Playing => {
                // Check if the user clicked on an arrow
                // and move it if it can be moved.
                println!("Playing mode");
                println!("Clicked at {}, {}", app.mouse.x, app.mouse.y);
                let w = app.window_rect().w();
                let index_clicked = (9.0 * (app.mouse.x + w / 2.0) / w) as usize;
                println!("Index clicked: {}", index_clicked);
                model.try_move(index_clicked);
            }
            GameOver | Solved => {
                model.reset();
            }
        },
        KeyPressed(Key::R) => model.reset(),
        KeyPressed(Key::U) if model.state == Playing => model.undo(),
        _ => (),
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    frame.clear(WHITE);

    let draw = app.draw();

    // draw the board
    match model.state {
        Playing | Solved | GameOver => {
            let win = app.window_rect();
            let pad = win.h() / 5.0;
            let cell_width = (win.w() - 2.0 * pad) / 9 as f32;
            let cell_height = win.h() - pad;
            // draw all the cells
            for col in 0..9 {
                let x = win.left() + pad + col as f32 * cell_width + cell_width / 2.0;
                let y = 0.0;
                // draw the cell
                draw.rect()
                    .x_y(x, y)
                    .w_h(cell_width, cell_height)
                    .color(WHITE)
                    .stroke(BLACK)
                    .stroke_weight(1.0);
                // draw the arrow
                let arrow = model.boards.last().unwrap()[col];
                let arrow_color = match arrow {
                    Left => RED,
                    Right => BLUE,
                    Empty => WHITE,
                };
                let arrow_size = cell_width / 2.0;
                // draw the shaft of the arrow using a rectangle
                draw.rect()
                    .x_y(x, y)
                    .w_h(arrow_size, arrow_size / 2.0)
                    .color(arrow_color);
                // draw the head of the arrow using a triangle
                match arrow {
                    Left => {
                        draw.tri()
                            .points(
                                pt2(x - arrow_size / 2.0, y - arrow_size / 2.0),
                                pt2(x - arrow_size / 2.0, y + arrow_size / 2.0),
                                pt2(x - arrow_size / 1.0, y),
                            )
                            .color(arrow_color);
                    }
                    Right => {
                        draw.tri()
                            .points(
                                pt2(x + arrow_size / 2.0, y - arrow_size / 2.0),
                                pt2(x + arrow_size / 2.0, y + arrow_size / 2.0),
                                pt2(x + arrow_size / 1.0, y),
                            )
                            .color(arrow_color);
                    }
                    Empty => (),
                };
            }
        }
        Intro => (),
    }

    // draw any text
    match model.state {
        Intro | Solved | GameOver => {
            let text = match model.state {
                Intro => format!("{INTRO_TEXT}"),
                Solved => format!("{SOLVED_TEXT}"),
                GameOver => format!("{GAMEOVER_TEXT}"),
                _ => format!(""),
            };
            let winp = app.window_rect().pad(20.0);
            let text_area = geom::Rect::from_wh(winp.wh()).top_left_of(winp);
            draw.text(&text)
                .xy(text_area.xy())
                .wh(text_area.wh())
                .align_text_bottom()
                .left_justify()
                .color(RED);
        }
        // no text during playing
        Playing => (),
    }

    draw.to_frame(app, &frame).unwrap();
}
