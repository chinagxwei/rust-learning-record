trait Peg {
    fn get_radius(&self) -> f64;
}

struct RoundHole {
    radius: f64
}

struct RoundPeg {
    radius: f64
}

struct SquarePeg {
    width: f64
}

struct SquarePegAdapter {
    peg: SquarePeg
}

impl RoundHole {
    fn new(radius: f64) -> RoundHole {
        RoundHole { radius }
    }
}

impl RoundPeg {
    fn new(radius: f64) -> RoundPeg {
        RoundPeg { radius }
    }
}

impl SquarePeg {
    fn new(width: f64) -> SquarePeg {
        SquarePeg { width }
    }
}

impl SquarePegAdapter {
    fn new(peg: SquarePeg) -> SquarePegAdapter {
        SquarePegAdapter { peg }
    }
}

impl RoundHole {
    fn get_radius(&self) -> f64 {
        self.radius
    }

    fn fits(&self, peg: Box<dyn Peg>) -> bool {
        return self.radius >= peg.get_radius();
    }
}

impl Peg for RoundPeg {
    fn get_radius(&self) -> f64 {
        self.radius
    }
}

impl SquarePeg {
    fn get_width(&self) -> f64 {
        self.width
    }

    fn get_square(&self) -> f64 {
        self.width.powi(2)
    }
}

impl Peg for SquarePegAdapter {
    fn get_radius(&self) -> f64 {
        ((self.peg.get_width() / 2.0).powi(2) * 2.0).sqrt()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_adapter() {
        let hole = RoundHole::new(5.0);
        let rpeg = RoundPeg::new(5.0);
        if hole.fits(Box::new(rpeg)) {
            println!("Round peg r5 fits round hole r5.");
        }

        let small_sq_peg = SquarePeg::new(2.0);
        let large_sq_peg = SquarePeg::new(20.0);

        let small_sq_peg_adapter = SquarePegAdapter::new(small_sq_peg);
        let large_sq_peg_adapter = SquarePegAdapter::new(large_sq_peg);

        if hole.fits(Box::new(small_sq_peg_adapter)) {
            println!("Square peg w2 fits round hole r5.")
        }

        if !hole.fits(Box::new(large_sq_peg_adapter)) {
            println!("Square peg w20 does not fit into round hole r5.")
        }
    }
}


