use std::collections::HashMap;

use crate::jasmine::*;

static mut CREATE_CLOSURES: Option<HashMap<String, ClosureTypeData>> = None;

const BUILTINS_JAVA: &'static str = include_str!("Builtins.java");

pub fn add_closure(closure: ClosureTypeData) -> String {
    let args = closure
        .args
        .iter()
        .cloned()
        .map(|n| {
            if n.generic {
                "Generic".to_string()
            } else {
                n.ty.rewrite()
            }
        })
        .map(|n| {
            n.char_indices()
                .map(|(idx, ch)| {
                    if idx == 0 {
                        ch.to_ascii_uppercase()
                    } else {
                        ch
                    }
                })
                .collect::<String>()
        })
        .join("");

    let ret = closure
        .ret
        .clone()
        .map(|n| n.as_ref().clone())
        .map(|n| {
            if n.generic {
                "Generic".to_string()
            } else {
                n.ty.rewrite()
                    .char_indices()
                    .map(|(idx, ch)| {
                        if idx == 0 {
                            ch.to_ascii_uppercase()
                        } else {
                            ch
                        }
                    })
                    .collect()
            }
        })
        .unwrap_or("void".to_string());

    let mut rewritten = format!(
        "Closure_{}_Ret{}",
        {
            if args.len() == 0 {
                "void".to_string()
            } else {
                args
            }
        },
        ret
    );

    unsafe {
        if let Some(closures) = &mut CREATE_CLOSURES {
            closures.insert(rewritten.clone(), closure.clone());
        } else {
            CREATE_CLOSURES = Some(HashMap::new());
            CREATE_CLOSURES
                .as_mut()
                .unwrap()
                .insert(rewritten.clone(), closure.clone());
        }
    }

    let generics = closure
        .args
        .iter()
        .filter(|n| n.generic)
        .map(|n| n.ty.rewrite())
        .collect_vec();

    let ret_is_generic = closure.ret.as_ref().map(|n| n.generic).unwrap_or(false);

    if !generics.is_empty() || ret_is_generic {
        rewritten.push('<');

        for i in generics {
            rewritten.push_str(&i);
            rewritten.push(',');
        }

        if ret_is_generic {
            rewritten.push_str(&closure.ret.as_ref().unwrap().ty.rewrite());
        } else {
            rewritten.pop();
        }

        rewritten.push('>');
    }

    rewritten
}

pub fn rewrite_struct_impl(structure: Structure, impls: Vec<Impl>) -> String {
    let mut rewritten = "".to_string();

    let ident = structure.ident.clone();
    rewritten.push_str(&format!("public static class {ident}"));

    if let Some(generics) = structure.generics {
        rewritten.push_str(&generics.rewrite(structure.where_clause.as_ref()));
    }

    rewritten.push_str(" {\n");

    rewritten.push_str(&Arg::rewrite_many((&structure.fields).clone(), ";\n"));
    rewritten.push_str(";\n");

    let constructor_args = structure
        .fields
        .iter()
        .sorted_by(|a, b| a.ident.cmp(&b.ident))
        .map(|n| format!("{} _{}", n.ty.rewrite(), n.ident))
        .join(", ");

    let constructor_values = structure
        .fields
        .iter()
        .sorted_by(|a, b| a.ident.cmp(&b.ident))
        .map(|n| format!("this.{} = _{};", n.ident, n.ident))
        .join("\n");

    let constructor = format!(
        "
		public {}({}) {{
			{}
		}}
		",
        structure.ident, constructor_args, constructor_values
    );

    rewritten.push_str(&constructor);

    for imp in impls {
        for method in imp.methods {
            rewritten.push_str(&method.rewrite());
        }
    }

    rewritten.push_str("}\n");

    rewritten
}

fn rewrite_enum_impl(enu: Enumeration, impls: Vec<Impl>) -> String {
    let mut rewritten = enu.rewrite_no_closing();

    for imp in impls {
        for method in imp.methods {
            rewritten.push_str(&method.rewrite());
        }
    }

    rewritten.push_str("}\n");

    rewritten
}

