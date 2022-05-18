use std::fmt::Debug;

pub trait Gear: Debug {
    fn turn(&self, rot: f64, n: u32) -> bool;
}

fn self_rot(rot: f64, n: u32, sn: u32) -> f64 {
    rot * (n as f64 / sn as f64) * -1.0
}

#[derive(Debug)]
pub struct NGear {
    pub n: u32,
    pub parrarel: Vec<Box<dyn Gear>>,
    pub child: Option<Box<dyn Gear>>,
}

impl Gear for NGear {
    fn turn(&self, rot: f64, n: u32) -> bool {
        let self_rot = self_rot(rot, n, self.n);

        let mut acc_b = true;
        for p in &self.parrarel {
            acc_b &= p.turn(-self_rot, self.n);
        }

        match &self.child {
            Some(c) => acc_b & c.turn(self_rot, self.n),
            None => false,
        }
    }
}

#[derive(Debug)]
pub struct CGear {
    pub n: u32,
    pub label: Option<String>,
    pub symbols: Vec<String>,
    pub child: Option<Box<dyn Gear>>,
}

impl Gear for CGear {
    fn turn(&self, rot: f64, n: u32) -> bool {
        let self_rot = self_rot(rot, n, self.n);

        let norm_rot = if self_rot > 0.0 {
            self_rot % 1.0
        } else {
            self_rot % 1.0 + 1.0
        };

        let symbol_index =
            (norm_rot * self.symbols.len() as f64 % self.symbols.len() as f64) as usize;

        print!(
            "{}{}",
            match &self.label {
                Some(l) => format!("{}: ", l),
                None => String::new(),
            },
            self.symbols[symbol_index]
        );

        match &self.child {
            Some(c) => c.turn(self_rot, self.n),
            None => true,
        }
    }
}

#[derive(Debug)]
pub struct EGear {
    pub n: u32,
}

impl Gear for EGear {
    fn turn(&self, rot: f64, n: u32) -> bool {
        let self_rot = self_rot(rot, n, self.n);

        self_rot.abs() <= 1.0
    }
}
