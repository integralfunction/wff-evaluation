#[derive(Debug, Clone, Copy)]
pub struct Term {
    pub symbol: char,
    pub value: bool,
}

impl Term {
    pub fn set_value(&mut self, new_value: bool) {
        self.value = new_value
    }
}
