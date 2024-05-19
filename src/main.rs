use ggez::event::{self, EventHandler};
use ggez::glam::Vec2;
use ggez::graphics::{self, Color, DrawParam};
use ggez::{Context, ContextBuilder, GameResult};

const WINDOW_SIZE_X: f32 = 750.0;
const WINDOW_SIZE_Y: f32 = 750.0;
const CELL_SIZE: f32 = 10.0;
const ROWS: f32 = WINDOW_SIZE_X / CELL_SIZE;
const HALF_A_SECOND: std::time::Duration = std::time::Duration::from_millis(500);

#[derive(Clone, Default, Debug)]
enum CellState {
    #[default]
    DEAD,
    ALIVE,
}

#[derive(Clone, Default, Debug)]
struct Cell {
    state: CellState,
    next_state: CellState,
    location: Vec2,
}

struct GameState {
    tick: std::time::Instant,
    grid: Vec<Vec<Cell>>,
    cell_mesh: graphics::Mesh,
}

impl Cell {
    pub fn new(pos: Vec2) -> Cell {
        Cell {
            state: CellState::DEAD,
            next_state: CellState::DEAD,
            location: pos,
        }
    }
}

impl GameState {
    pub fn new(_ctx: &mut Context) -> GameState {
        let rows_length = ROWS.trunc() as usize;
        let mut grid: Vec<Vec<Cell>> = vec![Vec::with_capacity(rows_length); rows_length];

        let cell_mesh = graphics::Mesh::new_rectangle(
            _ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new(0.0, 0.0, CELL_SIZE, CELL_SIZE),
            Color::WHITE,
        )
        .expect("Unable to create rectangle in Cell.draw()");

        for x in 0..rows_length {
            let mut row: Vec<Cell> = Vec::with_capacity(rows_length);
            for y in 0..rows_length {
                row.push(Cell::new(Vec2 {
                    x: x as f32,
                    y: y as f32,
                }));
            }
            grid.push(row);
        }

        GameState {
            grid,
            cell_mesh,
            tick: std::time::Instant::now(),
        }
    }
}

impl EventHandler for GameState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        if self.tick.elapsed() < HALF_A_SECOND {
            return Ok(());
        }
        self.tick = std::time::Instant::now();

        for row in &mut self.grid {
            for cell in row.iter_mut() {
                if cell.location.x % 2.0 != cell.location.y % 2.0 {
                    cell.state = CellState::ALIVE
                } else {
                    cell.state = CellState::DEAD
                };
            }
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::WHITE);

        for row in &self.grid {
            for cell in row {
                let color = match cell.state {
                    CellState::ALIVE => Color::BLACK,
                    CellState::DEAD => Color::WHITE,
                };

                let params = DrawParam::default()
                    .dest(cell.location * CELL_SIZE)
                    .color(color);

                canvas.draw(&self.cell_mesh, params);
            }
        }

        canvas.finish(ctx)?;
        ggez::timer::yield_now();
        Ok(())
    }
}

fn main() {
    let (mut ctx, event_loop) = ContextBuilder::new("ronway", "Joseph-D-Bradshaw")
        .window_setup(ggez::conf::WindowSetup::default().title("Ronway"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(WINDOW_SIZE_X, WINDOW_SIZE_Y))
        .build()
        .expect("Could not create a ggez context!");
    let game_state = GameState::new(&mut ctx);
    event::run(ctx, event_loop, game_state);
}
