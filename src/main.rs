use clap::Parser;
use std::process::Command;
use serde::Deserialize;
use serde_json::Result;
use std::str;
mod cal;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    /// A valid Github username. Tries to guess
    username: Option<String>,
    
    /// Disable colour. Respects NO_COLOR by default
    #[clap(long, short)]
    no_colour: bool,

    /// See raw levels
    #[clap(long, short)]
    raw: bool,
}

#[derive(Deserialize)]
struct GithubApiResponse {
    login: String,
}

fn main() -> Result<()> {
    let args = Cli::parse();

    match &args.username {
        Some(username) => cal::cal(username, !args.no_colour, args.raw).unwrap(),
        None => {
            if let Ok(gh) = Command::new("gh").args(["api", "user"]).output() {
                let text = String::from_utf8_lossy(&gh.stdout);

                if text != "" {
                    let resp: GithubApiResponse = serde_json::from_str(&text)?;

                    cal::cal(&resp.login, !args.no_colour, args.raw).unwrap();
                } else {
                    eprintln!("Github-cli is not authenticated. Try --help?")
                }
            } else {
                eprintln!("Failed to guess your github username. Try --help?")
            }
        }
    };

    Ok(())
}
