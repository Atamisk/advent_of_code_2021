use std::cmp;

#[derive(Clone)]
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

    pub fn get_points(&self) -> Vec<(usize, usize)>{
        let target_length = cmp::max( (self.start.0 as isize - self.end.0 as isize).abs(),
                                 (self.start.1 as isize - self.end.1 as isize).abs()) as usize + 1;
        let x_rng: Vec<usize> = {
            if self.start.0 < self.end.0 {
                (self.start.0..=self.end.0).collect()
            }
            else if self.start.0 > self.end.0 {
                (self.end.0..=self.start.0).rev().collect()
            }
            else{
                vec![self.start.0; target_length]
            }
        };
        let y_rng: Vec<usize> = {
            if self.start.1 < self.end.1 {
                (self.start.1..=self.end.1).collect()
            }
            else if self.start.1 > self.end.1{
                (self.end.1..=self.start.1).rev().collect()
            }
            else{
                vec![self.start.1; target_length]
            }
        };
        assert_eq!(x_rng.len(), target_length);
        assert_eq!(y_rng.len(), target_length);
        x_rng.into_iter().zip(y_rng.into_iter()).collect()
    }

    pub fn max_values(&self) -> (usize, usize) {
        let max_x = cmp::max(self.start.0, self.end.0);
        let max_y = cmp::max(self.start.1, self.end.1);
        (max_x, max_y)
    }
    
    pub fn is_ortho(&self) -> bool {
        self.start.0 == self.end.0 || self.start.1 == self.end.1
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

#[derive(PartialEq, Eq, Debug, Clone)]
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
    #[test]
    fn test_get_points(){
        let vent = Vent::new((10,0), (0,0));
        let mut line = vent.get_points().into_iter();
        assert_eq!(line.next(), Some((10,0)));
        assert_eq!(line.next(), Some((9,0)));
        assert_eq!(line.next(), Some((8,0)));
        assert_eq!(line.next(), Some((7,0)));
        assert_eq!(line.next(), Some((6,0)));
        assert_eq!(line.next(), Some((5,0)));
        assert_eq!(line.next(), Some((4,0)));
        assert_eq!(line.next(), Some((3,0)));
        assert_eq!(line.next(), Some((2,0)));
        assert_eq!(line.next(), Some((1,0)));
        assert_eq!(line.next(), Some((0,0)));

    }
}
