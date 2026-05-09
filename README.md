# onf

> a simple way to swap your configurations.

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
$ onf new gruvbox
Creating profile gruvbox
Enter config file paths one at a time. Press Enter to finish.

  add file: ~/.config/polybar/colors.ini
  profile name [colors.ini]: polybar-colors.ini
  ✓ saved as polybar-colors.ini

  add file:

✓ profile gruvbox saved with 1 file(s). Run onf apply gruvbox to activate.
```

```
$ onf apply gruvbox
Applying profile gruvbox

  → polybar-colors.ini → /home/ciao/.config/polybar/colors.ini

✓ active profile is now gruvbox
```

```
$ onf list
Profiles:

  gruvbox (active)
    polybar-colors.ini → /home/ciao/.config/polybar/colors.ini
```

---

## how it works

- `onf new` snapshots your files into `~/.config/onf/profiles/<name>/`
- each file gets a friendly alias so the store stays readable
- `onf apply` removes the originals and replaces them with symlinks to the stored copies
- state lives in `~/.config/onf/config.toml` — plain toml, always readable

---

## built with

- [`clap`](https://github.com/clap-rs/clap) — cli
- [`serde`](https://serde.rs) + [`toml`](https://github.com/toml-rs/toml) — config
- [`dirs`](https://github.com/dirs-dev/dirs-rs) — platform paths
- [`colored`](https://github.com/colored-rs/colored) — terminal output
