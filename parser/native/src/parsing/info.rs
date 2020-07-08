use serde::Serialize;

#[derive(Serialize)]
pub struct Info {
    length: usize,
    atoms: Vec<Box<dyn erased_serde::Serialize>>,
}

impl Info {
    pub fn new(length: usize, atoms: Vec<Box<dyn erased_serde::Serialize>>) -> Self {
        Info { length, atoms }
    }
}
