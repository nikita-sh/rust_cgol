#[derive(Debug)]
pub struct Grid {
    pub width: usize,
    pub height: usize,
    pub values: Vec<Vec<bool>>
}

impl Grid {
    pub fn change_coord(&mut self, x: usize, y: usize, new_val: bool) -> i32 {
        if x < 0 || x > self.width || y < 0 || y > self.height {
            println!("Invalid coordinates: ({},{}) for grid with width {} and height {}", x, y, self.width, self.height);
            return 1
        }

        self.values[x][y] = new_val;
        0
    }

    pub fn change_nth(&mut self, n: usize, new_val: bool) -> i32 {
        if n >= self.width*self.height || n < 0 {
            println!("Invalid pixel: {} for grid with {} pixels", n, self.width*self.height);
            return 1
        }

        let y : usize = n % self.width;
        let x : usize = n / self.width;

        self.values[x][y] = new_val;
        0
    }
}