use super::parser::*;
use super::tokenizer::*;

pub struct DocProcessor<'a> {
    input: &'a str,
    tokens: Vec<Token<'a>>,
    tree: Vec<Paragraph<'a>>,

    tokenizer: Tokenizer<'a>,
    parser: Parser<'a>,
}

impl<'a> DocProcessor<'a> {
    pub fn new(input: &'a str) -> Self {
        DocProcessor {
            input,
            tokens: Vec::new(),
            tree: Vec::new(),
            tokenizer: Tokenizer::new(""),
            parser: Parser::new(Vec::new()),
        }
    }

    pub fn process(&mut self) -> Result<(), ()> {
        self.tokenizer = Tokenizer::new(self.input);
        self.tokens = self.tokenizer.tokenize().expect("fatal err in tokenizer");
        self.parser = Parser::new(self.tokens.clone());
        self.tree = self.parser.parse();
        Ok(())
    }

    pub fn tree(&self) -> &Vec<Paragraph<'a>> {
        &self.tree
    }
}
