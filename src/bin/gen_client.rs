use std::io::Write;

use improved_eureka::verification::id_secret::generate_client_keystr;
use rpassword::read_password;

fn main() {
    print!("Enter the client secret (not shown): ");
    std::io::stdout().flush().unwrap();

    let secret = read_password().unwrap();
    let client_id = uuid::Uuid::new_v4();

    let keystr = generate_client_keystr(secret.as_bytes()).unwrap();

    println!("Run the following SQL query:");
    println!();
    println!("INSERT INTO clients (id, client_key)");
    println!("VALUES ('{}', '{}');", client_id.as_hyphenated(), keystr);
    println!();
    println!();
    println!("Then, add the following to the .env file:");
    println!();
    println!("CLIENT_ID='{};", client_id.as_hyphenated());
    println!("CLIENT_SECRET=<what you just pasted in>;");
}
