#[allow(unused_imports)]
use syn::{visit::Visit, File, Generics, Item, TypeParamBound};
use std::fs;

#[derive(Debug)]
struct TypeInfo {
    name: String,
    bounds: Vec<String>,
    context: String,  // Stores where this type was found (struct/trait/function name)
}

#[derive(Debug)]
struct LifetimeInfo {
    name: String,
    context: String,
}

/// Enhanced visitor to collect detailed generic information
struct GenericVisitor {
    types: Vec<TypeInfo>,
    lifetimes: Vec<LifetimeInfo>,
    current_context: String,
}

impl GenericVisitor {
    fn new() -> Self {
        Self {
            types: Vec::new(),
            lifetimes: Vec::new(),
            current_context: String::new(),
        }
    }

    fn set_context(&mut self, context: &str) {
        self.current_context = context.to_string();
    }

    fn format_output(&self) -> String {
        let mut output = String::new();

        // Format Generic Types and their bounds
        output.push_str("\n=== Generic Types ===\n");
        for type_info in &self.types {
            output.push_str(&format!("\nIn {}:\n", type_info.context));
            output.push_str(&format!("  Type: {}\n", type_info.name));
            if !type_info.bounds.is_empty() {
                output.push_str("  Bounds:\n");
                for bound in &type_info.bounds {
                    output.push_str(&format!("    - {}\n", bound.trim()));
                }
            }
        }

        // Format Lifetimes
        output.push_str("\n=== Lifetimes ===\n");
        let mut lifetime_map: std::collections::HashMap<String, Vec<String>> = std::collections::HashMap::new();
        for lifetime in &self.lifetimes {
            lifetime_map
                .entry(lifetime.name.clone())
                .or_default()
                .push(lifetime.context.clone());
        }

        for (lifetime, contexts) in lifetime_map.iter() {
            output.push_str(&format!("\nLifetime '{}\n", lifetime));
            output.push_str("  Used in:\n");
            for context in contexts {
                output.push_str(&format!("    - {}\n", context));
            }
        }

        output
    }
}

impl<'ast> Visit<'ast> for GenericVisitor {
    fn visit_item(&mut self, item: &'ast Item) {
        match item {
            Item::Struct(item_struct) => {
                self.set_context(&format!("struct {}", item_struct.ident));
                syn::visit::visit_item_struct(self, item_struct);
            }
            Item::Trait(item_trait) => {
                self.set_context(&format!("trait {}", item_trait.ident));
                syn::visit::visit_item_trait(self, item_trait);
            }
            Item::Fn(item_fn) => {
                self.set_context(&format!("function {}", item_fn.sig.ident));
                syn::visit::visit_item_fn(self, item_fn);
            }
            _ => {
                syn::visit::visit_item(self, item);
            }
        }
    }

    fn visit_generics(&mut self, generics: &'ast Generics) {
        for param in &generics.params {
            match param {
                syn::GenericParam::Type(type_param) => {
                    let mut bounds = Vec::new();
                    for bound in &type_param.bounds {
                        if let TypeParamBound::Trait(trait_bound) = bound {
                            bounds.push(quote::quote!(#trait_bound).to_string());
                        }
                    }
                    
                    self.types.push(TypeInfo {
                        name: type_param.ident.to_string(),
                        bounds,
                        context: self.current_context.clone(),
                    });
                }
                syn::GenericParam::Lifetime(lifetime) => {
                    self.lifetimes.push(LifetimeInfo {
                        name: lifetime.lifetime.ident.to_string(),
                        context: self.current_context.clone(),
                    });
                }
                _ => {}
            }
        }
        syn::visit::visit_generics(self, generics);
    }
}

fn main() {
    // Load the Rust source file
    let source_code = fs::read_to_string("src/sample.rs")
        .expect("Unable to read source file");

    // Parse the source code into a syntax tree
    let syntax_tree: File = syn::parse_file(&source_code)
        .expect("Unable to parse source code");

    // Initialize the visitor
    let mut visitor = GenericVisitor::new();

    // Walk through the syntax tree
    visitor.visit_file(&syntax_tree);

    // Output the formatted information
    println!("{}", visitor.format_output());
}