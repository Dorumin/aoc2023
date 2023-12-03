use std::borrow::Borrow;

pub struct SurroundScanner<'a> {
    pub lines: Vec<&'a str>
}

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub struct Block {
    pub x: usize,
    pub y: usize,
    pub width: usize,
    pub height: usize
}

impl Block {
    pub fn contains(&self, x: usize, y: usize) -> bool {
        x >= self.x && y >= self.y && x < self.x + self.width && y < self.y + self.height
    }
}

impl<'a> SurroundScanner<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            lines: input.lines().collect()
        }
    }

    fn get_offset(&self, line_index: Option<usize>, char_index: Option<usize>) -> Option<(usize, usize, char)> {
        let line = self.lines.iter().nth(line_index?)?;
        let char = line.chars().nth(char_index?)?;

        Some((line_index?, char_index?, char))
    }

    pub fn surrounding(&self, block: impl Borrow<Block>) -> Vec<(usize, usize, char)> {
        let mut chars = vec![];

        let block: &Block = block.borrow();

        // Top-left corner
        self.get_offset(block.y.checked_sub(1), block.x.checked_sub(1)).map(|c| chars.push(c));
        // Top row
        for i in 0..block.width {
            self.get_offset(block.y.checked_sub(1), block.x.checked_add(i)).map(|c| chars.push(c));
        }
        // Top-right corner
        self.get_offset(block.y.checked_sub(1), block.x.checked_add(block.width)).map(|c| chars.push(c));

        // Left and right columns
        for i in 0..block.height {
            self.get_offset(block.y.checked_add(i), block.x.checked_sub(1)).map(|c| chars.push(c));
            self.get_offset(block.y.checked_add(i), block.x.checked_add(block.width)).map(|c| chars.push(c));
        }

        // Bottom-left corner
        self.get_offset(block.y.checked_add(1), block.x.checked_sub(1)).map(|c| chars.push(c));
        // Bottom row
        for i in 0..block.width {
            self.get_offset(block.y.checked_add(1), block.x.checked_add(i)).map(|c| chars.push(c));
        }
        // Bottom-right corner
        self.get_offset(block.y.checked_add(1), block.x.checked_add(block.width)).map(|c| chars.push(c));

        chars
    }
}