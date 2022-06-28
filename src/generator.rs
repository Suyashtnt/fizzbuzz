use std::convert::TryInto;

pub struct Generator {
    pub current: u64,
    pub max: u64,
    pub match_against: Vec<(String, u64)>,
}

impl Iterator for Generator {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if self.max != 0 && self.current == self.max {
            return None;
        };

        self.current += 1;

        let mut output = String::new();

        for (value, num_to_match) in &self.match_against {
            if self.current % num_to_match == 0 {
                output += value;
            }
        }

        if output == "" {
            output = self.current.to_string();
        }

        Some(output)
    }

    fn count(self) -> usize {
        return self.max.try_into().unwrap();
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.max.try_into().unwrap(), self.max.try_into().ok())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_generate_until_10() {
        let mut gen = Generator {
            current: 0,
            max: 10,
            match_against: vec![("Fizz".to_string(), 3), ("Buzz".to_string(), 5)],
        };

        let mut output = String::new();

        for _ in 0..10 {
            output += &gen.next().unwrap();
            output += "\n"
        }

        assert_eq!(output, "1\n2\nFizz\n4\nBuzz\nFizz\n7\n8\nFizz\nBuzz\n");
    }

    #[test]
    fn can_generate_until_15() {
        let mut gen = Generator {
            current: 0,
            max: 15,
            match_against: vec![("Fizz".to_string(), 3), ("Buzz".to_string(), 5)],
        };

        let mut output = String::new();

        for _ in 0..15 {
            output += &gen.next().unwrap();
            output += "\n"
        }

        assert_eq!(
            output,
            "1\n2\nFizz\n4\nBuzz\nFizz\n7\n8\nFizz\nBuzz\n11\nFizz\n13\n14\nFizzBuzz\n"
        );
    }

    #[test]
    fn can_do_fuzz() {
        let mut gen = Generator {
            current: 0,
            max: 15,
            match_against: vec![
                ("Fizz".to_string(), 3),
                ("Buzz".to_string(), 5),
                ("Fuzz".to_string(), 7),
            ],
        };

        let mut output = String::new();

        for _ in 0..15 {
            output += &gen.next().unwrap();
            output += "\n"
        }

        assert_eq!(
            output,
            "1\n2\nFizz\n4\nBuzz\nFizz\nFuzz\n8\nFizz\nBuzz\n11\nFizz\n13\nFuzz\nFizzBuzz\n"
        );
    }
}
