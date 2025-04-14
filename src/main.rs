mod parser;
mod test;
mod tokenizer;
use std::time::Instant;

fn main() -> Result<(), ()> {
    let test_string = "This is a regular text paragraph to start with.
**_This is bold and underlined text_** using nested combinations.
what the hell *_Here we have italic and underlined text_* using another combination.
# **This centered text is also bold** which looks impressive.
# *Here we have centered and italic text* for variety.
# _This centered text is underlined_ for emphasis.
*This is just italic* and _this is just underlined_.
**This is just bold** text on its own.
# This is just centered without other formatting.
**_This is bold and underlined text
somethine else newline lol_** using nested combinations.
what the hell *_Here we have italic and underlined text_* using another combination.
***This text is both bold and italic*** using triple asterisks.
# **This centered text is also bold** which looks impressive.
-this is a bullet point
--this is a sub bullet point
-this is non unicode *é*
This paragraph has no special formatting at all.";
    let start = Instant::now();
    let mut tokenizer = tokenizer::Tokenizer::new(test_string);
    let tokens = tokenizer.tokenize().expect("error in tokenization");
    let parser = parser::Parser::new(&tokens);
    let paragraphs: Vec<parser::Paragraph> = parser.parse();
    let duration = start.elapsed();
    println!("processed {} bytes in {:?}", test_string.len(), duration,);

    test::makedoc(&paragraphs).unwrap();
    Ok(())
}
