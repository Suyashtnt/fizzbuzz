use std::{convert::TryInto, i128};

pub struct Generator {
    pub current: i128,
    pub max: i128,
    pub match_against: Vec<(String, i128)>,
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
        (
            self.max.try_into().unwrap(),
            Some(self.max.try_into().unwrap()),
        )
    }
}
