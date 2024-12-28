use crate::ioscreen;
use std::thread;
use std::time::Duration;

const REFRESH_RATE_MSEC: u64 = 500;
const GAME_CANVAS_WIDTH: usize = 80;
const GAME_CANVAS_HEIGHT: usize = 25;
const GLASS_WIDTH: usize = 10;
const GLASS_HEIGHT: usize = 20;


enum Position {
    P1x4([[bool; 1]; 4]),
    P4x2([[bool; 4]; 2]),
    P2x3([[bool; 2]; 3]),
    P3x2([[bool; 3]; 2]),
    P2x2([[bool; 2]; 2]),
}

impl Position {
    fn position_size(&self) -> (usize, usize) {
        match self {
            Self::P1x4(_) => (1, 4),
            Self::P2x2(_) => (2, 2),
            Self::P2x3(_) => (2, 3),
            Self::P3x2(_) => (2, 3),
            Self::P3x2(_) => (3, 2),
            Self::P4x2(_) => (4, 2)
        }
    }

    fn relative_coordinates_list(&self) -> Vec<(usize, usize)> {
        let (rows, cols) = self.position_size();
        let mut coordinates = Vec::new();
        match self {
            Self::P1x4(matrix) => {
                for row in 0..4 {
                    if matrix[row][0] {
                        coordinates.push((0, row));
                    }
                }
            }
            Self::P2x2(matrix) => {
                for row in 0..2 {
                    for col in 0..2 {
                        if matrix[row][col] {
                            coordinates.push((col, row));
                        }
                    }
                }
            }
            Self::P2x3(matrix) => {
                for row in 0..3 {
                    for col in 0..2 {
                        if matrix[row][col] {
                            coordinates.push((col, row));
                        }
                    }
                }
            }
            Self::P3x2(matrix) => {
                for row in 0..2 {
                    for col in 0..3 {
                        if matrix[row][col] {
                            coordinates.push((col, row));
                        }
                    }
                }
            }
            Self::P4x2(matrix) => {
                for row in 0..2 {
                    for col in 0..4 {
                        if matrix[row][col] {
                            coordinates.push((col, row));
                        }
                    }
                }
            }
        }
        coordinates
    }
}

enum PositionOptions {
    P1(Position),
    P2(Position, Position),
    P4(Position, Position, Position, Position),
}

impl PositionOptions {
    fn position_by_index(&self, idx: usize)->Option<&Position> {
        match self {
            Self::P1(p1) => match idx {
                0 => Some(p1),
                _ => None
            },
            Self::P2(p1, p2) => match idx {
                0 => Some(p1),
                1 => Some(p2),
                _ => None
            },
            Self::P4(p1, p2, p3, p4) => match idx {
                0 => Some(p1),
                1 => Some(p2),
                2 => Some(p3),
                4 => Some(p3),
                _ => None
            },
        }
    }
}

enum Shape {
    I(PositionOptions),
    J(PositionOptions),
    L(PositionOptions),
    O(PositionOptions),
    S(PositionOptions),
    T(PositionOptions),
    Z(PositionOptions)
}

impl Shape {
    fn get_position_options(&self) -> &PositionOptions {
        match &self {
            Self::I(v) | Self::J(v) | Self::L(v) | Self::O(v) | Self::T(v) | Self::Z(v) | Self::S(v) => v
        }
    }

    fn position_by_index(&self, index: usize)->Option<&Position> {
        self.get_position_options().position_by_index(index)
    }

    fn get_shape_coordinates(&self, position: usize, x: usize, y: usize)->Vec<(usize, usize)> {
        let mut coordinates = Vec::new();
        for (rel_x, rel_y) in self.position_by_index(position).unwrap().relative_coordinates_list() {
            coordinates.push((x as usize + rel_x, y as usize + rel_y));
        }
        coordinates
    }
}

const ISHAPE: Shape = Shape::I(PositionOptions::P2(
        Position::P1x4([[true], [true], [true], [true]]),
        Position::P4x2([[false, false, false, false], [true, true, true, true]])
    )
);

const JSHAPE: Shape = Shape::J(PositionOptions::P4(
        Position::P2x3([[false, true], [false, true], [true, true]]),
        Position::P3x2([[true, false, false], [true, true, true]]),
        Position::P2x3([[true, true], [true, false], [true, false]]),
        Position::P3x2([[true, true, true], [false, false, true]])
    )
);

const LSHAPE: Shape = Shape::L (PositionOptions::P4(
        Position::P2x3([[true, false], [true, false], [true, true]]),
        Position::P3x2([[true, true, true], [true, false, false]]),
        Position::P2x3([[true, true], [false, true], [false, true]]),
        Position::P3x2([[false, false, true], [true, true, true]])
    )
);

const OSHAPE: Shape = Shape::O(PositionOptions::P1(Position::P2x2([[true, true], [true, true]])));

const SSHAPE: Shape = Shape::S(PositionOptions::P2(
        Position::P3x2([[false, true, true], [true, true, false]]),
        Position::P2x3([[true, false],[true, true], [false, true]])
    )
);

const TSHAPE: Shape = Shape::T(PositionOptions::P4(
        Position::P3x2([[true, true, true], [false, true, false]]),
        Position::P2x3([[false, true], [true, true], [false, true]]),
        Position::P3x2([[false, true, false], [true, true, true]]),
        Position::P2x3([[true, false], [true, true], [true, true]])
    )
);

const ZSHAPE: Shape = Shape::Z(PositionOptions::P2(
        Position::P3x2([[true, true, false], [false, true, true]]),
        Position::P2x3([[false, true], [true, true], [true, false]])
    )
);

