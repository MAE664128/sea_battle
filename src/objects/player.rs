use crate::objects::ship::{Ship, get_default_fleet};
use std::slice::{Iter, IterMut};
use rand::Rng;

pub struct Player {
    name: String,
    ships: Vec<Ship>,
    misses_shots: Vec<usize>,
    fire_shots: Vec<usize>,
    _is_manual_control: bool,
}


impl Player {
    pub fn create(name: &str, is_manual_control: bool) -> Self {
        Self {
            name: name.to_string(),
            ships: get_default_fleet(),
            misses_shots: Vec::new(),
            fire_shots: Vec::new(),
            _is_manual_control: is_manual_control,
        }
    }
    pub fn _set_name(&mut self, name: &str) {
        self.name = name.to_string()
    }
    pub fn _set_type_control(&mut self, is_manual_control: bool) {
        self._is_manual_control = is_manual_control
    }
    pub fn get_name(&self) -> &str {
        self.name.as_str()
    }
    pub fn get_ships_as_iter_mut(&mut self) -> IterMut<'_, Ship> {
        self.ships.iter_mut()
    }
    pub fn get_ships_as_iter(&self) -> Iter<'_, Ship> {
        self.ships.iter()
    }
    /// Return the number of living ships
    pub fn get_num_living_ships(&self) -> usize {
        let mut count: usize = 0;
        for ship in &self.ships {
            if ship.check_is_alive() { count += 1; }
        }
        count
    }

    // Returns a mutable reference to the ship by index, or None
    pub fn get_ship_by_idx_as_mut_ref(&mut self, idx_ship: usize) -> Option<&Ship> {
        if idx_ship <= self.ships.len() {
            Some(&self.ships[idx_ship])
        } else {
            None
        }
    }

    // Returns a reference to the ship by index, or None
    pub fn _get_ship_by_idx_as_ref(&self, idx_ship: usize) -> Option<&Ship> {
        if idx_ship <= self.ships.len() {
            Some(&self.ships[idx_ship])
        } else {
            None
        }
    }


    /// Process the shot to the cell with the idx index and,
    /// in case of a successful hit, return "True" as the second value.
    /// If the shot was fatal, then the third value is to return "False"
    pub fn process_a_shot(&mut self, idx_cell: usize) -> (Option<usize>, bool, bool) {
        let mut number_ship: Option<usize> = None;
        let mut is_successful_shot: bool = false;
        let mut is_alive_ship: bool = true;

        for (idx_ship, ship) in self.get_ships_as_iter_mut().enumerate() {
            if ship.check_idx_for_ship(idx_cell) {
                number_ship = Some(idx_ship);
                is_successful_shot = true;
                is_alive_ship = ship.knock_down(idx_cell);
                if !is_successful_shot {
                    self.misses_shots.push(idx_ship)
                } else {
                    self.fire_shots.push(idx_ship)
                };
                break;
            }
        }

        (number_ship, is_successful_shot, is_alive_ship)
    }

    /// Own shot in your fleet
    pub fn generate_new_auto_shot_idx(&self) -> usize {
        // TODO This is a sketch for choosing a shooting location.
        // TODO You need to come back later and rethink this code.
        let mut rng = rand::thread_rng();
        let mut variant_shot: Vec<usize> = Vec::new();
        // Check if there are any damaged (but not killed) ships and make a shot next to them
        for ship in self.get_ships_as_iter() {
            if ship.check_is_wounded() {
                // Take the indexes of the padded decks.
                let fire_cell_idxes: &Vec<usize> = ship.get_fire_cell_idxes();
                if fire_cell_idxes.len() == 1 {
                    // If there is only one hit,
                    // then we form options for shots in 4 directions
                    variant_shot = self.calculate_near_area(fire_cell_idxes[0]);
                } else if fire_cell_idxes.len() > 1 {
                    // If there was more than one hit,
                    // then we determine the direction of fire and form options for the shot.
                    let is_horizontal: bool = fire_cell_idxes[0] % 10 == fire_cell_idxes[1] % 10;
                    for (i, idx_cell) in fire_cell_idxes.iter().enumerate() {
                        let calc_idx: usize = if is_horizontal { idx_cell % 10 } else { idx_cell / 10 };
                        let direction_k: usize = if is_horizontal { 1 } else { 10 };
                        if calc_idx != 0
                            &&
                            self.misses_shots.iter()
                                .find(|&&idx| idx == idx_cell - 1 * direction_k).is_none()
                            &&
                            fire_cell_idxes.iter()
                                .find(|&&idx| idx == idx_cell - 1 * direction_k).is_none() {
                            variant_shot.push(idx_cell.clone());
                        }
                        if i == fire_cell_idxes.len() - 1
                            &&
                            calc_idx != 9
                            &&
                            self.misses_shots.iter()
                                .find(|&&idx| idx == (idx_cell + 1 * direction_k)).is_none()
                            &&
                            fire_cell_idxes.iter()
                                .find(|&&idx| idx == (idx_cell + 1 * direction_k)).is_none() {
                            variant_shot.push(idx_cell.clone());
                        }
                    }
                }
                // There is enough information about one ship.
                break;
            }
        }
        let result_val: usize = if variant_shot.len() != 0 {
            variant_shot[rng.gen_range(0, variant_shot.len())]
        } else {
            let mut count: usize = 0;
            let mut tmp_idx: usize = 0;
            loop {
                tmp_idx = rng.gen_range(0, 100);
                // Check if the generated index is missing from the miss and fire list
                if self.misses_shots.iter().find(|&&idx| idx == tmp_idx).is_none() {
                    if self.fire_shots.iter().find(|&&idx| idx == tmp_idx).is_none() {
                        break;
                    }
                }
                if count > 100 { break; }
                count += 1;
            };
            tmp_idx
        };
        result_val
    }

    /// Returns the vec of indices around the cell at the given index.
    fn calculate_near_area(&self, idx: usize) -> Vec<usize> {
        let mut variant_shot: Vec<usize> = Vec::new();
        let x = idx % 10;
        let y = idx / 10;
        let x_end = if x + 1 <= 9 { x + 1 } else { x };
        let x_start = if x == 0 { 0 } else { x - 1 };
        let y_end = if y + 1 <= 9 { y + 1 } else { y };
        let y_start = if y == 0 { 0 } else { y - 1 };
        for x in x_start..=x_end {
            for y in y_start..=y_end {
                let tmp_idx = y * 10 + x;
                if tmp_idx == idx { continue; };
                // Check if the generated index is missing from the miss list
                if self.misses_shots.iter().find(|&&idx| idx == (tmp_idx)).is_none() {
                    if self.fire_shots.iter().find(|&&idx| idx == (tmp_idx)).is_none() {
                        variant_shot.push(tmp_idx);
                    }
                }
            }
        }
        variant_shot
    }
}

