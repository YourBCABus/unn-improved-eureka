pub mod logging {
    use arcs_logging_rs::with_target;
    with_target! { "Webhook" }
    
    pub struct Shortened<'a>(&'a str, bool);
    pub fn shortened(string: &str, max_len: usize) -> Shortened {
        let (display_name, shortened) =  if string.chars().count() >= max_len {
            if let Some((idx, _)) = string.char_indices().nth(max_len-3) {
                (&string[..idx], true)
            } else { (string, false) }
        } else { (string, false) };
    
        Shortened(display_name, shortened)
    }
    
    impl<'a> std::fmt::Display for Shortened<'a> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.0)?;
            if self.1 {
                write!(f, "...")
            } else {
                Ok(())
            }
        }
    }
    impl<'a> std::fmt::Debug for Shortened<'a> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "`{self}`")
        }
    }
}

pub mod env {
    use arcs_env_rs::*;

    env_var_req!(PORT);
    
    assert_req_env!(
        check_env_vars:
            PORT
    );

    pub mod sql {
        use arcs_env_rs::*;

        env_var_req!(SQL_DB_NAME -> DB_NAME);
        // env_var_req!(SQL_DB_PASS -> DB_PASS);

        env_var_req!(SQL_USERNAME -> USERNAME);

        env_var_req!(DATABASE_URL -> DB_URL);

        assert_req_env!(
            check_env_vars:
                DB_NAME, // DB_PASS,
                USERNAME,
                DB_URL
        );
    }

    pub mod sheets {
        use arcs_env_rs::*;

        env_var_req!(SHEET_INTEGRATION_TOKEN -> TOKEN);
        env_var_req!(SHEET_INTEGRATION_ID -> ID);

        assert_req_env!(
            check_env_vars:
                TOKEN,
                ID
        );
    }

    pub mod checks {
        pub use super::check_env_vars as main;
        pub use super::sql::check_env_vars as sql;
        pub use super::sheets::check_env_vars as sheets;
    }
}
