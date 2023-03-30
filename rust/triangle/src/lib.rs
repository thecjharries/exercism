pub struct Triangle([u64; 3]);

impl Triangle {
    pub fn build(sides: [u64; 3]) -> Option<Triangle> {
        for (i, side) in sides.iter().enumerate() {
            if *side == 0 {
                return None;
            }

            for other_side in sides.iter().skip(i + 1) {
                if *side + *other_side <= *side {
                    return None;
                }
            }
        }
        Some(Triangle(sides))
    }

    pub fn is_equilateral(&self) -> bool {
        for side in self.0.iter().skip(1) {
            if *side != self.0[0] {
                return false;
            }
        }
        true
    }

    pub fn is_scalene(&self) -> bool {
        for (i, side) in self.0.iter().enumerate() {
            for other_side in self.0.iter().skip(i + 1) {
                if *side == *other_side {
                    return false;
                }
            }
        }
        true
    }

    pub fn is_isosceles(&self) -> bool {
        for (i, side) in self.0.iter().enumerate() {
            for other_side in self.0.iter().skip(i + 1) {
                if *side == *other_side {
                    return true;
                }
            }
        }
        false
    }
}
