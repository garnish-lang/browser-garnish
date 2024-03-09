use crate::context::BrowserContext;
use crate::script::SourceDetails;
use garnish_lang::compiler::build::build_with_data;
use garnish_lang::compiler::lex::{lex, LexerToken, TokenType};
use garnish_lang::compiler::parse::parse;
use garnish_lang::simple::SimpleGarnishData;
use garnish_lang::GarnishData;
use garnish_lang_annotations_collector::{Collector, PartBehavior, PartParser, Sink, TokenBlock};

pub fn compile_source_into_data(
    source: &SourceDetails,
    data: &mut SimpleGarnishData,
    context: &mut BrowserContext,
) -> Result<(), String> {
    let tokens = match lex(source.text()) {
        Ok(tokens) => tokens,
        Err(e) => {
            return Err(e.get_message().clone());
        }
    };

    compile_tokens_into_data(&tokens, source.name(), data, context)
}

fn compile_tokens_into_data(
    tokens: &Vec<LexerToken>,
    name: &String,
    data: &mut SimpleGarnishData,
    context: &mut BrowserContext,
) -> Result<(), String> {
    let collector = Collector::new(vec![Sink::new("@Def")
        .part(PartParser::new(PartBehavior::TokenCount(1)))
        .part(PartParser::new(PartBehavior::UntilToken(
            TokenType::EndExpression,
        )))]);

    let collection = collector.collect_tokens(&tokens)?;
    let (root_blocks, def_blocks): (Vec<TokenBlock>, Vec<TokenBlock>) = collection
        .into_iter()
        .partition(|block| block.annotation_text().is_empty());

    let root_tokens: Vec<LexerToken> = root_blocks
        .into_iter()
        .flat_map(|block| block.tokens_owned())
        .collect();

    let parse_result = parse(&root_tokens).or_else(|e| Err(e.get_message().clone()))?;

    let root_point = data.get_jump_table_len();
    context.add_expression_mapping(&name, root_point);

    build_with_data(
        parse_result.get_root(),
        parse_result.get_nodes().clone(),
        data,
    ).or_else(|e| Err(e.get_message().clone()))?;

    for def in def_blocks {
        let name_part = def
            .parts()
            .get(0)
            .ok_or("No name part found for @Def annotation")?;
        let identifier = name_part
            .iter()
            .find(|t| t.get_token_type() == TokenType::Identifier)
            .ok_or("Expected identifier for @Def name")?;
        let expression_part = def
            .parts()
            .get(1)
            .ok_or("No expression found for @Def annotation")?;
        let (start, _) = expression_part
            .iter()
            .enumerate()
            .find(|(_, token)| token.get_token_type() == TokenType::StartExpression)
            .ok_or("Expected expression after identifier for @Def annotation")?;
        let (end, _) = expression_part
            .iter()
            .enumerate()
            .rev()
            .find(|(_, token)| token.get_token_type() == TokenType::EndExpression)
            .ok_or("Expected expression after identifier for @Def annotation")?;

        compile_tokens_into_data(&Vec::from(&expression_part[(start + 1)..end]), identifier.get_text(), data, context)?;
    }

    for value in data.get_data().symbol_to_name().values() {
        context.add_symbol_name(value);
    }

    Ok(())
}
