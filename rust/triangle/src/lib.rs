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
        unimplemented!("Determine if the Triangle is equilateral.");
    }

    pub fn is_scalene(&self) -> bool {
        unimplemented!("Determine if the Triangle is scalene.");
    }

    pub fn is_isosceles(&self) -> bool {
        unimplemented!("Determine if the Triangle is isosceles.");
    }
}
