use anyhow::{Context, Error};
pub struct Assignment(u32, u32);

impl Assignment {
    pub fn fully_overlaps(&self, other: &Assignment) -> bool {
        (self.0 <= other.0 && self.1 >= other.1) || (other.0 <= self.0 && other.1 >= self.1)
    }

    pub fn overlaps(&self, other: &Assignment) -> bool {
        self.0 <= other.1 && other.0 <= self.1
    }
}

impl TryFrom<(&str, &str)> for Assignment {
    type Error = Error;

    fn try_from(value: (&str, &str)) -> Result<Self, Self::Error> {
        let low = value.0.parse().context("parsing low str as u32")?;
        let high = value.1.parse().context("parsing high str as u32")?;
        if low > high {
            return Err(Error::msg("low value is greater than high"));
        }
        Ok(Self(low, high))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! assignment {
        ($low:literal, $high:literal) => {
            Assignment::try_from(($low, $high)).unwrap()
        };
    }

    #[test]
    fn fully_overlaps() {
        assert!(assignment!("1", "2").fully_overlaps(&assignment!("1", "1")));
        assert!(assignment!("1", "1").fully_overlaps(&assignment!("1", "2")));
        assert!(assignment!("10", "80").fully_overlaps(&assignment!("11", "65")));
        assert!(assignment!("11", "65").fully_overlaps(&assignment!("10", "80")));
        assert!(assignment!("1", "3").fully_overlaps(&assignment!("1", "3")));
        assert!(assignment!("1", "3").fully_overlaps(&assignment!("2", "3")));

        assert!(!assignment!("1", "2").fully_overlaps(&assignment!("2", "3")));
        assert!(!assignment!("2", "3").fully_overlaps(&assignment!("1", "2")));
        assert!(!assignment!("1", "2").fully_overlaps(&assignment!("3", "4")));
        assert!(!assignment!("3", "4").fully_overlaps(&assignment!("1", "2")));
    }

    #[test]
    fn overlaps() {
        assert!(assignment!("1", "2").overlaps(&assignment!("2", "3")));
        assert!(assignment!("2", "3").overlaps(&assignment!("1", "2")));
        assert!(assignment!("1", "3").overlaps(&assignment!("2", "3")));
        assert!(assignment!("2", "3").overlaps(&assignment!("1", "3")));
        assert!(assignment!("1", "3").overlaps(&assignment!("2", "4")));
        assert!(assignment!("2", "4").overlaps(&assignment!("1", "3")));
        assert!(assignment!("1", "1").overlaps(&assignment!("1", "2")));
        assert!(assignment!("1", "2").overlaps(&assignment!("1", "1")));

        assert!(assignment!("1", "2").overlaps(&assignment!("3", "4")));
        assert!(assignment!("3", "4").overlaps(&assignment!("1", "2")));
    }
}
