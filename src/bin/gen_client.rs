use std::io::Write;

use improved_eureka::verification::{id_secret::generate_client_keystr, scopes::Scopes};
use rpassword::read_password;

fn main() {
    print!("Enter the client secret (not shown): ");
    std::io::stdout().flush().unwrap();

    let mut secret = read_password().unwrap().trim().to_string();
    let mut secret_was_generated = false;
    if secret.is_empty() {
        use rand::{ Rng, SeedableRng };
        use base64::{
            engine::{ GeneralPurpose, GeneralPurposeConfig, Engine },
            alphabet::STANDARD,
        };

        let secret_bytes_32 = rand::rngs::StdRng::from_entropy().gen::<[u8; 32]>(); // 32-byte secret
        let secret_bytes_16 = rand::rngs::StdRng::from_entropy().gen::<[u8; 16]>(); // 16-byte secret
        let secret_bytes: Vec<_> = secret_bytes_32.into_iter().chain(secret_bytes_16).collect();

        let encoder = GeneralPurpose::new(&STANDARD, GeneralPurposeConfig::default());
        secret = encoder.encode(secret_bytes);
        secret_was_generated = true;
    }
    let client_id = uuid::Uuid::new_v4();

    let mut scopes = Scopes::none();
    loop {
        print!("Enter a scope or scopes (comma separated, empty to finish): ");
        std::io::stdout().flush().unwrap();
        let mut new_scope = String::new();
        std::io::stdin().read_line(&mut new_scope).unwrap();
        if new_scope.trim().is_empty() {
            break;
        }
        let new_scope = new_scope.trim().to_lowercase().replace(", ", " ");

        println!();
        let Some(new_scope) = Scopes::try_from_str(&new_scope) else {
            println!(
                "Invalid scope(s). (Valid scopes: {})",
                Scopes::all().to_string().replace(' ', ", "),
            );
            continue;
        };
        println!();
        
        scopes = scopes | new_scope;
    }

    print!("Enter the client name (example `read.ios`): ");
    std::io::stdout().flush().unwrap();
    let mut name = String::new();
    std::io::stdin().read_line(&mut name).unwrap();
    let name = name.trim();

    let keystr = generate_client_keystr(secret.as_bytes()).unwrap();

    println!("Run the following SQL query:");
    println!();
    println!("INSERT INTO clients (id, client_key, scopes, description)");
    println!("VALUES ('{}', '{keystr}', '{}', '{name}');", client_id.as_hyphenated(), scopes.to_string());
    println!();
    println!();
    println!("Then, add the following to the .env file:");
    println!();
    println!("CLIENT_ID='{}';", client_id.as_hyphenated());
    if secret_was_generated {
        println!("CLIENT_SECRET='{}';", secret);
    } else {
        println!("CLIENT_SECRET=<what you just pasted in>;");
    }
    println!();
    println!();
    println!("Finally, post this to discord:");
    println!();
    println!("### Client {name}");
    println!("Hash: {keystr}");
    println!("Client ID: `{}`", client_id.as_hyphenated());
    println!("Client Secret: ||`{secret}`||");
    println!("Scopes: {}", scopes.to_string());

}
