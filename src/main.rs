use clap::{Parser, Subcommand};
use rusqlite::{params, Connection, Result as SqlResult};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Parser)] 
#[command(name ="mira", about="Your CLI companion")]
struct Cli{
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Talk to Mira
    Chat {
        /// Your message
        message: String,
    },
    /// Show recent conversations
    Memory,
    /// Wipe Mira's memory
    Forget,
    /// Show your identity card
    Who,
}

fn db_path()->String{
    "mira.db".to_string()
}

fn open_db()->SqlResult<Connection>{
    let conn=Connection::open(db_path())?;
    conn.pragma_update(None,"journal_mode","wal")?;
    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS episodes (

            id        INTEGER PRIMARY KEY,

            message   TEXT NOT NULL,

            timestamp TEXT NOT NULL

        );

        CREATE TABLE IF NOT EXISTS facts (

            key   TEXT PRIMARY KEY,

            value TEXT NOT NULL

        );"
    )?;
    Ok(conn)
}

fn now_timestamp()-> String{
    let duration=SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let secs=duration.as_secs();
    let hours=(secs/3600)%24;
    let minutes=(secs/60)%60;
    let seconds=secs%60;
    format!("{:02}:{:02}:{:02}",hours,minutes,seconds)
}

fn handle_chat(conn: &Connection, message:&str)->SqlResult<()>{
    let timestamp=now_timestamp();
    conn.execute(
        "INSERT INTO episodes (message, timestamp) VALUES (?1, ?2)",
        &[message, &timestamp],
    )?;
    extract_facts(conn, message)?;
    let response=generate_response(conn, message)?;
    println!("Mira: {}",response);
    Ok(())
}
fn extract_facts(conn: &Connection, message: &str) -> SqlResult<()> {
    let lower = message.to_lowercase();

    // Check for known patterns and store the extracted value
    if let Some(name) = extract_after(&lower, "my name is ") {
        upsert_fact(conn, "user_name", &name)?;
    }
        if let Some(color) = extract_after(&lower, "my favorite color is ") {
        upsert_fact(conn, "user_color", &color)?;
    }
    if let Some(city) = extract_after(&lower, "i live in ") {
        upsert_fact(conn, "user_city", &city)?;
    }
    if let Some(mood) = extract_after(&lower, "i'm feeling ") {
        upsert_fact(conn, "user_mood", &mood)?;
    }
    if let Some(mood) = extract_after(&lower, "im feeling ") {
        upsert_fact(conn, "user_mood", &mood)?;
    }

    Ok(())
}

fn extract_after(text: &str, prefix: &str) -> Option<String> {
    // Find the prefix in the text, then grab everything after it
    if let Some(start) = text.find(prefix) {
        let after = &text[start + prefix.len()..];
        let value = after.split(|c: char| c == '.' || c == ',' || c == '!')
            .next()
            .unwrap_or("")
            .trim()
            .to_string();
        if !value.is_empty() {
            return Some(value);
        }
    }
    None
}

fn get_fact(conn: &Connection, key: &str) -> SqlResult<Option<String>> {
    // Query a single fact by key, returning None if it doesn't exist
    let result = conn.query_row(
        "SELECT value FROM facts WHERE key = ?1",
        params![key],
        |row| row.get(0),
    );
    match result {
        Ok(val) => Ok(Some(val)),
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
        Err(e) => Err(e),
    }
}

fn generate_response(conn: &Connection, message: &str) -> SqlResult<String> {
    let lower = message.to_lowercase();

    // Check if the user is asking about themselves
    if lower.contains("who am i") {
        if let Some(name) = get_fact(conn, "user_name")? {
            return Ok(format!("You're {}! I remember.", name));
        } else {
            return Ok("I don't know your name yet. Tell me!".to_string());
        }
    }

    if lower.contains("where do i live") {
        if let Some(city) = get_fact(conn, "user_city")? {
            return Ok(format!("You live in {}!", city));
        } else {
            return Ok("You haven't told me where you live yet.".to_string());
        }
    }

    // Default to a friendly hardcoded response
    let responses = [
        "Hmm, interesting. Tell me more.",
        "Got it. I'll remember that.",
        "Okay! What else?",
        "Noted. Anything else on your mind?",
        "I see. Keep talking, I'm listening.",
    ];

    let index = message.len() % responses.len();
    Ok(responses[index].to_string())
}

