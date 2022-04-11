use std::fmt::Display;

use crate::world::World;

#[derive(PartialEq, Clone)]
enum ElementType {
    Liquid,
    MovableSolid,
    ImmovableSolid,
    Gas,
    Empty,
}

#[derive(Clone)]
enum ElementKind {
    Water,
    Sand,
    Stone,
    Smoke,
    Air,
}

#[derive(Clone)]
pub struct Element {
    stype: ElementType,
    kind: ElementKind,
    pub color: [u8; 4],
}

impl Display for Element {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.kind {
            ElementKind::Water => write!(f, "~"),
            ElementKind::Sand => write!(f, "*"),
            ElementKind::Stone => write!(f, "#"),
            ElementKind::Smoke => write!(f, ","),
            ElementKind::Air => write!(f, "."),
        }
    }
}

impl Element {
    pub fn water() -> Self {
        Self {
            stype: ElementType::Liquid,
            kind: ElementKind::Water,
            color: [0x5e, 0x48, 0xe8, 0xff],
        }
    }

    pub fn smoke() -> Self {
        Self {
            stype: ElementType::Gas,
            kind: ElementKind::Smoke,
            color: [0x7a, 0x7a, 0x7a, 0xff],
        }
    }

    pub fn stone() -> Self {
        Self {
            stype: ElementType::ImmovableSolid,
            kind: ElementKind::Stone,
            color: [0x7f, 0x7f, 0x7f, 0xff],
        }
    }

    pub fn air() -> Self {
        Self {
            stype: ElementType::Empty,
            kind: ElementKind::Air,
            color: [0x0, 0x0, 0x0, 0xff],
        }
    }

    pub fn sand() -> Self {
        Self {
            stype: ElementType::MovableSolid,
            kind: ElementKind::Sand,
            color: [0x5e, 0x48, 0x00, 0xff],
        }
    }

    pub fn step(&self, world: &mut World, index: usize) {
        if self.stype == ElementType::Empty || self.stype == ElementType::ImmovableSolid {
            return;
        }

        match self.kind {
            ElementKind::Sand => {
                self.update_sand(world, index);
            }
            ElementKind::Water => {
                self.update_water(world, index);
            }
            _ => {}
        }
    }

    fn update_sand(&self, world: &mut World, index: usize) -> bool {
        let x = index % world.width;
        let y = index / world.width;

        // check below
        let index_below = world.index(x, y + 1);
        let element_below = world.element(index_below);

        if element_below.stype == ElementType::Empty || element_below.stype == ElementType::Liquid {
            world.switch(index, index_below);
            return true;
        }

        if element_below.stype == ElementType::MovableSolid
            || element_below.stype == ElementType::ImmovableSolid
        {
            // check diagonally (with random preffered side)
            let random: bool = rand::random();
            let factor: i32 = if random { -1 } else { 1 };

            if world.within_bounds(x as i32 - factor) {
                let side = x as i32 - factor;
                let index_left_diagonally = world.index(side as usize, y + 1);
                let left_diagonally = world.element(index_left_diagonally);

                if left_diagonally.stype == ElementType::Empty
                    || left_diagonally.stype == ElementType::Liquid
                {
                    world.switch(index, index_left_diagonally);
                    return true;
                }
            }

            if world.within_bounds(x as i32 + factor) {
                let side = x as i32 + factor;
                let index_right_diagonally = world.index(side as usize, y + 1);
                let right_diagonally = world.element(index_right_diagonally);

                if right_diagonally.stype == ElementType::Empty
                    || right_diagonally.stype == ElementType::Liquid
                {
                    world.switch(index, index_right_diagonally);
                    return true;
                }
            }
        }

        return false;
    }

    fn update_water(&self, world: &mut World, index: usize) -> bool {
        let x = index % world.width;
        let y = index / world.width;

        // check below
        let index_below = world.index(x, y + 1);
        let element_below = world.element(index_below);

        if element_below.stype == ElementType::Empty {
            world.switch(index, index_below);
            return true;
        }

        // check diagonally (with random preffered side)
        let random: bool = rand::random();
        let factor: i32 = if random { -1 } else { 1 };

        if world.within_bounds(x as i32 - factor) {
            let side = x as i32 - factor;
            let index_left_diagonally = world.index(side as usize, y + 1);
            let left_diagonally = world.element(index_left_diagonally);

            if left_diagonally.stype == ElementType::Empty {
                world.switch(index, index_left_diagonally);
                return true;
            }
        }

        if world.within_bounds(x as i32 + factor) {
            let side = x as i32 + factor;
            let index_right_diagonally = world.index(side as usize, y + 1);
            let right_diagonally = world.element(index_right_diagonally);

            if right_diagonally.stype == ElementType::Empty {
                world.switch(index, index_right_diagonally);
                return true;
            }
        }

        // check side (with random preffered side)
        if world.within_bounds(x as i32 - factor) {
            let side = x as i32 - factor;
            let index_left = world.index(side as usize, y);
            let left = world.element(index_left);

            if left.stype == ElementType::Empty {
                world.switch(index, index_left);
                return true;
            }
        }

        if world.within_bounds(x as i32 + factor) {
            let side = x as i32 + factor;
            let index_right = world.index(side as usize, y);
            let right = world.element(index_right);

            if right.stype == ElementType::Empty {
                world.switch(index, index_right);
                return true;
            }
        }

        return false;
    }
}
