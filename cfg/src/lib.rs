use arcs_env_rs::*;

env_var_req!(PORT);
env_var_req!(COMPLEXITY -> GRAPHQL_COMPLEXITY_LIMIT);

/// Get the port to bind to from the environment variables
/// 
/// # Panics
/// 
/// Panics if the port is not a valid u16
/// - `> 65535`
/// - `< 0`
/// - not a number
pub async fn port_u16_panic() -> u16 {
    let port = port();
    let Ok(port) = port.parse() else {
        logging::report!("Failed to parse port as u16": { "port": port });
        logging::error!("Failed to parse port as u16");
        logging::debug!("Port: {:#?}", port);
        panic!("Failed to parse port as u16");
    };
    port
}

pub async fn graphql_complexity_limit_usize_panic() -> usize {
    let complexity = graphql_complexity_limit();
    let Ok(complexity) = complexity.parse() else {
        logging::report!("Failed to parse graphql complexity as usize": { "complexity": complexity });
        logging::error!("Failed to parse graphql complexity as usize");
        logging::debug!("Complexity: {:#?}", complexity);
        panic!("Failed to parse complexity as usize");
    };
    complexity
}

assert_req_env!(
    check_env_vars:
        PORT, GRAPHQL_COMPLEXITY_LIMIT
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

pub mod checks {
    pub use super::check_env_vars as main;
    pub use super::sql::check_env_vars as sql;
}
