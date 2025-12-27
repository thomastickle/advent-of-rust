use std::fs;
use std::process::Command;
use std::path::Path;

fn main() {
    let readme_path = "readme.md";
    let mut content = fs::read_to_string(readme_path).expect("Unable to read readme.md");

    for day in 1..=25 {
        let day_str = format!("{:02}", day);
        let file_path = format!("src/day{}.rs", day_str);
        
        let status = if Path::new(&file_path).exists() {
            println!("Testing Day {}...", day_str);
            let test_output = Command::new("cargo")
                .args(["test", &format!("day{}", day_str)])
                .output()
                .expect("Failed to execute cargo test");

            if test_output.status.success() {
                "✅"
            } else {
                "❌"
            }
        } else {
            "⬜"
        };

        let lines: Vec<String> = content.lines().map(|line| {
            if line.contains(&format!("Day {}:", day)) || (line.contains(&format!("Day {}", day)) && !line.contains(':')) {
                if let Some(pos) = line.find("Day") {
                    let prefix = &line[..pos];
                    if prefix.contains("- ") {
                        return format!("- {} {}", status, &line[pos..]);
                    }
                }
            }
            line.to_string()
        }).collect();
        
        content = lines.join("\n") + "\n";
    }

    fs::write(readme_path, content.trim_end().to_string() + "\n").expect("Unable to write to readme.md");
    println!("Progress updated in readme.md");
}
