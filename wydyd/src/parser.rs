use env::Var;
use parser_rs::Parser;

// TODO
pub fn parse_vars(content: String) -> Vec<Var> {
    let mut parser = Parser::new(content);
    let mut vars = Vec::new();
    while !parser.is_eof() {
        parser.consume_whitespace();
        let name = parser.consume_while(|c| !c.is_whitespace() && c != '=');
        parser.consume_whitespace();
        if parser.consume() != Some('=') {
            error!("Expected = in the vars file");
            ::std::process::exit(1);
        }
        parser.consume_whitespace();
        let value = if parser.next() == Some('\"') {
            parser.consume_inside('\"')
        } else {
            parser.consume_while(|c| !c.is_whitespace())
        };
        let var = Var::new(name, value);
        debug!("var: {:?}", var);
        vars.push(var);
        parser.consume_whitespace();
    }
    vars
}
