use std::time::Duration;

static INIT_STATE: &str = r"
..*..
...*.
.***.
.....";

const WIDTH: i32 = 40;
const HEIGHT: i32 = 20;
const ITERATIONS: usize = 100;
const BENCHMARK: bool = false;

/// A two-dimensional matrix.
pub struct Matrix {
    width: i32,
    // [0..2**32)
    height: i32,
    data: Vec<bool>,
}

impl Matrix {
    /// Allocate a new matrix with the specified size.
    pub fn new(width: i32, height: i32) -> Matrix {
        assert!(width >= 0 && height >= 0, "size must be positive");
        let size = (width as usize) * (height as usize);
        let mut data = Vec::with_capacity(size);
        data.resize(size, false);
        Matrix {
            width,
            height,
            data,
        }
    }

    pub fn read(&self, x: i32, y: i32) -> bool {
        debug_assert!(x >= 0 && x < self.width);
        debug_assert!(y >= 0 && y < self.height);
        self.data[(y * self.width + x) as usize]
    }

    pub fn read_with_wrap(&self, mut x: i32, mut y: i32) -> bool {
        if x < 0 {
            x += self.width;
        } else if x >= self.width {
            x -= self.width;
        }
        if y < 0 {
            y += self.height;
        } else if y >= self.height {
            y -= self.height;
        }

        debug_assert!(x >= 0 && x < self.width);
        debug_assert!(y >= 0 && y < self.height);

        self.data[(y * self.width + x) as usize]
    }

    pub fn write(&mut self, x: i32, y: i32, value: bool) {
        debug_assert!(x >= 0 && x < self.width);
        debug_assert!(y >= 0 && y < self.width);
        self.data[(y * self.width + x) as usize] = value;
    }
}

pub fn count_neighbours(m: &Matrix, x: i32, y: i32) -> u32 {
    let mut count = 0;
    for row in -1..=1 {
        for col in -1..=1 {
            if (col, row) != (0, 0) && m.read_with_wrap(x + col, y + row) {
                count += 1;
            }
        }
    }
    count
}

pub fn update_state(old_state: &Matrix, new_state: &mut Matrix) {
    for y in 0..old_state.height {
        for x in 0..old_state.width {
            let count = count_neighbours(old_state, x, y);
            new_state.write(x, y, count == 3 || count == 2 && old_state.read(x, y));
        }
    }
}

pub fn run_simulation(mut old_state: Matrix, steps: usize) -> Matrix {
    let mut new_state = Matrix::new(old_state.width, old_state.height);

    for _ in 0..steps {
        update_state(&old_state, &mut new_state);
        print_state(&new_state);
        delay(50);
        std::mem::swap(&mut old_state, &mut new_state);
    }

    old_state
}

pub fn state_from_str(state: &mut Matrix, source: &str) {
    let mut row = 0i32;
    let mut col = 0i32;
    for ch in source.chars() {
        if ch == '\n' {
            col = 0;
            row += 1;
            if row >= state.height {
                break;
            }
            continue;
        }
        if col < state.width {
            state.write(col, row, ch == '*');
        }
        col += 1;
    }
}

pub fn clear_screen() {
    eprint!("\x1b[2J\x1b[0;0H");
}

pub fn print_state(state: &Matrix) {
    if BENCHMARK {
        return;
    }
    clear_screen();
    let mut col = 0i32;
    for b in &state.data {
        eprint!("{}", if *b { '*' } else { '.' });
        col += 1;
        if col == state.width {
            col = 0;
            eprintln!();
        }
    }
}

pub fn delay(millis: u64) {
    if BENCHMARK {
        return;
    }
    std::thread::sleep(Duration::from_millis(millis));
}

fn main() {
    let mut state = Matrix::new(WIDTH, HEIGHT);
    state_from_str(&mut state, INIT_STATE);
    print_state(&state);
    run_simulation(state, ITERATIONS);
}
