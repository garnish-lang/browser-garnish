use garnish_lang::compiler::build::build_with_data;
use garnish_lang::compiler::lex::lex;
use garnish_lang::compiler::parse::parse;
use garnish_lang::GarnishData;
use garnish_lang::simple::{SimpleGarnishData, symbol_value};
use crate::context::BrowserContext;
use crate::script::SourceDetails;

pub fn compile_source_into_data(source: &SourceDetails, data: &mut SimpleGarnishData, context: &mut BrowserContext) -> Result<(), String> {
    let tokens = match lex(source.text()) {
        Ok(tokens) => tokens,
        Err(e) => {
            return Err(e.get_message().clone());
        }
    };

    let parse_result = match parse(&tokens) {
        Err(e) => {
            return Err(e.get_message().clone());
        }
        Ok(result) => result,
    };

    let root_point = data.get_jump_table_len();
    context.add_expression_mapping(symbol_value(source.name()), root_point);

    match build_with_data(
        parse_result.get_root(),
        parse_result.get_nodes().clone(),
        data,
    ) {
        Err(e) => {
            return Err(e.get_message().clone());
        }
        Ok(_) => {}
    }

    Ok(())
}