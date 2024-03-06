use std::collections::HashMap;
use garnish_lang::{GarnishContext, GarnishData, RuntimeError};
use garnish_lang::simple::{DataError, NoCustom, SimpleData, SimpleGarnishData, SimpleNumber, symbol_value};
use garnish_lang_utilities::DataInfoProvider;

pub struct BrowserContext {
    symbol_to_expression: HashMap<u64, usize>,
    symbol_to_data: HashMap<u64, SimpleData>
}

impl BrowserContext {
    pub fn new() -> Self {
        let mut symbol_to_data = HashMap::new();
        symbol_to_data.insert(symbol_value("Math::PI"), SimpleData::Number(SimpleNumber::Float(std::f64::consts::PI)));
        symbol_to_data.insert(symbol_value("Math::IntegerMax"), SimpleData::Number(SimpleNumber::Integer(i32::MAX)));
        symbol_to_data.insert(symbol_value("Math::IntegerMin"), SimpleData::Number(SimpleNumber::Integer(i32::MIN)));
        symbol_to_data.insert(symbol_value("Math::FloatMax"), SimpleData::Number(SimpleNumber::Float(f64::MAX)));
        symbol_to_data.insert(symbol_value("Math::FloatMin"), SimpleData::Number(SimpleNumber::Float(f64::MIN)));

        BrowserContext {
            symbol_to_expression: HashMap::new(),
            symbol_to_data
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
            None => match self.symbol_to_data.get(&symbol) {
                Some(v) => match v {
                    SimpleData::Number(n) => {
                        data.add_number(*n).and_then(|addr| data.push_register(addr))?;
                        Ok(true)
                    },
                    _ => Ok(false)
                }
                None => Ok(false)
            }
        }
    }
}

impl DataInfoProvider<SimpleGarnishData> for BrowserContext {
    
}