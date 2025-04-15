mod docprocessor;
mod parser;
mod test;
mod tokenizer;
use std::time::Instant;

fn main() -> Result<(), ()> {
    let test_string = "# _Test Document for ***LMATdoc***_

Welcome to our comprehensive documentation on advanced text formatting techniques for modern document creation systems.

This initial paragraph provides context and serves as a regular block of text without any special formatting applied. It's important to establish baseline typography before demonstrating various formatting options that can enhance readability and visual hierarchy within professional documents.

**_The combination of bold and underlined text_** creates strong emphasis that draws the reader's attention immediately to critical information. This technique should be used sparingly for maximum effect in executive summaries or when highlighting key conclusions within extensive reports.

*_Italic with underlining creates a more subtle emphasis_* that works exceptionally well for definitions, foreign phrases, or specialized terminology that requires visual distinction without the boldness that might overpower surrounding content in academic or technical documentation.

# **This centered and bold heading establishes a clear section break** which helps organize longer documents into logical components that readers can easily navigate.

# *Centered italic text provides an elegant stylistic alternative* for quotations, epigraphs, or creative elements within more formal business communications.

# _This centered and underlined text establishes hierarchy_ for subsections or important callouts that need prominence without the visual weight of bold formatting.

*Simple italic formatting* remains one of the most versatile tools for providing subtle emphasis or indicating publication titles, scientific names, or technical terms within flowing text passages.

_Underlined text without additional formatting_ continues to serve as a reliable method for highlighting hyperlinks or drawing attention to specific phrases that warrant further investigation by the reader.

**Bold text stands alone** as the clearest way to create visual emphasis for key points, summaries, or action items within procedural documentation or instructional materials.

# Centered text creates visual interest and breaks the monotony of left-aligned paragraphs when introducing new topics or presenting important quotes.

**_The strategic combination of bold and underlined text can extend across multiple lines when presenting complex ideas that benefit from heightened visual distinction within dense information._** This technique works particularly well for mission-critical warnings or procedural steps that cannot be overlooked.

Professional documentation often requires *_special formatting for technical terminology or industry-specific concepts_* that might otherwise blend into surrounding content without proper visual distinction.

***The triple-asterisk approach creates text that is simultaneously bold and italic*** providing maximum emphasis for warnings, critical alerts, or absolutely essential information that demands immediate reader attention regardless of surrounding content.

# **Centered bold headings create natural pauses in document flow** giving readers visual cues about content organization and hierarchical relationships between different sections of complex information.

~This primary bullet point introduces a fundamental concept or initial step in a procedural sequence.

~~This subordinate bullet point provides additional detail or clarification about the primary point immediately preceding it.

~This bullet point demonstrates non-ASCII character handling with special characters such as *é* which is essential for multilingual documentation.

The conclusion returns to standard formatting, allowing the reader's eye to rest after processing the various visual treatments demonstrated throughout the document. This approach reinforces the principle that special formatting should be used strategically rather than abundantly to maintain readability and professional appearance.";
    let start = Instant::now();

    let mut doc = docprocessor::DocProcessor::new(test_string); // moves ownershup to doc
    doc.process()?;

    let duration = start.elapsed();
    println!("processed {} bytes in {:?}", test_string.len(), duration,);

    test::makedoc(doc.tree()).unwrap();
    Ok(())
}
