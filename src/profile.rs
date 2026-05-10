use crate::config::{self, ProfileEntry, ProfileHooks};
use colored::Colorize;
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;
use std::process::Command;

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

fn add_file(name: &str, entries: &mut Vec<ProfileEntry>) -> anyhow::Result<()> {
    loop {
        let input = prompt(&format!("  {} ", "add file:".dimmed()));
        if input.is_empty() {
            break;
        }

        let expanded = expand_tilde(&input);

        if expanded.symlink_metadata().is_err() {
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
    Ok(())
}

fn run_hooks(hooks: &ProfileHooks) {
    if let Some(cmds) = &hooks.post_apply {
        println!("\n{}", "Running hooks:".bold());
        for cmd in cmds {
            println!("  {} {}", "$".dimmed(), cmd.cyan());
            let status = Command::new("sh").arg("-c").arg(cmd).status();
            match status {
                Ok(s) if s.success() => println!("  {}", "✓".green()),
                Ok(s) => println!("  {} exited with {}", "⚠".yellow(), s),
                Err(e) => println!("  {} failed to run: {}", "✗".red(), e),
            }
        }
    }
}

pub fn create(name: &str) -> anyhow::Result<()> {
    let mut cfg = config::load()?;

    if cfg.profiles.contains_key(name) {
        anyhow::bail!("profile \"{}\" already exists", name);
    }

    println!("{} {}", "Creating profile".bold(), name.cyan().bold());
    println!("Enter config file paths one at a time. Press {} to finish.\n", "Enter".dimmed());

    let mut entries: Vec<ProfileEntry> = Vec::new();
    add_file(name, &mut entries)?;

    if entries.is_empty() {
        println!("\n{} no files added, profile not saved.", "!".yellow());
        return Ok(());
    }

    println!("\n{}", "Add post-apply hooks (commands to run after applying). Press Enter to finish.".dimmed());
    let mut hooks: Vec<String> = Vec::new();
    loop {
        let cmd = prompt(&format!("  {} ", "command:".dimmed()));
        if cmd.is_empty() {
            break;
        }
        hooks.push(cmd);
    }

    cfg.profiles.insert(name.to_string(), entries.clone());
    if !hooks.is_empty() {
        cfg.hooks.insert(name.to_string(), ProfileHooks { post_apply: Some(hooks) });
    }
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

    let hooks = cfg.hooks.get(name).cloned().unwrap_or_default();

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

    run_hooks(&hooks);

    Ok(())
}

pub fn edit(name: &str) -> anyhow::Result<()> {
    let mut cfg = config::load()?;

    if !cfg.profiles.contains_key(name) {
        anyhow::bail!("no profile named \"{}\"", name);
    }

    loop {
        println!("\n{} {}\n", "Editing profile".bold(), name.cyan().bold());
        println!("  {} add a file", "[1]".cyan());
        println!("  {} remove a file", "[2]".cyan());
        println!("  {} rename profile", "[3]".cyan());
        println!("  {} manage hooks", "[4]".cyan());
        println!("  {} quit\n", "[q]".dimmed());

        let choice = prompt(&format!("{} ", ">".dimmed()));

        match choice.as_str() {
            "1" => {
                let entries = cfg.profiles.get_mut(name).unwrap();
                add_file(name, entries)?;
                config::save(&cfg)?;
            }
            "2" => {
                let entries = cfg.profiles.get(name).unwrap().clone();
                if entries.is_empty() {
                    println!("  {} no files in this profile", "!".yellow());
                    continue;
                }
                println!();
                for (i, e) in entries.iter().enumerate() {
                    println!("  [{}] {} {} {}", i + 1, e.alias.cyan(), "→".dimmed(), e.original.dimmed());
                }
                let input = prompt("\n  remove #: ");
                if let Ok(n) = input.parse::<usize>() {
                    if n >= 1 && n <= entries.len() {
                        let removed = cfg.profiles.get_mut(name).unwrap().remove(n - 1);
                        let stored = stored_path(name, &removed.alias);
                        if stored.exists() {
                            fs::remove_file(&stored)?;
                        }
                        println!("  {} removed {}", "✓".green(), removed.alias.cyan());
                        config::save(&cfg)?;
                    } else {
                        println!("  {} invalid number", "⚠".yellow());
                    }
                }
            }
            "3" => {
                let new_name = prompt(&format!("  new name [{}]: ", name.cyan()));
                if new_name.is_empty() || new_name == name {
                    continue;
                }
                if cfg.profiles.contains_key(&new_name) {
                    println!("  {} profile \"{}\" already exists", "⚠".yellow(), new_name);
                    continue;
                }
                let entries = cfg.profiles.remove(name).unwrap();
                let hooks = cfg.hooks.remove(name);
                let profile_dir = config::profiles_dir().join(name);
                let new_profile_dir = config::profiles_dir().join(&new_name);
                if profile_dir.exists() {
                    fs::rename(&profile_dir, &new_profile_dir)?;
                }
                cfg.profiles.insert(new_name.clone(), entries);
                if let Some(h) = hooks {
                    cfg.hooks.insert(new_name.clone(), h);
                }
                if cfg.active.as_deref() == Some(name) {
                    cfg.active = Some(new_name.clone());
                }
                config::save(&cfg)?;
                println!("  {} renamed to {}", "✓".green(), new_name.cyan());
                break;
            }
            "4" => {
                let hooks = cfg.hooks.entry(name.to_string()).or_default();
                let existing = hooks.post_apply.clone().unwrap_or_default();
                println!("\n  {} post-apply hooks:", "current".dimmed());
                if existing.is_empty() {
                    println!("  {}", "none".dimmed());
                } else {
                    for (i, cmd) in existing.iter().enumerate() {
                        println!("  [{}] {}", i + 1, cmd.cyan());
                    }
                }
                println!("\n  {} add  {} remove #  {} clear", "[a]".cyan(), "[r]".cyan(), "[c]".cyan());
                let action = prompt(&format!("  {} ", ">".dimmed()));
                match action.as_str() {
                    "a" => {
                        let cmd = prompt("  command: ");
                        if !cmd.is_empty() {
                            hooks.post_apply.get_or_insert_with(Vec::new).push(cmd.clone());
                            println!("  {} added: {}", "✓".green(), cmd.cyan());
                        }
                    }
                    "c" => {
                        hooks.post_apply = None;
                        println!("  {} hooks cleared", "✓".green());
                    }
                    s if s.starts_with('r') => {
                        let n: usize = s[1..].trim().parse().unwrap_or(0);
                        let cmds = hooks.post_apply.get_or_insert_with(Vec::new);
                        if n >= 1 && n <= cmds.len() {
                            let removed = cmds.remove(n - 1);
                            println!("  {} removed: {}", "✓".green(), removed.cyan());
                        } else {
                            println!("  {} invalid number", "⚠".yellow());
                        }
                    }
                    _ => {}
                }
                config::save(&cfg)?;
            }
            "q" | "" => break,
            _ => println!("  {} unknown option", "⚠".yellow()),
        }
    }

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
        if let Some(hooks) = cfg.hooks.get(profile_name) {
            if let Some(cmds) = &hooks.post_apply {
                for cmd in cmds {
                    println!("    {} {}", "hook:".dimmed(), cmd.dimmed());
                }
            }
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
    cfg.hooks.remove(name);
    if cfg.active.as_deref() == Some(name) {
        cfg.active = None;
    }
    config::save(&cfg)?;

    println!("{} profile {} deleted.", "✓".green().bold(), name.cyan().bold());
    Ok(())
}
