pub struct Modifier {
    pub id: u32,
    pub properties: String,
}

pub struct Cell {
    pub id: u32,
    pub mod_stack: Vec<Modifier>,
}

pub struct CellStack {
    pub id: u32,
    pub stack: Vec<Cell>,
}
