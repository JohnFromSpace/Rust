use std::fs;
use regex::Regex;

fn main() {
    // Read the C++ code from a file
    let code = match fs::read_to_string("your_file.cpp") { 
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            return;
        }
    };
    
    // Validate the code
    if !is_valid_cpp(&code) {
        println!("Validation failed. Aborting analysis.");
        return;
    }

    // Regex patterns for identifying common constructs
    let for_loop_pattern = Regex::new(r#"for\s*\([^;]+;\s*[^;]+;\s*[^)]+\)"#).unwrap();
    let while_loop_pattern = Regex::new(r#"while\s*\([^)]+\)"#).unwrap();
    let if_statement_pattern = Regex::new(r#"if\s*\([^)]+\)"#).unwrap();
    let function_definition_pattern = Regex::new(r#"[\w<>]+\s+\w+\s*\([^)]*\)\s*{\s*"#).unwrap();

    // Regex patterns for identifying common constructs
    let for_loop_pattern = Regex::new(r#"for\s*\([^;]+;\s*[^;]+;\s*[^)]+\)"#).unwrap();
    let while_loop_pattern = Regex::new(r#"while\s*\([^)]+\)"#).unwrap();
    let if_statement_pattern = Regex::new(r#"if\s*\([^)]+\)"#).unwrap();
    let function_definition_pattern = Regex::new(r#"[\w<>]+\s+\w+\s*\([^)]*\)\s*{\s*"#).unwrap();

    // Display results
    println!("Occurrences of for loops: {}", for_loop_count);
    println!("Occurrences of while loops: {}", while_loop_count);
    println!("Occurrences of if statements: {}", if_statement_count);
    println!("Occurrences of function definitions: {}", function_definition_count);
}

// Function to perform additional validations on C++ code
fn is_valid_cpp(code: &str) -> bool {
    // Validation rules
    let no_tabs_or_spaces_at_beginning = code.lines().all(|line| line.trim_start().is_empty());
    let no_unmatched_braces = code.chars().filter(|&c| c == '{').count() == code.chars().filter(|&c| c == '}').count();
    let no_unmatched_parentheses = code.chars().filter(|&c| c == '(').count() == code.chars().filter(|&c| c == ')').count();
    let no_single_line_comments = !code.lines().any(|line| line.trim_start().starts_with("//"));
    let no_multiple_line_comments = !code.contains("/*") && !code.contains("*/");
    let no_empty_lines = code.lines().all(|line| !line.trim().is_empty());
    let no_goto_statements = !code.contains("goto");
    let no_using_namespace_directive = !code.contains("using namespace");
    let no_function_prototypes = code.lines().all(|line| !line.contains(';') || line.contains('(') || line.contains(')'));
    let proper_indentation = code.lines().all(|line| line.starts_with(' '));
    let no_static_globals = !code.contains("static");
    let proper_function_definitions = is_proper_function_definitions(code);
    let proper_variable_naming = is_proper_variable_naming(code);    

    if !no_tabs_or_spaces_at_beginning {
        eprintln!("Validation failed: Remove leading spaces or tabs from the beginning of lines.");
        return false;
    }

    if !no_unmatched_braces {
        eprintln!("Validation failed: Check for unmatched braces.");
        return false;
    }

    if !no_unmatched_parentheses {
        eprintln!("Validation failed: Check for unmatched parentheses.");
        return false;
    }

    if !no_single_line_comments {
        eprintln!("Validation failed: Remove single-line comments (//) from the code.");
        return false;
    }

    if !no_multiple_line_comments {
        eprintln!("Validation failed: Remove multi-line comments (/* */) from the code.");
        return false;
    }

    if !no_empty_lines {
        eprintln!("Validation failed: Remove empty lines from the code.");
        return false;
    }

    if !no_goto_statements {
        eprintln!("Validation failed: Avoid using 'goto' statements.");
        return false;
    }

    if !no_using_namespace_directive {
        eprintln!("Validation failed: Avoid using 'using namespace' directive.");
        return false;
    }

    if !no_function_prototypes {
        eprintln!("Validation failed: Remove function prototypes from the code.");
        return false;
    }

    if !proper_indentation {
        eprintln!("Validation failed: Ensure proper indentation in the code.");
        return false;
    }

    if !no_static_globals {
        eprintln!("Validation failed: Avoid using 'static' for globals. Prefer unnamed namespaces.");
        return false;
    }

    if !proper_function_definitions {
        eprintln!("Validation failed: Check for proper function definitions.");
        return false;
    }

    if !proper_variable_naming {
        eprintln!("Validation failed: Check for proper variable naming.");
        return false;
    }

    // If all validations pass
    true
}

// Function to check if function definitions follow a proper format
fn is_proper_function_definitions(code: &str) -> bool {
    let function_definition_pattern = Regex::new(r#"[\w<>]+\s+\w+\s*\([^)]*\)\s*{\s*"#).unwrap();  

    for cap in function_definition_pattern.captures_iter(code) {
        let function_signature = cap.get(0).unwrap().as_str();
        let opening_brace_index = function_signature.rfind('{');
        let closing_brace_index = function_signature.rfind('}');

        match (opening_brace_index, closing_brace_index) {
            (Some(open), Some(close)) if open < close => continue,
            _ => return false,
        }
    }

    true
}

// Function to check if variable names follow a proper format
fn is_proper_variable_naming(code: &str) -> bool {
    // Add your variable naming conventions or patterns here
    let valid_variable_name_pattern = Regex::new(r#"\b[a-z]\w*\b"#).unwrap();

    valid_variable_name_pattern.is_match(code)    
}