fn rewrite_closure(ident: String, closure: ClosureTypeData) -> String {
    let mut rewritten = format!("public interface {ident}");
    let mut generics = vec![];

    for (idx, arg) in closure.args.iter().enumerate() {
        if arg.generic {
            generics.push(format!("Arg{}", idx));
        }
    }

    if closure.ret.as_ref().map(|n| n.generic).unwrap_or(false) {
        generics.push("Ret".to_string());
    }

    if !generics.is_empty() {
        rewritten.push_str(&format!("<{}>", generics.join(", ")));
    }

    rewritten.push_str(&format!(
        " {{
		{} call(",
        closure
            .ret
            .as_ref()
            .map(|n| if n.generic {
                "Ret".to_string()
            } else {
                n.ty.rewrite()
            })
            .unwrap_or("void".to_string())
    ));

    for (idx, arg) in closure.args.iter().enumerate() {
        if arg.generic {
            rewritten.push_str(&format!("Arg{} arg{}", idx, idx));
        } else {
            rewritten.push_str(&format!("{} arg{}", arg.ty.rewrite(), idx));
        }

        if idx != closure.args.len() - 1 {
            rewritten.push_str(", ");
        }
    }

    rewritten.push_str(");\n}\n");

    rewritten
}

pub fn rewrite_ident<S>(_ident: S) -> String
where
    S: ToString,
{
    let ident = _ident.to_string();

    // handle special cases
    match ident.as_str() {
        "string" => return String::from("String"),
        "bool" => return String::from("booean"),
        "default" => return String::from("default_value"),
        _ => {}
    }

    if ident == "self" {
        return "this".to_string();
    }

    let mut words = ident.split('_');
    let mut new = words
        .next()
        .map(|f| if f == "" { "_" } else { f })
        .map(ToString::to_string)
        .unwrap();

    for word in words {
        if word == "" {
            new.push('_');
        } else {
            let l1upper = word.chars().next().unwrap().to_ascii_uppercase();

            let mut new_word = word.chars().skip(1).collect::<String>();
            new_word.insert(0, l1upper);

            new.push_str(&new_word);
        }
    }

    if new == "_" {
        return String::from("__");
    }

    new
}

pub fn rewrite(program: Vec<JasmineProgramComponent>, root_class: &String) -> String {
    let mut rewritten = format!(
        "
		import java.util.*;
		import java.util.stream.*;
	
		public class {root_class} {{\n"
    );

    let mut struct_impl_map = vec![];
    let mut enum_impl_map = vec![];

    rewritten.push_str(&mixin_builtins());

    for structure in program
        .iter()
        .filter_map(|n| {
            let JasmineProgramComponent::Struct(structure) = n else { return None };
            Some(structure)
        })
        .cloned()
    {
        let impls = program
            .iter()
            .filter_map(|n| {
                let JasmineProgramComponent::Impl(imp) = n else { return None };
                Some(imp)
            })
            .filter(|imp| imp.ident == structure.ident)
            .cloned()
            .collect::<Vec<_>>();

        struct_impl_map.push((structure, impls));
    }

    for enu in program
        .iter()
        .filter_map(|n| {
            let JasmineProgramComponent::Enum(enu) = n else { return None };
            Some(enu)
        })
        .cloned()
    {
        let impls = program
            .iter()
            .filter_map(|n| {
                let JasmineProgramComponent::Impl(imp) = n else { return None };
                Some(imp)
            })
            .filter(|imp| imp.ident == enu.ident)
            .cloned()
            .collect::<Vec<_>>();

        enum_impl_map.push((enu, impls));
    }

    for (structure, impls) in struct_impl_map {
        rewritten.push_str(&format!("{}\n", rewrite_struct_impl(structure, impls)));
    }

    for (enu, impls) in enum_impl_map {
        rewritten.push_str(&format!("{}\n", rewrite_enum_impl(enu, impls)));
    }

    for item in program {
        match item {
            JasmineProgramComponent::Fn(f) => {
                if f.ident == "main" {
                    /* Main override */
                    rewritten.push_str("public static void main(String[] args) {\n");
                    rewritten.push_str(&BlockPart::rewrite_many(f.body, "\n"));
                    rewritten.push_str("\n}");
                } else {
                    rewritten.push_str(&f.rewrite());
                }
            }
            JasmineProgramComponent::Var(v) => {
                rewritten.push_str(&format!("static {};", &v.rewrite()));
            }
            _ => {}
        }
    }

    for (ident, closure_type) in unsafe { CREATE_CLOSURES.clone().unwrap_or(HashMap::new()) } {
        rewritten.push_str(&rewrite_closure(ident, closure_type));
    }

    rewritten.push_str("}");

    rewritten
}

fn mixin_builtins() -> String {
    let mut lines = BUILTINS_JAVA.lines();
    let mut output_lines = vec![];

    let mut in_builtins = false;

    while let Some(line) = lines.next() {
        if line
            .trim()
            .to_lowercase()
            .contains("jasmine_builtins_start")
        {
            in_builtins = true;
        } else if line.trim().to_lowercase().contains("jasmine_builtins_end") {
            in_builtins = false;
        } else if in_builtins {
            output_lines.push(line);
        }
    }

    output_lines.join("\n")
}
