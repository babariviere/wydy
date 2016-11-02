use env::Var;
use parser_rs::Parser;

/// Parse all vars and return the list of parsed vars.
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

/// Defined keyword for WCommand
#[derive(Debug)]
pub enum WKeyword {
    Add,
    Edit,
    Delete,
    List,
    Search,
    Open,
    Run,
    None,
}

impl<'a> From<&'a str> for WKeyword {
    fn from(s: &'a str) -> WKeyword {
        let s = s.to_lowercase();
        match s.as_str() {
            "add" => WKeyword::Add,
            "edit" => WKeyword::Edit,
            "delete" => WKeyword::Delete,
            "list" => WKeyword::List,
            "search" => WKeyword::Search,
            "open" => WKeyword::Open,
            "run" => WKeyword::Run,
            _ => WKeyword::None,
        }
    }
}

impl From<String> for WKeyword {
    fn from(s: String) -> WKeyword {
        WKeyword::from(s.as_str())
    }
}

/// this define the wydy command parse result
pub type WCPResult = (WKeyword, String);

/// Parse a command in String format and return a command parse result.
/// It contains keyword and a string.
pub fn parse_command_str(command: String) -> WCPResult {
    let mut parser = Parser::new(command);
    parser.consume_whitespace();
    let word = parser.consume_while(|c| !c.is_whitespace());
    let keyword = WKeyword::from(word.as_ref());
    let mut content = match keyword {
        WKeyword::None => word,
        _ => String::new(),
    };
    content.push_str(&parser.consume_until_end());
    let content = content.trim().to_string();
    (keyword, content)
}
