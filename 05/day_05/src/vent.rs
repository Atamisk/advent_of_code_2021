pub struct Vent{
    start: (usize, usize),
    end: (usize, usize),
    slope: Slope
}

impl Vent{
    pub fn new(
        start: (usize, usize), 
        end: (usize, usize)
    ) -> Self {
        let slope = slope_between_points(start, end);
        Self {
            start, 
            end,
            slope
        }
    }

    pub fn contains(&self, pt: (usize, usize)) -> bool {
        let (x_min, x_max) = {
            if self.start.0 < self.end.0 {
                (self.start.0, self.end.0)
            }
            else {
                (self.end.0, self.start.0)
            }
        };
        let (y_min, y_max) = {
            if self.start.1 < self.end.1 {
                (self.start.1, self.end.1)
            }
            else {
                (self.end.1, self.start.1)
            }
        };

        if pt.0 >= x_min && pt.0 <= x_max
            && pt.1 >= y_min && pt.1 <= y_max 
        {
            let point_slope = slope_between_points(self.start, pt);
            self.slope == point_slope || point_slope == Slope::SamePoint
        }
        else {
            false
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
pub enum Slope{
    HorizPos,
    HorizNeg, 
    VertPos, 
    VertNeg, 
    SamePoint,
    Oblique(isize)
}

fn slope_between_points(
    start: (usize, usize), 
    end: (usize, usize)
) -> Slope {
    let delta_x = end.0 as isize - start.0 as isize;
    let delta_y = end.1 as isize - start.1 as isize;

    if delta_x == 0 {
        if delta_y == 0 {
            Slope::SamePoint
        }
        else if end.1 > start.1 {
            Slope::VertPos
        }
        else {
            Slope::VertNeg
        }
    }
    else if delta_y == 0 {
        if end.0 > start.0 {
            Slope::HorizPos
        }
        else {
            Slope::HorizNeg
        }
    }
    else {
        Slope::Oblique(delta_y / delta_x)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_contains() {
        let vent = Vent::new((10,0), (0,0));
        assert!(vent.contains((5,0)));
        assert!(!(vent.contains((11,0))));
        assert!((vent.contains((10,0))));
        assert!((vent.contains((0,0))));
        assert!(!(vent.contains((5,1))));
        assert!(!(vent.contains((11,0))));
        assert!(!(vent.contains((11,0))));
    }
}