fn upsert_fact(conn: &Connection, key: &str, value: &str) -> SqlResult<()> {
    // INSERT OR REPLACE overwrites the old value if the key already exists
    conn.execute(
        "INSERT OR REPLACE INTO facts (key, value) VALUES (?1, ?2)",
        params![key, value],
    )?;
    Ok(())
}

fn handle_memory(conn: &Connection) -> SqlResult<()> {
    // Prepare a query for the 5 most recent episodes
    let mut stmt = conn.prepare(
        "SELECT message, timestamp FROM episodes ORDER BY id DESC LIMIT 5"
    )?;

    // Iterate over each row returned by the query
    let rows = stmt.query_map([], |row| {
        let message: String = row.get(0)?;
        let timestamp: String = row.get(1)?;
        Ok((message, timestamp))
    })?;

    // Print each conversation entry with its timestamp
    let mut found = false;
    for row in rows {
        let (message, timestamp) = row?;
        if !found {
            println!("\u{1f4dd} Recent conversations:");
            println!();
            found = true;
        }
        println!("  [{}] {}", timestamp, message);
    }

    // Handle the empty case
    if !found {
        println!("No memories yet. Start chatting!");
    }

    Ok(())
}

fn handle_forget(conn: &Connection) -> SqlResult<()> {
    // Delete all conversation history
    conn.execute("DELETE FROM episodes", [])?;
    // Delete all stored facts
    conn.execute("DELETE FROM facts", [])?;
    println!("\u{1f9f9} Memory wiped. Fresh start!");
    Ok(())
}

fn handle_who(conn: &Connection) -> SqlResult<()> {
    let mut stmt = conn.prepare("SELECT key, value FROM facts ORDER BY key")?;
    let rows = stmt.query_map([], |row| {
        let key: String = row.get(0)?;
        let value: String = row.get(1)?;
        Ok((key, value))
    })?;

    let facts: Vec<(String, String)> = rows.filter_map(|r| r.ok()).collect();

    if facts.is_empty() {
        println!("I don't know anything about you yet!");
        return Ok(());
    }

    let label_map: Vec<(&str, &str)> = vec![
        ("user_name", "Name"),
        ("user_city", "City"),
        ("user_mood", "Mood"),
        ("user_color", "Color"),
    ];

    let mut lines: Vec<String> = Vec::new();
    for (key, value) in &facts {
        let label = label_map.iter()
            .find(|(k, _)| k == key)
            .map(|(_, l)| *l)
            .unwrap_or(key.as_str());
        lines.push(format!("  {}: {}", label, value));
    }

    let max_width = lines.iter().map(|l| l.len()).max().unwrap_or(20);
    let width = max_width.max(20) + 2;

    println!("╭{}╮", "─".repeat(width));
    println!("│{:^width$}│", "🧠 Mira knows...");
    println!("├{}┤", "─".repeat(width));
    for line in &lines {
        println!("│{:<width$}│", line);
    }
    println!("╰{}╯", "─".repeat(width));

    Ok(())
}
fn main() {
    let cli = Cli::parse();

    // Open the database connection, exit if it fails
    let conn = match open_db() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Failed to open database: {}", e);
            std::process::exit(1);
        }
    };

        let result = match cli.command {
        Commands::Chat { message } => handle_chat(&conn, &message),
        Commands::Memory => handle_memory(&conn),
        Commands::Forget => handle_forget(&conn),
        Commands::Who => handle_who(&conn),
    };

    if let Err(e) = result {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}