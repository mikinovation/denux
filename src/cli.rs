use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "nuxt-auto-import-replacer")]
#[command(about = "Replace auto-imports in Nuxt with explicit imports", long_about = None)]
pub struct Cli {
    /// 対象ディレクトリ (デフォルト: ./src)
    #[arg(short, long, default_value = "src")]
    pub target: PathBuf,

    /// Dry-run: 変更を適用せずに表示のみ行う
    #[arg(short, long)]
    pub dry_run: bool,

    /// 詳細ログを出力する
    #[arg(short, long)]
    pub verbose: bool,
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Parser;

    #[test]
    fn test_cli_default_values() {
        let args = Cli::parse_from(["nuxt-auto-import-replacer"]);

        assert_eq!(args.target, PathBuf::from("src"));
        assert!(!args.dry_run);
        assert!(!args.verbose);
    }

    #[test]
    fn test_cli_with_arguments() {
        let args = Cli::parse_from([
            "nuxt-auto-import-replacer",
            "--target",
            "my_project",
            "--dry-run",
            "--verbose",
        ]);

        assert_eq!(args.target, PathBuf::from("my_project"));
        assert!(args.dry_run);
        assert!(args.verbose);
    }

    #[test]
    fn test_cli_with_short_options() {
        let args = Cli::parse_from(["nuxt-auto-import-replacer", "-t", "my_project", "-d", "-v"]);

        assert_eq!(args.target, PathBuf::from("my_project"));
        assert!(args.dry_run);
        assert!(args.verbose);
    }
}
