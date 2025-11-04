use anyhow::Result;
use chrono::Local;
use colored::*;
use std::fs::{self, OpenOptions};
use std::io::{self, Write};
use std::process;
use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};

fn ask(prompt: &str) -> String {
    println!("\n{}", prompt.blue().bold());
    print!("{} ", ">".green());
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    input.trim().to_string()
}

struct IdeaWriter {
    file: std::fs::File,
}

impl IdeaWriter {
    fn new(filename: &str) -> Result<Self> {
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(filename)?;
        Ok(Self { file })
    }

    fn write_line(&mut self, text: &str) -> Result<()> {
        writeln!(self.file, "{}", text)?;
        Ok(())
    }

    fn section(&mut self, title: &str) -> Result<()> {
        self.write_line(&format!("\n## {}\n", title))
    }

    fn table_header(&mut self) -> Result<()> {
        self.write_line("| Aspect | Description |")?;
        self.write_line("|---------|-------------|")
    }

    fn score_header(&mut self) -> Result<()> {
        self.write_line("| Criteria | Score |")?;
        self.write_line("|-----------|--------|")
    }

    fn table_row(&mut self, key: &str, value: &str) -> Result<()> {
        self.write_line(&format!("| **{}** | {} |", key, value))
    }
}

fn main() -> Result<()> {
    let now = Local::now();
    let filename = format!("{}.md", now.format("%Y_%m_%d"));
    let running = Arc::new(AtomicBool::new(true));
    let running_ctrlc = running.clone();
    let filename_clone = filename.clone();

    ctrlc::set_handler(move || {
        if running_ctrlc.load(Ordering::SeqCst) {
            println!("\n⚠️  Interrupted! Cleaning up...");
            if let Err(e) = fs::remove_file(&filename_clone) {
                eprintln!("❌ Failed to remove file '{}': {}", filename_clone, e);
            } else {
                println!("🗑️  Removed incomplete file '{}'.", filename_clone.red());
            }
            process::exit(0);
        }
    })?;

    let mut writer = IdeaWriter::new(&filename)?;
    writer.write_line(&format!("## 🗓️ Date:\n{}\n", now.format("%Y-%m-%d")))?;

    writer.section("🧠 Step 1: Idea Prompt")?;
    let idea_prompts = vec![
        "What problem frustrated me or someone today?",
        "What can be done 10x faster or easier?",
        "What happens if I combine ___ and ___?",
        "What would this look like if it was automated?",
        "How could I do this without money or code?",
    ];

    println!("{}", "Step 1: Idea Prompt".bold().yellow());
    for (i, idea) in idea_prompts.iter().enumerate() {
        println!("  {}) {}", (i + 1).to_string().cyan(), idea);
    }

    let idea_prompt = ask("Choose one option (1-5), Default (1):");
    let selected = idea_prompt.parse::<usize>().unwrap_or(1);
    let chosen = idea_prompts.get(selected - 1).unwrap_or(&idea_prompts[0]);

    writer.write_line(&format!("{}\n", chosen))?;
    let user_answer = ask(chosen);
    writer.write_line(&user_answer)?;

    writer.section("💭 Step 2: Raw Idea")?;
    let raw_idea = ask("Write freely — don’t filter yourself.");
    writer.write_line(&raw_idea)?;

    writer.section("🔍 Step 3: Idea Breakdown")?;
    writer.table_header()?;

    let breakdowns = [
        "Problem it solves",
        "Core concept / solution",
        "Who benefits",
        "How it works (roughly)",
    ];

    for aspect in breakdowns {
        let answer = ask(aspect);
        writer.table_row(aspect, &answer)?;
    }

    writer.section("⚙️ Step 4: Feasibility & Impact Scoring")?;
    writer.score_header()?;

    let scores = [
        "Innovation (uniqueness)",
        "Feasibility (can it be done?)",
        "Impact (value or reach)",
        "Personal doability (can *I* pursue it?)",
    ];

    let mut total = 0;
    for s in scores {
        let val = ask(&format!("{} (1–10):", s)).parse::<i32>().unwrap_or(0);
        total += val;
        writer.table_row(s, &val.to_string())?;
    }

    writer.write_line(&format!("\n**Total Score**: {} / 40", total))?;
    running.store(false, Ordering::SeqCst);

    println!(
        "\n✅ {} {}",
        "Idea saved successfully to".green().bold(),
        filename.underline()
    );

    Ok(())
}
