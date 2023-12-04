pub struct NumInLine {
    num: i32,
    line: usize,
    start_pos: usize,
    end_pos: usize,
}

impl NumInLine {
    pub fn new(num: i32, line: usize, start_pos: usize, end_pos: usize) -> NumInLine {
        NumInLine { num, line, start_pos, end_pos }
    }

    pub fn to_string(&self) -> String {
        format!("NumInLine({}, {}, {}, {})", self.num, self.line, self.start_pos, self.end_pos)
    }
}