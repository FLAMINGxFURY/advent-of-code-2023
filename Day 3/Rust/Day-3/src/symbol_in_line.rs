pub struct SymbolInLine {
    symbol: char,
    line: usize,
    pos: usize,
}

impl SymbolInLine {
    pub fn new(symbol: char, line: usize, pos: usize) -> SymbolInLine {
        SymbolInLine { symbol, line, pos }
    }

    pub fn to_string(&self) -> String {
        format!("SymbolInLine({}, {}, {})", self.symbol, self.line, self.pos)
    }
}