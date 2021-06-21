use rand::Rng;

#[derive(Debug)]
pub struct Ship {
    // idx_first_deck: Index of the position of the first deck of the ship.
    idx_first_deck: usize,
    // is_horizontal_pos: Ship position (true: horizontal, false: vertical).
    is_horizontal_pos: bool,
    // is_alive: Ship status (false: broken; true: whole).
    is_alive: bool,
    // size: Ship size (1,2,3 or 4)
    size: usize,
    // fire_cell_idxes: vec with cell indices in which hit
    fire_cell_idxes: Vec<usize>,

}

impl Ship {
    /// Returns the number of shots on the ship.
    fn get_num_shot(&self) -> usize {
        self.fire_cell_idxes.len()
    }
    pub fn get_fire_cell_idxes(&self) -> &Vec<usize> {
        self.fire_cell_idxes.as_ref()
    }
    /// Return is alive.
    pub fn check_is_alive(&self) -> bool { self.is_alive }
    /// Will return "Truth" if the ship is alive but wounded.
    pub fn check_is_wounded(&self) -> bool {
        self.is_alive && self.get_num_shot() < self.size && self.get_num_shot() != 0
    }
    /// We carry out a successful shot at the ship. Return "true" if the ship is alive
    pub fn knock_down(&mut self, idx: usize) -> bool {
        self.fire_cell_idxes.push(idx);
        self.fire_cell_idxes.sort();
        if self.get_num_shot() >= self.size {
            self.is_alive = false;
        }
        self.is_alive
    }
    /// Setting a new location and position for the ship
    fn _set_location_and_position(&mut self, idx_first_deck: usize, is_horizontal_pos: bool) {
        self.idx_first_deck = idx_first_deck;
        self.is_horizontal_pos = is_horizontal_pos;
        // Bringing the ship back to life
        self.is_alive = true;
    }

    /// Returns "True" if the ship has a deck with the given index
    pub fn check_idx_for_ship(&self, idx: usize) -> bool {
        let mut result = false;
        for &idx_ship in self.get_area_ship().iter() {
            if idx == idx_ship {
                result = true;
                break;
            }
        };
        result
    }

    /// Returns the indexes of the cells that the ship occupies.
    pub fn get_area_ship(&self) -> Vec<usize> {
        let mut set_idx: Vec<usize> = Vec::new();
        for i in 0..self.size {
            if self.is_horizontal_pos {
                set_idx.push(self.idx_first_deck + i);
            } else {
                set_idx.push(self.idx_first_deck + i * 10);
            }
        }
        set_idx
    }
    /// Returns the indexes of the cells near the ship.
    pub fn get_area_near_ship(&self, area_ship: Option<Vec<usize>>) -> Vec<usize> {
        let mut set_idx: Vec<usize> = Vec::new();
        let set_ship_idx: Vec<usize> = match area_ship {
            None => { self.get_area_ship() }
            Some(v) => { v }
        };
        let x = self.idx_first_deck % 10;
        let y = self.idx_first_deck / 10;
        let y_start = if y == 0 { y } else { y - 1 };
        let x_start = if x == 0 { x } else { x - 1 };
        let x_end = if self.is_horizontal_pos {
            if x + self.size <= 9 { x + self.size } else { 9 }
        } else {
            if x + 1 <= 9 { x + 1 } else { 9 }
        };

        let y_end = if self.is_horizontal_pos {
            if y + 1 <= 9 { y + 1 } else { 9 }
        } else {
            if y + self.size <= 9 { y + self.size } else { 9 }
        };

        let mut ship_idx_iter = set_ship_idx.iter();
        for i in x_start..(x_end + 1) {
            for j in y_start..(y_end + 1) {
                match ship_idx_iter.find(|&&idx| idx == (j * 10 + i)) {
                    None => {
                        set_idx.push(j * 10 + i);
                    }
                    Some(_) => {}
                }
            }
        }
        set_idx
    }
}

/// Returns a vector of Ships with a random position.
pub fn get_default_fleet() -> Vec<Ship> {
    let mut ships: Vec<Ship> = Vec::new();
    // fleet: Corresponds to the number of available ships with the value of their size.
    let fleet: [usize; 10] = [4, 3, 3, 2, 2, 2, 1, 1, 1, 1];
    // vector_of_free_cells: Vector for tracking free cells.
    // If the element of the vector with the idx index has a value of 0,
    // then the cell of the playing field with the idx index
    // is available for placing a ship on it.
    let mut vector_of_free_cells: Vec<usize> = vec![0; 10 * 10];
    let mut rng = rand::thread_rng();
    for ship_size in fleet.iter() {
        loop {
            let is_horizontal_pos: bool = rng.gen::<bool>();
            // We generate indexes of the start and end of the ship, depending on the positioning.
            let (idx_start, idx_end) = if !is_horizontal_pos {
                let row = rng.gen_range(0, 11 - ship_size);
                let col = rng.gen_range(0, 10);
                (row * 10 + col, (row + ship_size - 1) * 10 + col)
            } else {
                let col = rng.gen_range(0, 11 - ship_size);
                let row = rng.gen_range(0, 10);
                (row * 10 + col, row * 10 + col + ship_size - 1)
            };
            // We check if the generated indexes are free to place the ship.
            if vector_of_free_cells[idx_start] == 0 && vector_of_free_cells[idx_end] == 0 {
                let new_ship = Ship {
                    idx_first_deck: idx_start,
                    is_horizontal_pos,
                    is_alive: true,
                    size: ship_size.clone(),
                    fire_cell_idxes: Vec::new(),
                };
                let area_ship: Vec<usize> = new_ship.get_area_ship();
                for &idx in &area_ship {
                    vector_of_free_cells[idx] = 1;
                };
                for idx in new_ship.get_area_near_ship(Some(area_ship)) {
                    vector_of_free_cells[idx] = 1;
                }
                ships.push(new_ship);
                break;
            }
        }
    }
    ships
}