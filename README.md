# onf

> A simple way to swap your configurations.

onf is a minimal dotfile profile switcher written in Rust. define named profiles, each owning a set of config files. one command and everything's symlinked into place.

---

## install

```bash
git clone https://github.com/seacloth/onf
cd onf
cargo install --path .
```

---

## commands

| command | description |
|---|---|
| `onf new <profile>` | create a profile interactively |
| `onf apply <profile>` | symlink a profile's files into place |
| `onf list` | show all profiles |
| `onf status` | show the active profile |
| `onf delete <profile>` | delete a profile |

---

## flow

```
$ onf new work
Creating profile "work"

  add file: ~/.zshrc       ✓
  add file: ~/.gitconfig   ✓
  add file:

✓ profile "work" saved with 2 files.
```

```
$ onf apply work

  → .zshrc
  → .gitconfig

✓ active profile is now "work"
```

```
$ onf status
Active profile: work
  /home/ciao/.zshrc
  /home/ciao/.gitconfig
```

---

## how it works

- `onf new` snapshots your files into `~/.config/onf/profiles/<name>/`
- `onf apply` removes the originals and replaces them with symlinks to the stored copies
- state lives in `~/.config/onf/config.toml` — plain toml, always readable

---

## built with

- [`clap`](https://github.com/clap-rs/clap) — cli
- [`serde`](https://serde.rs) + [`toml`](https://github.com/toml-rs/toml) — config
- [`dirs`](https://github.com/dirs-dev/dirs-rs) — platform paths
- [`colored`](https://github.com/colored-rs/colored) — terminal output
