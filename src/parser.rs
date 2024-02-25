use pest::iterators::Pair;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct PureParser;

pub fn is_not_eoi(pair: &Pair<Rule>) -> bool {
    if let Rule::EOI = pair.as_rule() {
        false
    } else {
        true
    }
}

pub fn is_param(pair: &Pair<Rule>) -> bool {
    if let Rule::param = pair.as_rule() {
        true
    } else {
        false
    }
}
