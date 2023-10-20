pub struct NBody {
    massive_positions: Vec<[f32; 3]>,
    massive_masses: Vec<f32>,
}

impl NBody {
    pub fn new() -> Self {
        Self {
            massive_positions: Vec::new(),
            massive_masses: Vec::new(),
        }
    }

    pub fn massive_positions(&self) -> &[[f32; 3]] {
        &self.massive_positions
    }

    pub fn massive_masses(&self) -> &[f32] {
        &self.massive_masses
    }

    pub fn num_massive_particles(&self) -> usize {
        self.massive_positions.len()
    }

    pub fn add_massive_particle(&mut self, position: [f32; 3]) {
        self.massive_positions.push(position);
        self.massive_masses.push(1.0);
    }
}