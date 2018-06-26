pub struct Rotor {
    position: u32,
    standard: u32
}

impl Rotor {
    pub fn new(standard: u32, position: u32) -> Self {
        Rotor{
            position,
            standard
        }
    }

    pub fn rotate(&mut self, steps: u32) {
        match self.standard >= self.position {
            true => self.position -= self.standard + steps,
            false => self.position += steps
        };
    }

    pub fn position(&self) -> u32 {
        self.position
    }

    pub fn name(&self) -> bool {
        self.standard - self.position == 0
    }
}