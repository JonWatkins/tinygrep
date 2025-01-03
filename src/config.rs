use crate::error::ApplicationError;

#[derive(Debug, PartialEq)]
pub struct Config {
    pub query: String,
    pub file_paths: Vec<String>,
    pub ignore_case: bool,
    pub show_line_numbers: bool,
    pub use_regex: bool,
    pub enable_highlighting: bool,
    pub read_from_stdin: bool,
    pub recursive_search: bool,
}

impl Config {
    pub fn build(args: impl Iterator<Item = String>) -> Result<Config, ApplicationError> {
        let mut ignore_case = false;
        let mut show_line_numbers = false;
        let mut use_regex = false;
        let mut enable_highlighting = false;
        let mut recursive_search = false;
        let mut query = String::new();
        let mut file_paths = Vec::new();
        let mut args_iter = args.skip(1);

        while let Some(arg) = args_iter.next() {
            match arg.as_str() {
                "-h" | "--help" => return Err(ApplicationError::HelpRequested),
                "-i" | "--ignore-case" => ignore_case = true,
                "-n" | "--line-numbers" => show_line_numbers = true,
                "-R" | "--recursive" => recursive_search = true,
                "-r" | "--use-regex" => use_regex = true,
                "-c" | "--color" => enable_highlighting = true,
                _ => {
                    if arg.starts_with('-') {
                        return Err(ApplicationError::InvalidFlag(arg.to_string()));
                    }

                    if query.is_empty() {
                        query = arg.to_string();
                    } else {
                        file_paths.push(arg.to_string());
                    }
                }
            }
        }

        let read_from_stdin = file_paths.is_empty();

        if query.is_empty() {
            return Err(ApplicationError::NotEnoughArguments);
        }

        Ok(Config {
            query,
            file_paths,
            ignore_case,
            show_line_numbers,
            use_regex,
            enable_highlighting,
            read_from_stdin,
            recursive_search,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_config() {
        let args = vec![
            "minigrep".to_string(),
            "rust".to_string(),
            "poem.txt".to_string(),
        ];

        let config = Config::build(args.into_iter()).unwrap();
        assert_eq!(config.query, "rust");
        assert_eq!(config.file_paths, vec!["poem.txt".to_string()]);
        assert!(!config.ignore_case);
        assert!(!config.show_line_numbers);
        assert!(!config.use_regex);
        assert!(!config.enable_highlighting);
    }

    #[test]
    fn test_config_with_flags() {
        let args = vec![
            "minigrep".to_string(),
            "-i".to_string(),
            "-n".to_string(),
            "-r".to_string(),
            "-c".to_string(),
            "rust".to_string(),
            "poem.txt".to_string(),
        ];

        let config = Config::build(args.into_iter()).unwrap();
        assert_eq!(config.query, "rust");
        assert_eq!(config.file_paths, vec!["poem.txt".to_string()]);
        assert!(config.ignore_case);
        assert!(config.show_line_numbers);
        assert!(config.use_regex);
        assert!(config.enable_highlighting);
    }

    #[test]
    fn test_config_with_long_flags() {
        let args = vec![
            "minigrep".to_string(),
            "--ignore-case".to_string(),
            "--line-numbers".to_string(),
            "--use-regex".to_string(),
            "--color".to_string(),
            "rust".to_string(),
            "poem.txt".to_string(),
        ];

        let config = Config::build(args.into_iter()).unwrap();
        assert_eq!(config.query, "rust");
        assert_eq!(config.file_paths, vec!["poem.txt".to_string()]);
        assert!(config.ignore_case);
        assert!(config.show_line_numbers);
        assert!(config.use_regex);
        assert!(config.enable_highlighting);
    }

    #[test]
    fn test_invalid_flag() {
        let args = vec![
            "minigrep".to_string(),
            "rust".to_string(),
            "--unknown".to_string(),
            "poem.txt".to_string(),
        ];

        let result = Config::build(args.into_iter());

        assert!(
            matches!(result, Err(ApplicationError::InvalidFlag(ref flag)) if flag == "--unknown"),
            "Expected InvalidFlag error with '--unknown', but got {:?}",
            result
        );
    }

    #[test]
    fn test_not_enough_arguments() {
        let args = vec!["minigrep".to_string()];
        let result = Config::build(args.into_iter());
        assert!(
            matches!(result, Err(ApplicationError::NotEnoughArguments)),
            "Expected NotEnoughArguments error, but got {:?}",
            result
        );
    }

    #[test]
    fn test_missing_query_or_file_paths() {
        let args = vec!["minigrep".to_string(), "-i".to_string()];
        let result = Config::build(args.into_iter());
        assert!(
            matches!(result, Err(ApplicationError::NotEnoughArguments)),
            "Expected NotEnoughArguments error, but got {:?}",
            result
        );
    }

    #[test]
    fn test_help_requested() {
        let args = vec!["minigrep".to_string(), "--help".to_string()];
        let result = Config::build(args.into_iter());
        assert!(
            matches!(result, Err(ApplicationError::HelpRequested)),
            "Expected HelpRequested error, but got {:?}",
            result
        );
    }
}