const SHAPES: [Shape; 7] = [ISHAPE, JSHAPE, LSHAPE, OSHAPE, SSHAPE, TSHAPE, ZSHAPE];

struct ShapeState {
    shape: &'static Shape,
    position: usize,
    x: usize,
    y: usize
}

impl ShapeState {
    fn new(shape: &'static Shape) -> Self {
        let x = GLASS_WIDTH as usize / 2usize;
        let y = 0usize;
        let position = 0usize;
        ShapeState { shape, position, x, y}
    }

    fn get_coordinates(&self)->Vec<(usize, usize)> {
        self.shape.get_shape_coordinates(self.position as usize, self.x as usize, self.y as usize)
    }
}

pub struct Game {
    current_shape: ShapeState,
    screen_canvas: ioscreen::Canvas,
    glass: [[bool; GLASS_WIDTH]; GLASS_HEIGHT],
}

impl Game {
    pub fn new()->Self {
        let mut glass: [[bool; GLASS_WIDTH]; GLASS_HEIGHT] = Default::default();
        for t in &mut glass {t.fill(false)}

        Game {
            current_shape: ShapeState::new(&TSHAPE),
            screen_canvas: ioscreen::Canvas::new(GAME_CANVAS_WIDTH, GAME_CANVAS_HEIGHT),
            glass
        }
    }

    fn update_glass_with_shape(&mut self) {
        // Draw shape
        let coordinates = self.current_shape.get_coordinates();
        for (x, y) in coordinates {
            self.glass[y][x] = true;
        }

    }

    fn update_glass(&mut self) {
        self.update_glass_with_shape();
    }

    fn glass_lr()-> (usize, usize) {
        let glass_left = GAME_CANVAS_WIDTH/2 - GLASS_WIDTH;
        let glass_right: usize = glass_left + GLASS_WIDTH*2;
        (glass_left, glass_right)
    }

    fn draw_text(&mut self, text: &str, x: usize, y: usize) {
        let mut char_x_pos = x;
        for character in text.chars() {
            self.screen_canvas.set_char(char_x_pos, y, character);
            char_x_pos += 1;
        }
    }

    fn draw_glass_inside(&mut self) {
        let (glass_left, _) = Self::glass_lr();
        for y in 0..GLASS_HEIGHT {
            for x in 0..GLASS_WIDTH {
                let (char1, char2) = if self.glass[y][x] {('█', '█')} else {(' ', '.')};
                self.screen_canvas.set_char(glass_left + x*2 + 0, y, char1);
                self.screen_canvas.set_char(glass_left + x*2 + 1, y, char2);
            }
        }
    }

    fn draw_glass_outside(&mut self) {
        self.draw_text("ПОЛНЫХ СТРОК: ", 0, 0);
        self.draw_text("УРОВЕНЬ:      ", 0, 1);
        self.draw_text("  СЧЕТ:  ", 0, 2);

        let (glass_left, glass_right) = Self::glass_lr();
        self.draw_text("7: НАЛЕВО   9: НАПРАВО", glass_right + 4, 1);
        self.draw_text("     8:ПОВОРОТ        ", glass_right + 4, 2);
        self.draw_text("4:УСКОРИТЬ  5:СБРОСИТЬ", glass_right + 4, 3);
        self.draw_text("1: ПОКАЗАТЬ  СЛЕДУЮЩУЮ", glass_right + 4, 4);
        self.draw_text("0:  СТЕРЕТЬ ЭТОТ ТЕКСТ", glass_right + 4, 5);
        self.draw_text("  ПРОБЕЛ - СБРОСИТЬ   ", glass_right + 4, 6);
    }

    fn draw_glass(&mut self) {
        let (glass_left, glass_right) = Self::glass_lr();
        for line_idx in 0..GLASS_HEIGHT {
            self.screen_canvas.set_char(glass_left - 2, line_idx, '<');
            self.screen_canvas.set_char(glass_left - 1, line_idx, '!');
            for dot_x in glass_left..glass_right {
                if dot_x % 2 == 0 {
                    self.screen_canvas.set_char(dot_x, line_idx, ' ');
                } else {
                    self.screen_canvas.set_char(dot_x, line_idx, '.');
                }
            }
            self.screen_canvas.set_char(glass_right, line_idx, '!');
            self.screen_canvas.set_char(glass_right + 1, line_idx, '>');
        }
        self.screen_canvas.set_char(glass_left - 2, GLASS_HEIGHT, '<');
        self.screen_canvas.set_char(glass_left - 1, GLASS_HEIGHT, '!');
        for dot_x in glass_left..glass_right {
            self.screen_canvas.set_char(dot_x, GLASS_HEIGHT, '=');
            if dot_x % 2 == 0 {
                self.screen_canvas.set_char(dot_x, GLASS_HEIGHT + 1, '\\');
            } else {
                self.screen_canvas.set_char(dot_x, GLASS_HEIGHT + 1, '/');
            }
        }
        self.screen_canvas.set_char(glass_right, GLASS_HEIGHT, '!');
        self.screen_canvas.set_char(glass_right + 1, GLASS_HEIGHT, '>');
    }

    fn draw_frame(&mut self) {
        self.draw_glass();
        self.draw_glass_inside();
        self.draw_glass_outside();
        self.screen_canvas.display();
    }

    pub fn game_loop(&mut self) {
        ioscreen::clear_screen();
        self.current_shape = ShapeState::new(&SSHAPE);
        loop {
            self.update_glass();
            self.screen_canvas.clear();
            self.draw_frame();
            self.current_shape.y += 1;
            thread::sleep(Duration::from_millis(REFRESH_RATE_MSEC));
        }
    }
}