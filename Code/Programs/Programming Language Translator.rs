use std::fmt::Write;

#[derive(Debug)]
struct RustFunction {
    name: String,
    parameters: Vec<(String, String)>,
    return_type: String,
    body: String,
}

impl RustFunction {
    fn new(name: &str, parameters: Vec<(&str, &str)>, return_type: &str, body: &str) -> Self {
        let parameters = parameters
            .into_iter()
            .map(|(n, t)| (n.to_string(), t.to_string()))
            .collect(); 
        RustFunction {
            name: name.to_string(),
            parameters,
            return_type: return_type.to_string(),
            body: body.to_string(),
        }
    }
}

trait LanguageConverter {
    fn convert(&self, rust_function: &RustFunction) -> Result<String, String>;
    fn details(&self) -> &'static LanguageDetails;    
}

trait LanguageDetails {
    fn name(&self) -> &'static str;
    fn file_extension(&self) -> &'static str;    
}

struct CodeConverter<'a> {
    language_converters: Vec<Box<dyn LanguageConverter + 'a>>,    
}

impl<'a> CodeConverter<'a> {
    fn new() -> Self {
        CodeConverter {
            language_converters: vec![
                Box::new(JavaConverter {}),
                Box::new(CppConverter {}),
                Box::new(KotlinConverter {}),
                Box::new(PythonConverter {}),
                Box::new(RustConverter {}),
                Box::new(GoConverter {}),
                Box::new(RubyConverter {}),
                Box::new(SwiftConverter {}),
                Box::new(CSharpConverter {}),
                // Add other converters here
            ],    
        }
    }

    fn convert_to_languages(&self, rust_functions: &[RustFunction]) {
        for function in rust_functions {
            for converter in &self.language_converters {
                match converter.convert(function) {
                    Ok(result) => println!("Converted {}:\n{}", converter.details().name(), result),
                    Err(err) => eprintln!(
                        "Error converting {} to {}: {}",
                        function.name,
                        converter.details().name(),
                        err
                    ),
                }
            }
        }    
    }
}

macro_rules! language_converter {
    ($struct_name:ident, $lang_name:expr, $ext:expr, $format_code:expr) => {
        struct $struct_name;

        impl LanguageDetails for $struct_name {
            fn name(&self) -> &'static str {
                $lang_name
            }

            fn file_extension(&self) -> &'static str {
                $ext
            }
        }

        impl LanguageConverter for $struct_name {
            fn convert(&self, rust_function: &RustFunction) -> Result<String, String> {
                let mut code = String::new();
                write!(
                    code,
                    $format_code,
                    rust_function.return_type,
                    rust_function.name,
                    format_parameters(rust_function),
                    rust_function.body
                )
                .map_err(|e| format!("Error formatting {} code: {}", $lang_name, e))?;
                Ok(code)    
            }
        }

        fn details(&self) -> &'static LanguageDetails {
                self
        }    
    };
}

language_converter!(
    JavaConverter,
    "Java",
    "java",
    "public class Main {{\n  public static {} {}({}) {{\n    return {};\n  }}\n}}"
);

language_converter!(
    CppConverter,
    "C++",
    "cpp",
    "{} {}({}) {{\n  return {};\n}}"
);

language_converter!(
    KotlinConverter,
    "Kotlin",
    "kt",
    "fun {}({}): {} {{\n  return {};\n}}"
);

language_converter!(
    PythonConverter,
    "Python",
    "py",
    "def {}({}) -> {}:\n  return {}"
);

language_converter!(
    RustConverter,
    "Rust",
    "rs",
    "fn {}({}) -> {} {{\n  {}\n}}"
);

language_converter!(
    GoConverter,
    "Go",
    "go",
    "package main\n\nimport \"fmt\"\n\nfunc {}({}) {} {{\n  return {}\n}}"
);

language_converter!(
    RubyConverter,
    "Ruby",
    "rb",
    "def {}({})\n  {}\nend"
);

language_converter!(
    SwiftConverter,
    "Swift",
    "swift",
    "func {}({}) -> {} {{\n  return {}\n}}"
);

language_converter!(
    CSharpConverter,
    "C#",
    "cs",
    "using System;\n\npublic class Program {{\n  public static {} {}({}) {{\n    return {};\n}}}}"
);

fn format_parameters(rust_function: &RustFunction) -> String {
    rust_function
        .parameters
        .iter()
        .map(|(name, ty)| format!("{} {}", ty, name))
        .collect::<Vec<String>>()
        .join(", ")    
}

fn main() {
    let rust_functions = vec![
        RustFunction::new("add", vec![("a", "i32"), ("b", "i32")], "i32", "a + b"),
        RustFunction::new("multiply", vec![("a", "i32"), ("b", "i32")], "i32", "a * b"),
    ];  

    let code_converter = CodeConverter::new();
    code_converter.convert_to_languages(&rust_functions);
}
