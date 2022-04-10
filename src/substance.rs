use rand::Rng;

#[derive(Clone, PartialEq, Debug)]
pub enum SubstanceKind {
    AIR,
    STONE,
    SAND,
    WATER,
}

#[derive(Clone, Debug)]
pub struct Substance {
    pub kind: SubstanceKind,
    pub color: [u8; 4],
    pub is_stationary: bool,
    pub is_updatable: bool,
}

impl Substance {
    pub fn air() -> Self {
        Self {
            kind: SubstanceKind::AIR,
            color: [0x0, 0x0, 0x0, 0xff],
            is_stationary: false,
            is_updatable: false,
        }
    }

    pub fn stone() -> Self {
        Self {
            kind: SubstanceKind::STONE,
            color: [0x7f, 0x7f, 0x7f, 0xff],
            is_stationary: true,
            is_updatable: false,
        }
    }

    pub fn sand() -> Self {
        Self {
            kind: SubstanceKind::SAND,
            color: [0x5e, 0x48, 0x00, 0xff],
            is_stationary: true,
            is_updatable: true,
        }
    }

    pub fn water() -> Self {
        Self {
            kind: SubstanceKind::WATER,
            color: [0x5e, 0x48, 0xe8, 0xff],
            is_stationary: true,
            is_updatable: true,
        }
    }
}
