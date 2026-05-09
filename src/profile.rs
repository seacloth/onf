use crate::config::{self, ProfileEntry};
use colored::Colorize;
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;

fn expand_tilde(path: &str) -> PathBuf {
    if let Some(rest) = path.strip_prefix("~/") {
        dirs::home_dir()
            .expect("could not find home directory")
            .join(rest)
    } else if path == "~" {
        dirs::home_dir().expect("could not find home directory")
    } else {
        PathBuf::from(path)
    }
}

fn stored_path(profile: &str, alias: &str) -> PathBuf {
    config::profiles_dir().join(profile).join(alias)
}

fn prompt(msg: &str) -> String {
    print!("{}", msg);
    io::stdout().flush().unwrap();
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.trim().to_string()
}

fn default_alias(path: &std::path::Path) -> String {
    path.file_name()
        .unwrap_or_default()
        .to_string_lossy()
        .into_owned()
}

pub fn create(name: &str) -> anyhow::Result<()> {
    let mut cfg = config::load()?;

    if cfg.profiles.contains_key(name) {
        anyhow::bail!("profile \"{}\" already exists", name);
    }

    println!("{} {}", "Creating profile".bold(), name.cyan().bold());
    println!("Enter config file paths one at a time. Press {} to finish.\n", "Enter".dimmed());

    let mut entries: Vec<ProfileEntry> = Vec::new();

    loop {
        let input = prompt(&format!("  {} ", "add file:".dimmed()));
        if input.is_empty() {
            break;
        }

        let expanded = expand_tilde(&input);

        if !expanded.exists() {
            println!("  {} file not found, skipping: {}", "⚠".yellow(), expanded.display());
            continue;
        }

        let suggested = default_alias(&expanded);
        let alias_input = prompt(&format!(
            "  {} [{}]: ",
            "profile name".dimmed(),
            suggested.dimmed()
        ));
        let alias = if alias_input.is_empty() {
            suggested
        } else {
            alias_input
        };

        let dest = stored_path(name, &alias);
        fs::create_dir_all(dest.parent().unwrap())?;
        fs::copy(&expanded, &dest)?;

        println!("  {} saved as {}\n", "✓".green(), alias.cyan());

        entries.push(ProfileEntry {
            original: expanded.to_string_lossy().into_owned(),
            alias,
        });
    }

    if entries.is_empty() {
        println!("\n{} no files added, profile not saved.", "!".yellow());
        return Ok(());
    }

    cfg.profiles.insert(name.to_string(), entries.clone());
    config::save(&cfg)?;

    println!(
        "\n{} profile {} saved with {} file(s). Run {} to activate.",
        "✓".green().bold(),
        name.cyan().bold(),
        entries.len(),
        format!("onf apply {}", name).bold()
    );
    Ok(())
}

pub fn apply(name: &str) -> anyhow::Result<()> {
    let mut cfg = config::load()?;

    let entries = cfg
        .profiles
        .get(name)
        .ok_or_else(|| anyhow::anyhow!("no profile named \"{}\"", name))?
        .clone();

    println!("{} {}\n", "Applying profile".bold(), name.cyan().bold());

    for entry in &entries {
        let original = PathBuf::from(&entry.original);
        let stored = stored_path(name, &entry.alias);

        if !stored.exists() {
            println!("  {} stored copy missing for {}, skipping", "⚠".yellow(), entry.alias);
            continue;
        }

        if original.exists() || original.symlink_metadata().is_ok() {
            fs::remove_file(&original)?;
        }

        if let Some(parent) = original.parent() {
            fs::create_dir_all(parent)?;
        }

        #[cfg(unix)]
        std::os::unix::fs::symlink(&stored, &original)?;

        #[cfg(windows)]
        std::os::windows::fs::symlink_file(&stored, &original)?;

        println!(
            "  {} {} {} {}",
            "→".blue(),
            entry.alias.cyan(),
            "→".dimmed(),
            original.display()
        );
    }

    cfg.active = Some(name.to_string());
    config::save(&cfg)?;

    println!("\n{} active profile is now {}", "✓".green().bold(), name.cyan().bold());
    Ok(())
}

pub fn list() -> anyhow::Result<()> {
    let cfg = config::load()?;

    if cfg.profiles.is_empty() {
        println!("{}", "no profiles yet. Run `onf new <name>` to create one.".dimmed());
        return Ok(());
    }

    println!("{}\n", "Profiles:".bold());
    for (profile_name, entries) in &cfg.profiles {
        let active_marker = if cfg.active.as_deref() == Some(profile_name) {
            format!(" {}", "(active)".green())
        } else {
            String::new()
        };
        println!("  {}{}", profile_name.cyan().bold(), active_marker);
        for e in entries {
            println!("    {} {} {}", e.alias.cyan(), "→".dimmed(), e.original.dimmed());
        }
    }
    Ok(())
}

pub fn status() -> anyhow::Result<()> {
    let cfg = config::load()?;

    match &cfg.active {
        Some(name) => {
            println!("{} {}", "Active profile:".bold(), name.cyan().bold());
            if let Some(entries) = cfg.profiles.get(name) {
                for e in entries {
                    println!("  {} {} {}", e.alias.cyan(), "→".dimmed(), e.original.dimmed());
                }
            }
        }
        None => println!("{}", "No active profile.".dimmed()),
    }
    Ok(())
}

pub fn delete(name: &str) -> anyhow::Result<()> {
    let mut cfg = config::load()?;

    if !cfg.profiles.contains_key(name) {
        anyhow::bail!("no profile named \"{}\"", name);
    }

    let confirm = prompt(&format!(
        "Delete profile {}? This removes stored copies. [y/N] ",
        name.cyan().bold()
    ));

    if confirm.to_lowercase() != "y" {
        println!("Aborted.");
        return Ok(());
    }

    let profile_dir = config::profiles_dir().join(name);
    if profile_dir.exists() {
        fs::remove_dir_all(&profile_dir)?;
    }

    cfg.profiles.remove(name);
    if cfg.active.as_deref() == Some(name) {
        cfg.active = None;
    }
    config::save(&cfg)?;

    println!("{} profile {} deleted.", "✓".green().bold(), name.cyan().bold());
    Ok(())
}
