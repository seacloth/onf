# onf

> a simple way to swap your configurations.

onf is a minimal configuration switcher written in Rust.

---

> onf is updated quite regularly since its a personal project!

---

### install

```bash
cargo install onf
```

---

### commands

| command | description |
|---|---|
| `onf new <profile>` | create a profile interactively |
| `onf apply <profile>` | symlink a profile's files into place |
| `onf restore` | put your real files back and deactivate the profile |
| `onf copy <from> <to>` | duplicate a profile including its files and hooks |
| `onf export <profile> <path>` | bundle a profile into a tar.gz |
| `onf import <path>` | import a previously exported profile |
| `onf edit <profile>` | add/remove files, rename, or manage hooks |
| `onf list` | show all profiles |
| `onf status` | show the active profile |
| `onf delete <profile>` | delete a profile |
 
both `onf apply` and `onf restore` accept a `--dry-run` flag to preview changes without touching anything.

---

### what it does

- `onf new` snapshots into `~/.config/onf/profiles/<name>/`
- each file gets a friendly alias so the store stays readable
- `onf apply` removes the originals and replaces them with symlinks to the stored copies, also using hooks to execute commands with the applying configs
- `onf restore` puts your real files back and leaves no active profile
- state lives in `~/.config/onf/config.toml` — plain toml, readable

---

### built with

- [`clap`](https://github.com/clap-rs/clap) — cli
- [`serde`](https://serde.rs) + [`toml`](https://github.com/toml-rs/toml) — config
- [`dirs`](https://github.com/dirs-dev/dirs-rs) — platform paths
- [`colored`](https://github.com/colored-rs/colored) — terminal output
- [`flate2`](https://github.com/rust-lang/flate2-rs) + [`tar`](https://github.com/alexcrichton/tar-rs) — export/import
