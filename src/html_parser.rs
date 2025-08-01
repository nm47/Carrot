use html5ever::parse_document;
use html5ever::tendril::TendrilSink;
use markup5ever_rcdom::{RcDom, NodeData};
use std::collections::VecDeque;

/// Represents a text node with its parent container information
#[derive(Debug, Clone)]
pub struct TextWithParent {
    pub text: String,
    pub parent_path: String,
    pub parent_classes: Vec<String>,
    pub parent_id: Option<String>,
}

/// HTML parser that extracts text content while preserving parent container information
pub struct HtmlParser;

impl HtmlParser {
    /// Parse HTML and extract all text nodes with their parent container information
    pub fn parse_html_to_text_with_parents(html_content: &str) -> Vec<TextWithParent> {
        let dom = parse_document(RcDom::default(), Default::default())
            .from_utf8()
            .read_from(&mut html_content.as_bytes())
            .unwrap();
        
        let mut results = Vec::new();
        let mut queue = VecDeque::new();
        queue.push_back((dom.document.clone(), Vec::new()));
        
        while let Some((node, path)) = queue.pop_front() {
            match &node.data {
                NodeData::Text { contents } => {
                    let text = contents.borrow().to_string();
                    let trimmed_text = text.trim();
                    
                    // Skip empty text or very short content
                    if !trimmed_text.is_empty() && trimmed_text.len() >= 3 {
                        let parent_path = path.join(" > ");
                        results.push(TextWithParent {
                            text: trimmed_text.to_string(),
                            parent_path,
                            parent_classes: Vec::new(), // Simplified for now
                            parent_id: None, // Simplified for now
                        });
                    }
                }
                NodeData::Element { name, attrs, .. } => {
                    // Build the path for child elements
                    let mut new_path = path.clone();
                    let element_name = name.local.to_string();
                    
                    // Look for class and id attributes
                    let attrs_borrowed = attrs.borrow();
                    let mut element_identifier = element_name.clone();
                    
                    for attr in attrs_borrowed.iter() {
                        match attr.name.local.as_ref() {
                            "id" => {
                                element_identifier.push_str(&format!("#{}", attr.value));
                            }
                            "class" => {
                                let classes: Vec<&str> = attr.value.split_whitespace().collect();
                                if let Some(first_class) = classes.first() {
                                    element_identifier.push_str(&format!(".{}", first_class));
                                }
                            }
                            _ => {}
                        }
                    }
                    
                    new_path.push(element_identifier);
                    
                    // Add children to the queue
                    for child in node.children.borrow().iter() {
                        queue.push_back((child.clone(), new_path.clone()));
                    }
                }
                _ => {
                    // For other node types, continue with children
                    for child in node.children.borrow().iter() {
                        queue.push_back((child.clone(), path.clone()));
                    }
                }
            }
        }
        
        results
    }
    
    
    /// Convert HTML to plain text for backwards compatibility with existing code
    pub fn html_to_text(html_content: &str) -> String {
        let text_chunks = Self::parse_html_to_text_with_parents(html_content);
        text_chunks
            .into_iter()
            .map(|chunk| chunk.text)
            .collect::<Vec<_>>()
            .join("\n")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_html_parsing() {
        let html = r#"
        <html>
            <body>
                <div class="recipe">
                    <h2>Test Recipe</h2>
                    <ul class="ingredients">
                        <li>2 cups flour</li>
                        <li>1 tsp salt</li>
                    </ul>
                    <ol class="instructions">
                        <li>Mix flour and salt</li>
                        <li>Add water gradually</li>
                    </ol>
                </div>
            </body>
        </html>
        "#;
        
        let results = HtmlParser::parse_html_to_text_with_parents(html);
        
        // Should find the ingredient and instruction texts
        assert!(results.iter().any(|r| r.text.contains("2 cups flour")));
        assert!(results.iter().any(|r| r.text.contains("Mix flour and salt")));
        
        // Should have parent path information
        let flour_entry = results.iter().find(|r| r.text.contains("2 cups flour")).unwrap();
        assert!(flour_entry.parent_path.contains("ul") || 
                flour_entry.parent_path.contains("ingredients"));
    }
    
    #[test]
    fn test_preserves_all_text() {
        let html = r#"
        <div>
            <p>2 cups flour and 1 tsp salt</p>
            <ul>
                <li>2 cups flour</li>
                <li>1 tsp salt</li>
            </ul>
        </div>
        "#;
        
        let results = HtmlParser::parse_html_to_text_with_parents(html);
        
        // Should keep all text entries, even if they contain similar content
        let flour_texts: Vec<_> = results.iter()
            .filter(|r| r.text.contains("flour"))
            .collect();
            
        // Should have multiple entries with flour text
        assert!(flour_texts.len() >= 2);
    }
}