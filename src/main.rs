use std::fs;

use clap::Parser;
use html::HTMLTag;

mod html;

fn main() {
    let args = Args::parse();

    let head = generate_head();

    fs::write(args.output_file, head.to_string())
        .unwrap_or_else(|_| panic!("Could not write {} to file", head))
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    output_file: String,

    #[arg(short, long)]
    text_node_count: i32,
}

fn generate_head() -> HTMLTag {
    let title = HTMLTag::new("title", "generated page");

    HTMLTag::new("head", &title.to_string())
}
