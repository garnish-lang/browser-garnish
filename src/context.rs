use garnish_lang::simple::{symbol_value, DataError, SimpleData, SimpleGarnishData, SimpleNumber};
use garnish_lang::{GarnishContext, GarnishData, RuntimeError};
use garnish_lang_utilities::DataInfoProvider;
use std::collections::HashMap;

pub struct BrowserContext {
    symbol_to_expression: HashMap<u64, usize>,
    symbol_to_data: HashMap<u64, SimpleData>,
    symbol_to_name: HashMap<u64, String>,
}

const MATH_PI_SYMBOL: &str = "Math::PI";
const MATH_INTEGER_MAX_SYMBOL: &str = "Math::IntegerMax";
const MATH_INTEGER_MIN_SYMBOL: &str = "Math::IntegerMin";
const MATH_FLOAT_MAX_SYMBOL: &str = "Math::FloatMax";
const MATH_FLOAT_MIN_SYMBOL: &str = "Math::FloatMin";

impl BrowserContext {
    pub fn new() -> Self {
        let mut context = BrowserContext {
            symbol_to_expression: HashMap::new(),
            symbol_to_name: HashMap::new(),
            symbol_to_data: HashMap::new(),
        };

        context.add_symbol_data(
            MATH_PI_SYMBOL,
            SimpleData::Number(SimpleNumber::Float(std::f64::consts::PI)),
        );
        context.add_symbol_data(
            MATH_INTEGER_MAX_SYMBOL,
            SimpleData::Number(SimpleNumber::Integer(i32::MAX)),
        );
        context.add_symbol_data(
            MATH_INTEGER_MIN_SYMBOL,
            SimpleData::Number(SimpleNumber::Integer(i32::MIN)),
        );
        context.add_symbol_data(
            MATH_FLOAT_MAX_SYMBOL,
            SimpleData::Number(SimpleNumber::Float(f64::MAX)),
        );
        context.add_symbol_data(
            MATH_FLOAT_MIN_SYMBOL,
            SimpleData::Number(SimpleNumber::Float(f64::MIN)),
        );

        context
    }

    pub fn add_symbol_name(&mut self, name: &str) {
        self.symbol_to_name.insert(symbol_value(name), name.to_string());
    }

    pub fn add_symbol_data(&mut self, name: &str, data: SimpleData) {
        let symbol = symbol_value(name);
        self.symbol_to_name.insert(symbol, name.to_string());
        self.symbol_to_data.insert(symbol, data);
    }

    pub fn add_expression_mapping(
        &mut self,
        name: &str,
        expression_index: usize,
    ) {
        let symbol = symbol_value(name);
        self.symbol_to_name.insert(symbol, name.to_string());
        self.symbol_to_expression.insert(symbol, expression_index);
    }
}

impl GarnishContext<SimpleGarnishData> for BrowserContext {
    fn resolve(
        &mut self,
        symbol: u64,
        data: &mut SimpleGarnishData,
    ) -> Result<bool, RuntimeError<DataError>> {
        match self.symbol_to_expression.get(&symbol) {
            Some(v) => {
                data.add_expression(*v)
                    .and_then(|addr| data.push_register(addr))?;
                Ok(true)
            }
            None => match self.symbol_to_data.get(&symbol) {
                Some(v) => match v {
                    SimpleData::Number(n) => {
                        data.add_number(*n)
                            .and_then(|addr| data.push_register(addr))?;
                        Ok(true)
                    }
                    _ => Ok(false),
                },
                None => Ok(false),
            },
        }
    }
}

impl DataInfoProvider<SimpleGarnishData> for BrowserContext {
    fn get_symbol_name(&self, sym: u64, _data: &SimpleGarnishData) -> Option<String> {
        self.symbol_to_name.get(&sym).map(|name| format!(";{}", name))
    }

    fn format_symbol_data(&self, sym: u64, _data: &SimpleGarnishData) -> Option<String> {
        self.symbol_to_name.get(&sym).map(|name| format!(";{}", name))
    }
}
