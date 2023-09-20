use clap::{Parser, Subcommand};
use git2::Repository;
use std::env;
use std::process::Command;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// open home page in browser
    Home { name: Option<String> },
}

fn main() {
    let cli = Cli::parse();

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Commands::Home { name } => {
            // 获取当前目录
            let current_dir = env::current_dir().expect("Failed to get current directory");

            // 打开 Git 仓库
            let repo = Repository::open(current_dir).expect("Failed to open repository");

            // 获取远程 URL
            let remote = repo
                .find_remote("origin")
                .expect("Failed to find remote 'origin'");
            let remote_url = remote
                .url()
                .expect("Failed to get remote URL");

            // 转换为 HTTP/HTTPS URL
            let http_http_url = if remote_url.starts_with("git@") {
                // 如果是 SSH URL，则转换为 HTTP/HTTPS URL
                let url_parts: Vec<&str> = remote_url.split(':').collect();
                let host = url_parts[0].split('@').collect::<Vec<&str>>()[1];
                let path = url_parts[1].split('.').collect::<Vec<&str>>()[0];
                //trim .git
                let path = path.trim_end_matches(".git");
                let http_url = format!("https://{}/{}", host, path);
                http_url
            } else {
                // 否则，已经是 HTTP/HTTPS URL
                remote_url.to_string()
            };

            // 使用浏览器打开 URL
            webbrowser::open(&*http_http_url).unwrap();
        }
    }
}
