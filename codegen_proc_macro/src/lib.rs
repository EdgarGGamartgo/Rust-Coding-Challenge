use proc_macro::TokenStream as TokenStream1;
use proc_macro::TokenStream;
use quote::quote;
use std::fs::File;
use std::io::Read;
use syn::parse_macro_input;
use serde::Deserialize;

#[derive(Deserialize)]
struct MethodConfig {
    methods: Vec<String>,
}

#[proc_macro]
pub fn codegen(input: TokenStream1) -> TokenStream {
    let json_file_path: String = parse_macro_input!(input as syn::LitStr).value();

    // Read the JSON file content
    let json_content = read_json_file(&json_file_path).unwrap_or_else(|e| {
        panic!("Failed to read JSON file {}: {}", json_file_path, e);
    });

    // Deserialize the JSON content
    let method_config: MethodConfig = serde_json::from_str(&json_content).unwrap_or_else(|e| {
        panic!("Failed to parse JSON content: {}", e);
    });

    // Generate Rust code based on the deserialized configuration
    let generated_code = generate_code(&method_config);

    // Parse the generated code back into a proc_macro::TokenStream
    let tokens = proc_macro2::TokenStream::from(syn::parse_str::<proc_macro2::TokenStream>(&generated_code).unwrap_or_else(|e| {
        panic!("Failed to parse generated code: {}", e);
    }));

    tokens.into()
}

fn read_json_file(file_path: &str) -> Result<String, std::io::Error> {
    let mut file = File::open(file_path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

fn generate_code(config: &MethodConfig) -> String {
    let method_names = &config.methods;
    let generated_functions = method_names
        .iter()
        .map(|method_name| {
            let function_name = syn::Ident::new(&method_name, proc_macro2::Span::call_site());
            format!(
                "pub fn {}() {{ println!(\"Function {} called\"); }}",
                function_name, method_name
            )
        })
        .collect::<Vec<String>>()
        .join("\n");

    format!(
        "pub mod my_generated_module {{\n{}\n}}",
        generated_functions
    )
}
