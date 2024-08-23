#[derive(Debug)]
#[allow(dead_code)]
pub struct Symbol {
    pub name: String,
    pub offset: u8,
    symbol_type: SymbolType,
}

impl Symbol {
    pub fn new(name: String, offset: u8, symbol_type: SymbolType) -> Symbol {
        Symbol {
            name,
            offset,
            symbol_type,
        }
    }
}

#[derive(Debug)]
pub enum SymbolType {
    Label,
}
