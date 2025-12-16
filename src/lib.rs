use anyhow::{Context, Result};
use regex::Regex;
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

/// Represents a processed context file
#[derive(Debug)]
struct ContextFile {
    /// Relative path from the .context folder
    path: String,

    /// Processed content with folds replaced
    content: String,
    
    /// Whether this file contains fold tags
    has_folds: bool,
}

/// Process fold tags in content, replacing them with placeholders
fn process_folds(content: &str, file_path: &str) -> (String, bool) {
    let fold_regex = Regex::new(r"(?s)<ctxgen:fold>(.*?)</ctxgen:fold>").unwrap();
    
    let mut has_folds = false;
    let mut result = String::new();
    let mut last_end = 0;
    
    for cap in fold_regex.captures_iter(content) {
        has_folds = true;
        let full_match = cap.get(0).unwrap();
        let fold_content = cap.get(1).unwrap().as_str();
        
        // Add content before this fold
        result.push_str(&content[last_end..full_match.start()]);
        
        // Count lines in the folded content
        let line_count = fold_content.lines().count();
        let line_text = if line_count == 1 { "line" } else { "lines" };
        
        // Calculate start and end line numbers (counting only content, not the fold tags)
        // Find where the actual fold content starts (after the opening tag)
        let fold_content_start = cap.get(1).unwrap().start();
        let fold_content_end = cap.get(1).unwrap().end();
        
        let start_line = content[..fold_content_start].lines().count() + 1;
        let end_line = content[..fold_content_end].lines().count();
        
        // Add placeholder
        let placeholder = format!(
            "[Folded content: {} {} (lines {}-{}). Read '{}' for full content.]",
            line_count, line_text, start_line, end_line, file_path
        );
        result.push_str(&placeholder);
        
        last_end = full_match.end();
    }
    
    // Add remaining content after last fold
    result.push_str(&content[last_end..]);
    
    (result, has_folds)
}

/// Generate file tag with appropriate attributes
fn format_file_entry(file: &ContextFile) -> String {
    let has_folds_attr = if file.has_folds {
        " has_folds=\"true\""
    } else {
        ""
    };
    
    format!(
        "<file path=\"{}\"{}>\n{}\n</file>",
        file.path,
        has_folds_attr,
        file.content.trim()
    )
}

/// Collect and process all context files from the given directory
pub fn generate_context_markdown(context_dir: &Path) -> Result<String> {
    let mut context_files: Vec<ContextFile> = Vec::new();
    
    // Walk the directory tree
    for entry in WalkDir::new(context_dir)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        
        // Skip directories
        if path.is_dir() {
            continue;
        }
        
        // Read file content
        let content = fs::read_to_string(path)
            .with_context(|| format!("Failed to read file: {}", path.display()))?;
        
        // Get relative path from context directory
        let relative_path = path
            .strip_prefix(context_dir)
            .unwrap_or(path)
            .to_string_lossy()
            .to_string();
        
        // Process folds
        let (processed_content, has_folds) = process_folds(&content, &relative_path);
        
        context_files.push(ContextFile {
            path: relative_path,
            content: processed_content,
            has_folds,
        });
    }
    
    // Sort files by path for consistent output
    context_files.sort_by(|a, b| a.path.cmp(&b.path));
    
    // Generate markdown content
    let file_entries: Vec<String> = context_files
        .iter()
        .map(format_file_entry)
        .collect();
    
    Ok(file_entries.join("\n\n"))
}

/// Write the generated content to AGENTS.md and CLAUDE.md
pub fn write_output_files(output_dir: &Path, content: &str) -> Result<()> {
    // Ensure output directory exists
    fs::create_dir_all(output_dir)
        .with_context(|| format!("Failed to create output directory: {}", output_dir.display()))?;
    
    // Write AGENTS.md
    let agents_path = output_dir.join("AGENTS.md");
    fs::write(&agents_path, content)
        .with_context(|| format!("Failed to write {}", agents_path.display()))?;
    
    // Write CLAUDE.md
    let claude_path = output_dir.join("CLAUDE.md");
    fs::write(&claude_path, content)
        .with_context(|| format!("Failed to write {}", claude_path.display()))?;
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_process_folds_no_folds() {
        let content = "This is regular content\nwith no folds.";
        let (result, has_folds) = process_folds(content, "test.txt");
        assert_eq!(result, content);
        assert!(!has_folds);
    }
    
    #[test]
    fn test_process_folds_single_fold() {
        let content = "Before\n<ctxgen:fold>Hidden content</ctxgen:fold>\nAfter";
        let (result, has_folds) = process_folds(content, "test.txt");
        assert!(has_folds);
        assert!(result.contains("[Folded content:"));
        assert!(result.contains("Read 'test.txt'"));
        assert!(!result.contains("Hidden content"));
    }
    
    #[test]
    fn test_process_folds_multiple_folds() {
        let content = "Start\n<ctxgen:fold>First</ctxgen:fold>\nMiddle\n<ctxgen:fold>Second</ctxgen:fold>\nEnd";
        let (result, has_folds) = process_folds(content, "test.txt");
        assert!(has_folds);
        assert!(result.contains("Start"));
        assert!(result.contains("Middle"));
        assert!(result.contains("End"));
        assert!(!result.contains("First"));
        assert!(!result.contains("Second"));
    }
    
    #[test]
    fn test_format_file_entry_with_folds() {
        let file = ContextFile {
            path: "example.txt".to_string(),
            content: "Some content".to_string(),
            has_folds: true,
        };
        let result = format_file_entry(&file);
        assert!(result.contains("has_folds=\"true\""));
        assert!(result.contains("path=\"example.txt\""));
    }
    
    #[test]
    fn test_format_file_entry_without_folds() {
        let file = ContextFile {
            path: "example.txt".to_string(),
            content: "Some content".to_string(),
            has_folds: false,
        };
        let result = format_file_entry(&file);
        assert!(!result.contains("has_folds"));
        assert!(result.contains("path=\"example.txt\""));
    }
}

