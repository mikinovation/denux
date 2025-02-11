use regex::Regex;
use std::fs;
use std::path::Path;

pub fn process_vue_file<F>(file_path: &Path, dry_run: bool, verbose: bool, process_script_setup: F)
where
    F: Fn(&str) -> String,
{
    let content = match fs::read_to_string(file_path) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Failed to read {:?}: {}", file_path, e);
            return;
        }
    };

    let script_re = Regex::new(r"(?s)<script\s+setup[^>]*>(.*?)</script>").unwrap();
    if let Some(captures) = script_re.captures(&content) {
        let script_content = &captures[1];
        let new_script_content = process_script_setup(script_content);

        if script_content != new_script_content {
            let new_content = script_re.replace(&content, |_caps: &regex::Captures| {
                format!("<script setup>\n{}\n</script>", new_script_content)
            });

            if dry_run {
                println!("Would update: {:?}", file_path);
            } else if let Err(e) = fs::write(file_path, new_content.as_bytes()) {
                eprintln!("Failed to write {:?}: {}", file_path, e);
            } else if verbose {
                println!("Updated: {:?}", file_path);
            }
        }
    }
}

pub fn process_ts_file<F>(file_path: &Path, dry_run: bool, verbose: bool, process_script_setup: F)
where
    F: Fn(&str) -> String,
{
    let content = match fs::read_to_string(file_path) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Failed to read {:?}: {}", file_path, e);
            return;
        }
    };

    let new_content = process_script_setup(&content);

    if content != new_content {
        if dry_run {
            println!("Would update: {:?}", file_path);
        } else if let Err(e) = fs::write(file_path, new_content.as_bytes()) {
            eprintln!("Failed to write {:?}: {}", file_path, e);
        } else if verbose {
            println!("Updated: {:?}", file_path);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::fs;
    use tempfile::NamedTempFile;

    thread_local! {
        static MOCK_CALLED: RefCell<Option<String>> = const { RefCell::new(None) };
    }

    fn mock_process_script_setup(input: &str) -> String {
        let modified = format!("processed: {}", input);
        MOCK_CALLED.with(|called| *called.borrow_mut() = Some(input.to_string()));
        modified
    }

    #[test]
    fn test_process_vue_file_calls_process_script_setup() {
        let vue_content = r#"
        <template><div>Hello</div></template>
        <script setup>
        const a = 42;
        </script>
        "#;

        let temp_file = NamedTempFile::new().expect("Failed to create temp file");
        fs::write(temp_file.path(), vue_content).expect("Failed to write to temp file");

        MOCK_CALLED.with(|called| *called.borrow_mut() = None);
        process_vue_file(temp_file.path(), false, false, mock_process_script_setup);

        MOCK_CALLED.with(|called| {
            let borrowed = called.borrow();
            assert!(borrowed.is_some(), "process_script_setup was not called");
            assert_eq!(
                borrowed.as_deref(),
                Some("\n        const a = 42;\n        "),
                "process_script_setup received incorrect input"
            );
        });

        let result_content =
            fs::read_to_string(temp_file.path()).expect("Failed to read temp file");
        assert!(
            result_content.contains("processed:"),
            "The processed content was not correctly updated"
        );
    }

    #[test]
    fn test_process_vue_file_no_script_setup() {
        let vue_content = r#"
        <template><div>Hello</div></template>
        <script>
        const a = 42;
        </script>
        "#;

        let temp_file = NamedTempFile::new().expect("Failed to create temp file");
        fs::write(temp_file.path(), vue_content).expect("Failed to write to temp file");

        MOCK_CALLED.with(|called| *called.borrow_mut() = None);
        process_vue_file(temp_file.path(), false, false, mock_process_script_setup);

        MOCK_CALLED.with(|called| {
            assert!(
                called.borrow().is_none(),
                "process_script_setup should not be called when there is no <script setup>"
            );
        });

        let result_content =
            fs::read_to_string(temp_file.path()).expect("Failed to read temp file");
        assert_eq!(
            result_content, vue_content,
            "The file should remain unchanged when there is no <script setup>"
        );
    }

    #[test]
    fn test_process_ts_file_calls_process_script_setup() {
        let ts_content = "const a = 42;";

        let temp_file = NamedTempFile::new().expect("Failed to create temp file");
        fs::write(temp_file.path(), ts_content).expect("Failed to write to temp file");

        MOCK_CALLED.with(|called| *called.borrow_mut() = None);
        process_ts_file(temp_file.path(), false, false, mock_process_script_setup);

        MOCK_CALLED.with(|called| {
            let borrowed = called.borrow();
            assert!(borrowed.is_some(), "process_script_setup was not called");
            assert_eq!(
                borrowed.as_deref(),
                Some("const a = 42;"),
                "process_script_setup received incorrect input"
            );
        });

        let result_content =
            fs::read_to_string(temp_file.path()).expect("Failed to read temp file");
        assert!(
            result_content.contains("processed:"),
            "The processed content was not correctly updated"
        );
    }

    #[test]
    fn test_process_vue_file_dry_run() {
        let vue_content = r#"
        <template><div>Hello</div></template>
        <script setup>
        let x = 10;
        </script>
        "#;

        let temp_file = NamedTempFile::new().expect("Failed to create temp file");
        fs::write(temp_file.path(), vue_content).expect("Failed to write to temp file");

        process_vue_file(temp_file.path(), true, false, mock_process_script_setup);

        let result_content =
            fs::read_to_string(temp_file.path()).expect("Failed to read temp file");
        assert_eq!(
            result_content, vue_content,
            "File should not be modified in dry_run mode"
        );
    }

    #[test]
    fn test_process_ts_file_dry_run() {
        let ts_content = "let y = 20;";

        let temp_file = NamedTempFile::new().expect("Failed to create temp file");
        fs::write(temp_file.path(), ts_content).expect("Failed to write to temp file");

        process_ts_file(temp_file.path(), true, false, mock_process_script_setup);

        let result_content =
            fs::read_to_string(temp_file.path()).expect("Failed to read temp file");
        assert_eq!(
            result_content, ts_content,
            "File should not be modified in dry_run mode"
        );
    }

    #[test]
    fn test_process_vue_file_verbose_mode() {
        let vue_content = r#"
        <template><div>Hello</div></template>
        <script setup>
        let y = 99;
        </script>
        "#;

        let temp_file = NamedTempFile::new().expect("Failed to create temp file");
        fs::write(temp_file.path(), vue_content).expect("Failed to write to temp file");

        process_vue_file(temp_file.path(), false, true, mock_process_script_setup);

        let result_content =
            fs::read_to_string(temp_file.path()).expect("Failed to read temp file");
        assert!(
            result_content.contains("processed:"),
            "Verbose mode should still apply changes"
        );
    }

    #[test]
    fn test_process_ts_file_verbose_mode() {
        let ts_content = "let z = 5;";

        let temp_file = NamedTempFile::new().expect("Failed to create temp file");
        fs::write(temp_file.path(), ts_content).expect("Failed to write to temp file");

        process_ts_file(temp_file.path(), false, true, mock_process_script_setup);

        let result_content =
            fs::read_to_string(temp_file.path()).expect("Failed to read temp file");
        assert!(
            result_content.contains("processed:"),
            "Verbose mode should still apply changes"
        );
    }
}
