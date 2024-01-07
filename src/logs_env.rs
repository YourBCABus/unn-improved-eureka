//! This crate is just for organizing `ARCS`-related crates that should
//! eventually be migrated over to more general libraries
//! 
//! See [`logging`] and [`env`]

#[allow(unused_macros)]
pub mod logging {
    //! Logging-related.
    //! 
    //! Contains:
    //! - macros for general logging:
    //!     - [`trace`]
    //!     - [`debug`]
    //!     - [`info`]
    //!     - [`warn`]
    //!     - [`error`]
    //! - [`shortened`] for displayable shortened strings
    //! 
    //! Usually you should just import all of it with
    //! ```no_run
    //! use crate::logging::*;
    //! ```

    use arcs_logging_rs::with_target;
    with_target! { "TableJet Improved Eureka" }
    
    /// Display struct for [`shortened`]
    pub struct Shortened<'a>(&'a str, bool);
    /// Get a version of a string which can be capped at a certain number of characters
    /// 
    /// This function is relatively fault-tolerant, and will default to the full
    /// string if it can't shorten it correctly.
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
            write!(f, "`{:?}", self.0)?;
            if self.1 {
                write!(f, "...")?;
            }
            write!(f, "`")
        }
    }


    pub struct SmallId(pub Option<&'static str>, pub uuid::Uuid);

    impl std::fmt::Display for SmallId {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "<")?;
            if let Some(prefix) = self.0 {
                write!(f, "{prefix}:")?;
            }
            write!(f, "{:08x}>", self.1.as_fields().0)
        }
    }

    pub fn fmt_req_id(id: uuid::Uuid) -> SmallId {
        SmallId(Some("req"), id)
    }

}

pub mod env {

    use arcs_env_rs::*;

    env_var_req!(PORT);

    /// Get the port to bind to from the environment variables
    /// 
    /// # Panics
    /// 
    /// Panics if the port is not a valid u16
    /// - `> 65535`
    /// - `< 0`
    /// - not a number
    pub fn port_u16_panic() -> u16 {
        let port = port();
        let Ok(port) = port.parse() else {
            crate::logging::error!("Failed to parse port as u16");
            crate::logging::debug!("Port: {:#?}", port);
            panic!("Failed to parse port as u16");
        };
        port
    }
    
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

    pub mod notifications {
        use arcs_env_rs::*;

        env_var_req!(FCM_API_KEY -> FCM_API_KEY);

        assert_req_env!(
            check_env_vars:
                FCM_API_KEY
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
