use swc_ecma_ast::*;
use swc_ecma_visit::{VisitMut, VisitMutWith};

pub struct ImportInserter {
    pub imports: Vec<(String, String)>,
    pub existing_imports: Vec<(String, String)>,
    pub used_functions: Vec<String>,
}

impl ImportInserter {
    pub fn new() -> Self {
        Self {
            imports: vec![
                ("defineNuxtComponent".into(), "#imports".into()),
                ("useState".into(), "#imports".into()),
                ("useRuntimeConfig".into(), "#imports".into()),
                ("useFetch".into(), "#imports".into()),
                ("NuxtLink".into(), "#components".into()),
                ("Suspense".into(), "#components".into()),
                ("NuxtLayout".into(), "#components".into()),
                ("NuxtPage".into(), "#components".into()),
            ],
            existing_imports: vec![],
            used_functions: vec![],
        }
    }
}

impl VisitMut for ImportInserter {
    fn visit_mut_module_items(&mut self, items: &mut Vec<ModuleItem>) {
        for item in items.iter() {
            if let ModuleItem::ModuleDecl(ModuleDecl::Import(import)) = item {
                let module_name = import.src.value.to_string();

                for specifier in &import.specifiers {
                    if let ImportSpecifier::Named(named) = specifier {
                        let import_name = named.local.sym.to_string();
                        self.existing_imports
                            .push((import_name.clone(), module_name.clone()));
                    }
                }
            }
        }

        // 使用されている関数やコンポーネントをリストアップ
        for item in items.iter_mut() {
            item.visit_mut_children_with(self);
        }

        // 必要な `import` を整理
        let mut needed_imports: std::collections::HashMap<String, Vec<String>> =
            std::collections::HashMap::new();

        for (func_name, module) in &self.imports {
            if self.used_functions.contains(func_name)
                && !self
                    .existing_imports
                    .contains(&(func_name.clone(), module.clone()))
            {
                needed_imports
                    .entry(module.clone())
                    .or_default()
                    .push(func_name.clone());
            }
        }

        // script 内に import 文を追加
        let mut new_imports = vec![];
        for (module, funcs) in needed_imports {
            let import_stmt = format!("import {{ {} }} from \"{}\";", funcs.join(", "), module);
            new_imports.push(import_stmt);
        }

        // 追加された import 文を script 内に挿入
        if !new_imports.is_empty() {
            let import_block = new_imports.join("\n") + "\n";
            let import_stmt = ModuleItem::Stmt(Stmt::Expr(ExprStmt {
                span: Default::default(),
                expr: Box::new(Expr::Lit(Lit::Str(Str {
                    span: Default::default(),
                    value: import_block.into(),
                    raw: None,
                }))),
            }));
            items.insert(0, import_stmt);
        }
    }

    fn visit_mut_expr(&mut self, expr: &mut Expr) {
        if let Expr::Call(CallExpr { callee, .. }) = expr {
            if let Callee::Expr(boxed_expr) = callee {
                if let Expr::Ident(ident) = &**boxed_expr {
                    let func_name = ident.sym.to_string();
                    if !self.used_functions.contains(&func_name) {
                        self.used_functions.push(func_name);
                    }
                }
            }
        }

        expr.visit_mut_children_with(self);
    }

    fn visit_mut_jsx_element(&mut self, jsx: &mut JSXElement) {
        if let JSXElementName::Ident(ident) = &jsx.opening.name {
            let component_name = ident.sym.to_string();
            let components = vec!["NuxtLink", "Suspense", "NuxtLayout", "NuxtPage"];

            if components.contains(&component_name.as_str())
                && !self.used_functions.contains(&component_name)
            {
                self.used_functions.push(component_name);
            }
        }

        jsx.visit_mut_children_with(self);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use swc_common::{sync::Lrc, SourceMap};
    use swc_ecma_codegen::{text_writer::JsWriter, Emitter};
    use swc_ecma_parser::{lexer::Lexer, Parser, StringInput, Syntax};
    use swc_ecma_visit::VisitMutWith;

    fn apply_transform(source: &str) -> String {
        let cm: Lrc<SourceMap> = Default::default();
        let fm = cm.new_source_file(swc_common::FileName::Anon.into(), source.into());

        let lexer = Lexer::new(
            Syntax::Es(Default::default()),
            EsVersion::latest(),
            StringInput::from(&*fm),
            None,
        );

        let mut parser = Parser::new_from(lexer);
        let mut module = parser.parse_module().expect("Failed to parse module");

        let mut inserter = ImportInserter::new();
        module.visit_mut_with(&mut inserter);

        let mut buf = vec![];
        {
            let wr = JsWriter::new(cm.clone(), "", &mut buf, None);
            let mut emitter = Emitter {
                cfg: Default::default(),
                cm: cm.clone(),
                comments: None,
                wr,
            };
            emitter.emit_module(&module).expect("Failed to emit module");
        }
        String::from_utf8(buf).expect("Generated code is not valid UTF-8")
    }

    #[test]
    fn test_insert_import_for_define_nuxt_component() {
        let source = r#"
        defineNuxtComponent({
            setup() {
                return {};
            }
        });
        "#;
        let transformed = apply_transform(source);
        let expected_import = "import { defineNuxtComponent } from \"#imports\";";
        assert!(
            transformed.contains(expected_import),
            "Import should be added"
        );
    }

    #[test]
    fn test_insert_multiple_imports() {
        let source = r#"
        const state = useState('count', () => 0);
        const config = useRuntimeConfig();
        const data = useFetch('/api/data');
        "#;
        let transformed = apply_transform(source);
        let expected_import = "import { useState, useRuntimeConfig, useFetch } from \"#imports\";";

        assert!(
            transformed.contains(expected_import),
            "All necessary imports should be added"
        );
    }

    #[test]
    fn test_prevent_duplicate_import() {
        let source = r#"
        import { defineNuxtComponent, useState, useRuntimeConfig, useFetch } from '#imports';
        
        defineNuxtComponent({
            setup() {
                const state = useState('count', () => 0);
                const config = useRuntimeConfig();
                const data = useFetch('/api/data');
                return {};
            }
        });
        "#;
        let transformed = apply_transform(source);
        let import_count = transformed.matches("import { defineNuxtComponent, useState, useRuntimeConfig, useFetch } from '#imports';").count();
        assert_eq!(import_count, 1, "Import should not be duplicated");
    }

    #[test]
    fn test_insert_import_for_template_components() {
        let source = r#"
        <template>
            <NuxtLink to=\"/about\">About</NuxtLink>
            <Suspense>
                <NuxtPage />
            </Suspense>
        </template>
        "#;
        let transformed = apply_transform(source);
        let expected_import = "import { NuxtLink, Suspense, NuxtPage } from \"#components\";";
        assert!(
            transformed.contains(expected_import),
            "Template component imports should be added"
        );
    }

    #[test]
    fn test_insert_import_when_mixed_with_existing_imports() {
        let source = r#"
        import { useState } from '#imports';
        
        const state = useState('count', () => 0);
        const config = useRuntimeConfig();
        "#;
        let transformed = apply_transform(source);
        let expected_import = "import { useRuntimeConfig } from \"#imports\";";

        assert!(
            transformed.contains(expected_import),
            "Missing imports should be added without duplicating existing ones"
        );
    }

    #[test]
    fn test_no_imports_added_when_not_needed() {
        let source = r#"
        console.log('Hello, world!');
        "#;
        let transformed = apply_transform(source);
        assert!(
            !transformed.contains("import "),
            "No import should be added"
        );
    }
}
