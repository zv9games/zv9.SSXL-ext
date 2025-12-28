use std::collections::HashMap;

use tracing::info;
use walkdir::WalkDir;
use syn::{
    visit::Visit,
    Attribute,
    File,
    FnArg,
    ImplItem,
    ItemImpl,
    ItemMod,
    ItemStruct,
    ReturnType,
    Type,
    Expr,
    ExprLit,
    ExprMethodCall,
    ExprReference,
    Lit,
    Meta,
    punctuated::Punctuated,
    token::Comma,
};

/// Represents the API surface of a single Godot-exposed class.
#[derive(Default)]
struct ClassApi {
    methods: Vec<String>,
    signals: Vec<String>,
}

/// ✅ Top-level scanner: walks ssxl_ext/src and reports the Godot API surface.
/// Now anchored so it never overwrites the scene or menu.
pub fn print_godot_api_surface(menu_y: u16) {
    use crossterm::{cursor::MoveTo, execute, terminal::{Clear, ClearType}};
    use std::io;

    info!("Scanning SSXL-ext source for exported Godot API...");

    let mut stdout = io::stdout();
    let mut y = menu_y + 4; // ✅ safely below menu + prompt

    // Clear action area before printing
    execute!(
        &mut stdout,
        MoveTo(0, y),
        Clear(ClearType::FromCursorDown)
    ).unwrap();

    let ext_src_path = format!(
        "{}/../ssxl_ext/src",
        env!("CARGO_MANIFEST_DIR")
    );
    let mut visitor = ApiScanVisitor::default();

    for entry in WalkDir::new(&ext_src_path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().map(|x| x == "rs").unwrap_or(false))
    {
        let content = match std::fs::read_to_string(entry.path()) {
            Ok(c) => c,
            Err(_) => continue,
        };
        let parsed: File = match syn::parse_file(&content) {
            Ok(f) => f,
            Err(_) => continue,
        };
        visitor.visit_file(&parsed);
    }

    if visitor.classes.is_empty() && visitor.global_signals.is_empty() {
        execute!(&mut stdout, MoveTo(0, y)).unwrap();
        println!("(No exported #[func] methods or signals found in ssxl_ext source.)");
        return;
    }

    execute!(&mut stdout, MoveTo(0, y)).unwrap();
    println!("--- SSXL-ext Godot API Surface (Source Scan) ---");
    y += 1;

    let mut class_names: Vec<_> = visitor.classes.keys().cloned().collect();
    class_names.sort();

    for class_name in class_names {
        let class_api = &visitor.classes[&class_name];

        execute!(&mut stdout, MoveTo(0, y)).unwrap();
        println!("Class: {class_name}");
        y += 1;

        if !class_api.signals.is_empty() {
            execute!(&mut stdout, MoveTo(0, y)).unwrap();
            println!(" Signals:");
            y += 1;

            for sig in &class_api.signals {
                execute!(&mut stdout, MoveTo(0, y)).unwrap();
                println!("  • {}", sig);
                y += 1;
            }
        }

        if !class_api.methods.is_empty() {
            if !class_api.signals.is_empty() {
                y += 1;
            }

            execute!(&mut stdout, MoveTo(0, y)).unwrap();
            println!(" Methods:");
            y += 1;

            for method in &class_api.methods {
                execute!(&mut stdout, MoveTo(0, y)).unwrap();
                println!("  • {}", method);
                y += 1;
            }
        }

        y += 1;
    }

    if !visitor.global_signals.is_empty() {
        execute!(&mut stdout, MoveTo(0, y)).unwrap();
        println!("Global signals (manual Engine.emit_signal calls):");
        y += 1;

        for sig in &visitor.global_signals {
            execute!(&mut stdout, MoveTo(0, y)).unwrap();
            println!("  • {}", sig);
            y += 1;
        }

        y += 1;
    }

    execute!(&mut stdout, MoveTo(0, y)).unwrap();
    println!("--- End of API ---");
}


/// Visitor that discovers Godot classes and exported methods/signals.
#[derive(Default)]
struct ApiScanVisitor {
    classes: HashMap<String, ClassApi>,
    global_signals: Vec<String>,
}

impl<'ast> Visit<'ast> for ApiScanVisitor {
    fn visit_item_mod(&mut self, node: &'ast ItemMod) {
        if let Some((_, items)) = &node.content {
            for item in items {
                self.visit_item(item);
            }
        }
        syn::visit::visit_item_mod(self, node);
    }

    fn visit_item_struct(&mut self, node: &'ast ItemStruct) {
        let is_godot_class = node.attrs.iter().any(derive_has_godot_class);
        if is_godot_class {
            let name = node.ident.to_string();
            self.classes.entry(name).or_default();
        }
        syn::visit::visit_item_struct(self, node);
    }

