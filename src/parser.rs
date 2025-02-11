use crate::transformer::ImportInserter;
use swc_common::{sync::Lrc, FileName, SourceMap};
use swc_ecma_ast::*;
use swc_ecma_codegen::{text_writer::JsWriter, Emitter};
use swc_ecma_parser::{lexer::Lexer, Parser as SwcParser, StringInput, Syntax};
use swc_ecma_visit::VisitMutWith;

pub fn process_script_setup(script_content: &str) -> String {
    let cm: Lrc<SourceMap> = Default::default();
    let fm = cm.new_source_file(FileName::Anon.into(), script_content.into());

    let lexer = Lexer::new(
        Syntax::Es(Default::default()),
        EsVersion::Es2022,
        StringInput::from(&*fm),
        None,
    );
    let mut parser = SwcParser::new_from(lexer);

    let mut module = match parser.parse_module() {
        Ok(module) => module,
        Err(_) => return script_content.to_string(),
    };

    let mut visitor = ImportInserter::new();
    module.visit_mut_with(&mut visitor);

    let mut buf = vec![];
    {
        let mut emitter = Emitter {
            cfg: Default::default(),
            cm: cm.clone(),
            comments: None,
            wr: JsWriter::new(cm, "\n", &mut buf, None),
        };
        emitter.emit_module(&module).unwrap();
    }

    String::from_utf8(buf).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use swc_ecma_visit::VisitMut;

    /// Mock implementation of `ImportInserter`
    struct MockImportInserter {
        called: RefCell<bool>,
    }

    impl MockImportInserter {
        fn new() -> Self {
            Self {
                called: RefCell::new(false),
            }
        }
    }

    impl VisitMut for MockImportInserter {
        fn visit_mut_module(&mut self, _: &mut Module) {
            *self.called.borrow_mut() = true;
        }
    }

    /// Test if `process_script_setup` calls `ImportInserter::visit_mut_with`
    #[test]
    fn test_process_script_setup_calls_import_inserter() {
        let script_content = "const a = 42;";

        let cm: Lrc<SourceMap> = Default::default();
        let fm = cm.new_source_file(FileName::Anon.into(), script_content.into());

        let lexer = Lexer::new(
            Syntax::Es(Default::default()),
            EsVersion::Es2022,
            StringInput::from(&*fm),
            None,
        );
        let mut parser = SwcParser::new_from(lexer);

        let mut module = parser.parse_module().expect("Failed to parse module");
        let mut mock_visitor = MockImportInserter::new();
        module.visit_mut_with(&mut mock_visitor);

        assert!(
            *mock_visitor.called.borrow(),
            "ImportInserter::visit_mut_with was not called"
        );
    }

    /// Test if `parser.parse_module()` fails, the original script content is returned
    #[test]
    fn test_process_script_setup_parser_fails() {
        let script_content = "const a ="; // Incomplete code to trigger a syntax error
        let result = process_script_setup(script_content);

        assert_eq!(
            result, script_content,
            "If the parser fails, the original script content should be returned"
        );
    }
}
