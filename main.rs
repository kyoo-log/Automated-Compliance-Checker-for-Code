
use std::env;
use std::fs;
use std::path::Path;
use regex::Regex;

mod rules;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: compliance_checker <code_directory>");
        std::process::exit(1);
    }

    let directory = &args[1];
    if !Path::new(directory).is_dir() {
        eprintln!("Error: Directory does not exist");
        std::process::exit(1);
    }

    match scan_directory(directory) {
        Ok(_) => println!("Compliance check completed successfully."),
        Err(e) => eprintln!("Error: {e}"),
    }
}

fn scan_directory(directory: &str) -> Result<(), Box<dyn std::error::Error>> {
    for entry in fs::read_dir(directory)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            scan_directory(path.to_str().unwrap())?;
        } else if path.extension().unwrap_or_default() == "rs" {
            let code = fs::read_to_string(&path)?;
            rules::check_compliance(&code, path.to_str().unwrap());
        }
    }
    Ok(())
}

// Placeholder for integrating compliance rules
pub mod rules {
    use regex::Regex;

    pub fn check_compliance(code: &str, filename: &str) {
        let gdpr_rule = Regex::new(r"(?i)data\s+retention")
            .expect("Invalid GDPR regex");
        let hipaa_rule = Regex::new(r"(?i)patient|health")
            .expect("Invalid HIPAA regex");

        if gdpr_rule.is_match(code) {
            println!("Potential GDPR issue found in file: {filename}");
        }
        if hipaa_rule.is_match(code) {
            println!("Potential HIPAA issue found in file: {filename}");
        }
    }
}
