use super::tetromino::{block_to_colour, Piece, PieceShape};
use super::BLOCK_SIZE;
use ggez::graphics::{Color, DrawMode, Point2, Rect};
use ggez::{graphics, Context, GameResult};

/// The y-offset to use as a starting point when drawing
pub const Y_OFFSET: f32 = 10f32;

/// The playfield. The top two rows are where pieces spawn but they are not
/// rendered.
#[derive(Debug)]
pub struct Well {
    data: [[u32; 10]; 22],
}

impl Well {
    pub fn new() -> Self {
        Well {
            data: [[0; 10]; 22],
        }
    }

    /// Add's the current piece, `current_t`, to the well.
    pub fn land(&mut self, current_t: &Piece) {
        let current_shape = current_t.get_shape();

        for (r, _) in current_shape.iter().enumerate() {
            for (c, _) in current_shape[r].iter().enumerate() {
                if current_shape[r][c] != 0 {
                    // add shape to well
                    self.data[r.wrapping_add(current_t.top_left.y as usize)]
                        [c.wrapping_add(current_t.top_left.x as usize)] = current_shape[r][c];
                }
            }
        }
    }

    /// Renders the well.
    pub fn draw(&self, ctx: &mut Context, image: &graphics::Image) -> GameResult<()> {
        // get starting position to draw window
        // TODO: doing all of this work every frame seems bad
        let width = graphics::get_screen_coordinates(ctx).w;
        let middle = width / 2.0;
        let starting_pos = middle - ((BLOCK_SIZE as f32 * self.data[0].len() as f32) / 2.0);

        // draw the "backside" of the well
        graphics::set_color(ctx, Color::from((15, 15, 15, 20)))?;
        graphics::rectangle(
            ctx,
            DrawMode::Fill,
            Rect {
                x: middle - (BLOCK_SIZE / 2.0),
                y: Y_OFFSET + ((BLOCK_SIZE / 2.0) * 20.0) + (BLOCK_SIZE / 2.0) * 3.0,
                w: BLOCK_SIZE * 10.0,
                h: BLOCK_SIZE * 20.0,
            },
        )?;

        for (r, _) in self.data.iter().enumerate() {
            if r < 2 {
                continue;
            } // don't drop top 2 rows
            for (c, _) in self.data[r].iter().enumerate() {
                if self.data[r][c] != 0 {
                    let colour = block_to_colour(self.data[r][c], false);
                    graphics::set_color(ctx, colour)?;

                    let x = starting_pos as f32 + (c as f32 * BLOCK_SIZE) as f32;
                    let y = Y_OFFSET + (r as f32 * BLOCK_SIZE) as f32;

                    graphics::draw(ctx, image, Point2::new(x, y), 0.0)?;
                } else {
                    graphics::set_color(ctx, Color::from((100, 100, 100, 20)))?;

                    graphics::rectangle(
                        ctx,
                        DrawMode::Line(1.0), // TODO: This may have to be changed
                        Rect {
                            x: starting_pos as f32 + (c as f32 * BLOCK_SIZE) as f32,
                            y: Y_OFFSET + (r as f32 * BLOCK_SIZE) as f32,
                            w: BLOCK_SIZE as f32,
                            h: BLOCK_SIZE as f32,
                        },
                    )?;
                }
            }
        }

        Ok(())
    }

    /// Checks for filled rows and clears any that are found. Returns the number
    /// of rows cleared.
    pub fn clear_lines(&mut self) -> u32 {
        let mut lines_cleared: u32 = 0;
        for r in (0..self.data.len()).rev() {
            let mut is_row_filled = true;
            for (c, _) in self.data[r].iter().enumerate() {
                if self.data[r][c] == 0 {
                    is_row_filled = false;
                    break; // no need to continue iterating, line is not clear...
                }
            }

            if is_row_filled {
                // TODO: implement more line clearing algorithms
                // TODO: make the current line clearing algorithm user selectable
                self.naive_line_clear(r);
                lines_cleared += 1;
            }
        }

        lines_cleared
    }

    /// Perform a line clear using the 'naive' algorith. Starting at the line to
    /// be cleared, pull the content of the row above down to the current row.
    /// If we are at row 0 (the top row), simply clear out that row.
    pub fn naive_line_clear(&mut self, starting_row: usize) {
        for row in (0..=starting_row).rev() {
            if row != 0 {
                self.data[row] = self.data[row - 1];
            } else {
                // if current row is 0, there is nothing above to pull down
                // clearing a line should always lead to top row being clear, so empty it
                self.data[row] = [0; 10];
            }
        }
    }

    /// Check if a collision would occur in the well given the shape and shape's
    /// position.
    pub fn check_for_collisions(&self, shape: &PieceShape, position: Point2) -> bool {
        let mut collision_found = false;

        for (r, _) in shape.iter().enumerate() {
            for (c, _) in shape[r].iter().enumerate() {
                if shape[r][c] != 0 && // if the space isn't empty AND one of the following:
                    (c as f32 + position.x < 0.0 || // outside left side
                         c as f32 + position.x >= self.data[r].len() as f32 || // outside right side
                         r as f32 + position.y >= self.data.len() as f32 || // outside bottom
                         self.data[(r as f32 + position.y) as usize][(c as f32 + position.x) as
                                                                         usize] !=
                             0)
                // space is not empty
                {
                    collision_found = true;
                }
            }
        }

        collision_found
    }

    /// Check if a landing would occur given the shape and the shape's position.
    pub fn check_for_landing(&self, shape: &PieceShape, position: Point2) -> bool {
        let mut collision_found = false;

        for (r, _) in shape.iter().enumerate() {
            for (c, _) in shape[r].iter().enumerate() {
                if shape[r][c] != 0 {
                    if r as f32 + position.y >= self.data.len() as f32 || // outside bottom
                        self.data[(r.wrapping_add(position.y as usize))][(c.wrapping_add(
                            position.x as
                                usize,
                        ))] != 0
                    // space is not empty
                    {
                        collision_found = true;
                    } else if (c as f32 + position.x) >= self.data[r].len() as f32
                        || (c as f32 + position.x) < 0.0
                    {
                        // do nothing
                    }
                }
            }
        }

        collision_found
    }
}
