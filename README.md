# onf

> a simple way to swap your configurations.

onf is a minimal configuration switcher written in Rust.

---

### install

```bash
git clone https://github.com/seacloth/onf
cd onf
cargo install --path .
```

---

### commands

| command | description |
|---|---|
| `onf new <profile>` | create a profile interactively |
| `onf apply <profile>` | symlink a profile's files into place |
| `onf edit <profile>` | add/remove files, rename, or manage hooks |
| `onf list` | show all profiles |
| `onf status` | show the active profile |
| `onf delete <profile>` | delete a profile |

---

### what it does

- `onf new` snapshots into `~/.config/onf/profiles/<name>/`
- each file gets a friendly alias so the store stays readable
- `onf apply` removes the originals and replaces them with symlinks to the stored copies, also using hooks to execute commands with the applying configs
- state lives in `~/.config/onf/config.toml` — plain toml, readable

---

### built with

- [`clap`](https://github.com/clap-rs/clap) — cli
- [`serde`](https://serde.rs) + [`toml`](https://github.com/toml-rs/toml) — config
- [`dirs`](https://github.com/dirs-dev/dirs-rs) — platform paths
- [`colored`](https://github.com/colored-rs/colored) — terminal output
