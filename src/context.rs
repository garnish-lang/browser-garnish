use std::collections::HashMap;
use garnish_lang::{GarnishContext, GarnishData, RuntimeError};
use garnish_lang::simple::{DataError, NoCustom, SimpleGarnishData};
use garnish_lang_utilities::DataInfoProvider;

pub struct BrowserContext {
    symbol_to_expression: HashMap<u64, usize>
}

impl BrowserContext {
    pub fn new() -> Self {
        BrowserContext {
            symbol_to_expression: HashMap::new()
        }
    }

    pub fn add_expression_mapping(&mut self, symbol: u64, expression_index: usize) {
        self.symbol_to_expression.insert(symbol, expression_index);
    }
}

impl GarnishContext<SimpleGarnishData<NoCustom>> for BrowserContext {
    fn resolve(&mut self, symbol: u64, data: &mut SimpleGarnishData<NoCustom>) -> Result<bool, RuntimeError<DataError>> {
        match self.symbol_to_expression.get(&symbol) {
            Some(v) => {
                data.add_expression(*v).and_then(|addr| data.push_register(addr))?;
                Ok(true)
            },
            None => Ok(false)
        }
    }
}

impl DataInfoProvider<SimpleGarnishData> for BrowserContext {
    
}