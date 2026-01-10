use std::{env, error, fmt, str::FromStr};
use tracing::debug;

pub fn init() -> Result<(), Box<dyn error::Error>> {
    let app_env = env::var("APP_ENV")
        .ok()
        .map(|v| v.parse())
        .transpose()?
        .unwrap_or(AppEnv::Dev);

    let files = vec![
        ".env".to_string(),
        format!(".env.{app_env}"),
        ".env.local".to_string(),
    ];

    // Load each .env file (silent fail if not found)
    for file in files {
        if let Ok(iter) = dotenvy::from_path_iter(&file) {
            debug!("Loaded environment variables from {file}:");

            for item in iter {
                let (key, value) = item?;
                debug!("\t{key} = {value}");

                unsafe {
                    env::set_var(&key, &value);
                }
            }
        }
    }

    Ok(())
}

#[derive(Debug)]
enum AppEnv {
    Dev,
    Prod,
}

impl fmt::Display for AppEnv {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppEnv::Dev => write!(f, "dev"),
            AppEnv::Prod => write!(f, "prod"),
        }
    }
}

impl FromStr for AppEnv {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "dev" => Ok(Self::Dev),
            "prod" => Ok(Self::Prod),
            s => Err(format!("Invalid APP_ENV: {s}. Must be 'dev' or 'prod'")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::fs;
    use std::path::PathBuf;
    use tempfile::TempDir;

    // Helper to create a temporary directory and change to it
    struct TestEnv {
        _temp_dir: TempDir,
        original_dir: PathBuf,
    }

    impl TestEnv {
        fn new() -> Self {
            let temp_dir = TempDir::new().unwrap();
            let original_dir = env::current_dir().unwrap();
            env::set_current_dir(temp_dir.path()).unwrap();

            Self {
                _temp_dir: temp_dir,
                original_dir,
            }
        }

        fn create_file(&self, name: &str, content: &str) {
            fs::write(name, content).unwrap();
        }
    }

    impl Drop for TestEnv {
        fn drop(&mut self) {
            env::set_current_dir(&self.original_dir).unwrap();
        }
    }

    // Helper to clear specific env vars
    fn clear_env_vars(vars: &[&str]) {
        for var in vars {
            unsafe {
                env::remove_var(var);
            }
        }
    }

    #[test]
    fn test_loads_basic_env_file() {
        let test_env = TestEnv::new();
        clear_env_vars(&["APP_ENV", "TEST_VAR"]);

        test_env.create_file(".env", "TEST_VAR=hello");

        init().unwrap();

        assert_eq!(env::var("TEST_VAR").unwrap(), "hello");
    }

    #[test]
    fn test_loads_dev_env_by_default() {
        let test_env = TestEnv::new();
        clear_env_vars(&["APP_ENV", "DEV_VAR", "BASE_VAR"]);

        test_env.create_file(".env", "BASE_VAR=base");
        test_env.create_file(".env.dev", "DEV_VAR=development");

        init().unwrap();

        assert_eq!(env::var("BASE_VAR").unwrap(), "base");
        assert_eq!(env::var("DEV_VAR").unwrap(), "development");
    }

    #[test]
    fn test_loads_prod_env_when_specified() {
        let test_env = TestEnv::new();
        clear_env_vars(&["APP_ENV", "PROD_VAR", "BASE_VAR"]);

        unsafe {
            env::set_var("APP_ENV", "prod");
        }

        test_env.create_file(".env", "BASE_VAR=base");
        test_env.create_file(".env.prod", "PROD_VAR=production");

        init().unwrap();

        assert_eq!(env::var("BASE_VAR").unwrap(), "base");
        assert_eq!(env::var("PROD_VAR").unwrap(), "production");
    }

    #[test]
    fn test_loads_local_env_last() {
        let test_env = TestEnv::new();
        clear_env_vars(&["APP_ENV", "OVERRIDE_VAR"]);

        test_env.create_file(".env", "OVERRIDE_VAR=base");
        test_env.create_file(".env.dev", "OVERRIDE_VAR=dev");
        test_env.create_file(".env.local", "OVERRIDE_VAR=local");

        init().unwrap();

        // .env.local should override previous values
        assert_eq!(env::var("OVERRIDE_VAR").unwrap(), "local");
    }

    #[test]
    fn test_env_files_cascade_correctly() {
        let test_env = TestEnv::new();
        clear_env_vars(&["APP_ENV", "VAR1", "VAR2", "VAR3"]);

        test_env.create_file(".env", "VAR1=from_base\nVAR2=from_base");
        test_env.create_file(".env.dev", "VAR2=from_dev\nVAR3=from_dev");
        test_env.create_file(".env.local", "VAR3=from_local");

        init().unwrap();

        assert_eq!(env::var("VAR1").unwrap(), "from_base");
        assert_eq!(env::var("VAR2").unwrap(), "from_dev");
        assert_eq!(env::var("VAR3").unwrap(), "from_local");
    }

    #[test]
    fn test_missing_env_files_dont_fail() {
        let _test_env = TestEnv::new();
        clear_env_vars(&["APP_ENV"]);

        // No .env files created
        let result = init();

        assert!(result.is_ok());
    }

    #[test]
    fn test_invalid_app_env_returns_error() {
        let _test_env = TestEnv::new();
        clear_env_vars(&["APP_ENV"]);

        unsafe {
            env::set_var("APP_ENV", "invalid");
        }

        let result = init();

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Invalid APP_ENV"));
    }

    #[test]
    fn test_app_env_case_sensitive() {
        let _test_env = TestEnv::new();
        clear_env_vars(&["APP_ENV"]);

        unsafe {
            env::set_var("APP_ENV", "Dev");
        }

        let result = init();

        assert!(result.is_err());
    }

    #[test]
    fn test_multiline_values() {
        let test_env = TestEnv::new();
        clear_env_vars(&["APP_ENV", "MULTILINE"]);

        test_env.create_file(".env", "MULTILINE='line1\nline2\nline3'");

        init().unwrap();

        let value = env::var("MULTILINE").unwrap();
        assert!(value.contains("line1"));
    }

    #[test]
    fn test_empty_values() {
        let test_env = TestEnv::new();
        clear_env_vars(&["APP_ENV", "EMPTY_VAR"]);

        test_env.create_file(".env", "EMPTY_VAR=");

        init().unwrap();

        assert_eq!(env::var("EMPTY_VAR").unwrap(), "");
    }

    #[test]
    fn test_values_with_equals_signs() {
        let test_env = TestEnv::new();
        clear_env_vars(&["APP_ENV", "URL"]);

        test_env.create_file(".env", "URL=https://example.com?param=value");

        init().unwrap();

        assert_eq!(env::var("URL").unwrap(), "https://example.com?param=value");
    }

    #[test]
    fn test_comments_are_ignored() {
        let test_env = TestEnv::new();
        clear_env_vars(&["APP_ENV", "VAR1"]);

        test_env.create_file(".env", "# This is a comment\nVAR1=value\n# Another comment");

        init().unwrap();

        assert_eq!(env::var("VAR1").unwrap(), "value");
    }

    #[test]
    fn test_appenv_display_trait() {
        assert_eq!(AppEnv::Dev.to_string(), "dev");
        assert_eq!(AppEnv::Prod.to_string(), "prod");
    }

    #[test]
    fn test_appenv_from_str_valid() {
        assert!(matches!("dev".parse::<AppEnv>(), Ok(AppEnv::Dev)));
        assert!(matches!("prod".parse::<AppEnv>(), Ok(AppEnv::Prod)));
    }

    #[test]
    fn test_appenv_from_str_invalid() {
        let result: Result<AppEnv, _> = "staging".parse();
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(
            err,
            "Invalid APP_ENV: staging. Must be 'dev' or 'prod'"
        );
    }
}