    fn visit_item_impl(&mut self, node: &'ast ItemImpl) {
        let type_name = match &*node.self_ty {
            Type::Path(tp) => tp.path.segments.last().map(|s| s.ident.to_string()),
            _ => None,
        };

        if let Some(type_name) = type_name {
            if let Some(class_api) = self.classes.get_mut(&type_name) {
                for item in &node.items {
                    if let ImplItem::Fn(m) = item {
                        let has_func = m.attrs.iter().any(|a| attr_contains(a, "func"));
                        if has_func {
                            let sig = format_method_signature(
                                m.sig.inputs.iter().collect(),
                                &m.sig.output,
                                &m.sig.ident.to_string(),
                            );
                            class_api.methods.push(sig);
                        }

                        let has_signal = m.attrs.iter().any(|a| attr_contains(a, "signal"));
                        if has_signal {
                            class_api
                                .signals
                                .push(format!("{}(...)", m.sig.ident.to_string()));
                        }
                    }
                }
            }
        }

        syn::visit::visit_item_impl(self, node);
    }

    fn visit_expr_method_call(&mut self, node: &'ast ExprMethodCall) {
        if node.method == "call" && !node.args.is_empty() {
            if let Some(signal_name) = extract_engine_signal_name(node) {
                let formatted = format!("Engine.emit_signal(\"{}\", ...)", signal_name);
                if !self.global_signals.contains(&formatted) {
                    self.global_signals.push(formatted);
                }
            }
        }
        syn::visit::visit_expr_method_call(self, node);
    }
}

/// Detect #[derive(GodotClass)]
fn derive_has_godot_class(attr: &Attribute) -> bool {
    if !attr.path().is_ident("derive") {
        return false;
    }

    if let Meta::List(meta_list) = &attr.meta {
        if let Ok(derives) =
            meta_list.parse_args_with(Punctuated::<Meta, Comma>::parse_terminated)
        {
            return derives.iter().any(|meta| {
                matches!(meta, Meta::Path(path) if path.is_ident("GodotClass"))
            });
        }
    }

    false
}

/// Format a pretty method signature
fn format_method_signature(inputs: Vec<&FnArg>, output: &ReturnType, name: &str) -> String {
    let mut args = Vec::new();

    for arg in inputs {
        if let FnArg::Typed(pat) = arg {
            let arg_name = match &*pat.pat {
                syn::Pat::Ident(id) => id.ident.to_string(),
                _ => "_".to_string(),
            };
            let arg_ty = match &*pat.ty {
                Type::Path(tp) => tp.path.segments.last().unwrap().ident.to_string(),
                _ => "Unknown".to_string(),
            };
            args.push(format!("{}: {}", arg_name, arg_ty));
        }
    }

    let args_joined = args.join(", ");

    let ret = match output {
        ReturnType::Default => "".to_string(),
        ReturnType::Type(_, ty) => {
            let ty_str = match &**ty {
                Type::Path(tp) => tp.path.segments.last().unwrap().ident.to_string(),
                _ => "Unknown".to_string(),
            };
            format!(" -> {}", ty_str)
        }
    };

    format!("{}({}){}", name, args_joined, ret)
}

/// Extract signal name from Engine::singleton().call("emit_signal", [...])
fn extract_engine_signal_name(call: &ExprMethodCall) -> Option<String> {
    let first_arg = call.args.first()?;
    let is_emit_signal = matches!(
        first_arg,
        Expr::Lit(ExprLit { lit: Lit::Str(s), .. }) if s.value() == "emit_signal"
    );
    if !is_emit_signal {
        return None;
    }

    let second_arg = call.args.iter().nth(1)?;
    let array_expr = match second_arg {
        Expr::Reference(ExprReference { expr, .. }) => match &**expr {
            Expr::Array(arr) => arr,
            _ => return None,
        },
        Expr::Array(arr) => arr,
        _ => return None,
    };

    array_expr.elems.first().and_then(extract_string_literal)
}

/// Recursively extract a string literal from an expression
fn extract_string_literal(expr: &Expr) -> Option<String> {
    match expr {
        Expr::Lit(ExprLit { lit: Lit::Str(s), .. }) => Some(s.value()),
        Expr::MethodCall(m) => extract_string_literal(&m.receiver),
        Expr::Reference(r) => extract_string_literal(&r.expr),
        Expr::Array(arr) => arr.elems.iter().find_map(extract_string_literal),
        _ => None,
    }
}

/// Helper: detect simple or nested attributes containing a given name
fn attr_contains(attr: &Attribute, name: &str) -> bool {
    if attr.path().is_ident(name) {
        return true;
    }

    if let Meta::List(list) = &attr.meta {
        let tokens = list.tokens.to_string();
        return tokens.contains(name);
    }

    false
}
