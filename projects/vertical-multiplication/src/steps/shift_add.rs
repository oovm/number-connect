use super::*;

impl Display for ShiftAdd {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let tail = ".".repeat(self.tail_digits);
        write!(f, "{}{}", self.result, tail)
    }
}

impl ShiftAdd {
    pub fn new<R>(result: R, tail: usize) -> ShiftAdd
    where
        R: Into<BigInt>,
    {
        ShiftAdd { result: result.into(), tail_digits: tail }
    }

    pub fn as_integer(&self) -> BigInt {
        BigInt::from(self.tailing_power()).mul(&self.result)
    }
    pub fn tailing_power(&self) -> BigInt {
        BigInt::from(10).pow(self.tail_digits as u32)
    }
    pub fn count_digits(&self) -> usize {
        self.result.to_string().len() + self.tail_digits
    }
    pub fn pretty_format(&self, width: usize, leading: &str) -> String {
        // let mut result = leading.to_string();
        let space_width = width - self.count_digits();
        format!("{}{}{}", leading, " ".repeat(space_width), self)
    }
}
