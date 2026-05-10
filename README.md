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

or just install the latest release

```bash
cd onf
cargo install --path .
```

---

## commands

| command | description |
|---|---|
| `onf new <profile>` | create a profile interactively |
| `onf apply <profile>` | symlink a profile's files into place |
| `onf edit <profile>` | add/remove files, rename, or manage hooks |
| `onf list` | show all profiles |
| `onf status` | show the active profile |
| `onf delete <profile>` | delete a profile |

---

## flow

```
$ onf new <profile name>
Creating profile <profile name>
Enter config file paths one at a time. Press Enter to finish.

  add file: ~/.config/polybar/colors.ini
  profile name [colors.ini]: colors.ini
  ✓ saved as colors.ini

  add file: ~/.config/nitrogen/bg-saved.cfg
  profile name [bg-saved.cfg]: bg-saved.cfg
  ✓ saved as bg-saved.cfg

  add file:

Add post-apply hooks (commands to run after applying). Press Enter to finish.
  command: nitrogen --restore
  command: killall polybar && polybar &
  command:

✓ profile <profile name> saved with 1 file(s). Run onf apply <profile name> to activate.
```

```
$ onf apply <profile name>
Applying profile <profile name>

  → colors.ini → /home/ciao/.config/polybar/colors.ini

  → bg-saved.cfg → /home/ciao/.config/nitrogen/bg-saved.cfg

✓ active profile is now <profile name>

Running hooks:
  $ nitrogen --restore
  ✓
  $ killall polybar && polybar &
  ✓
```

```
$ onf edit <profile name>
Editing profile <profile name>

  [1] add a file
  [2] remove a file
  [3] rename profile
  [4] manage hooks
  [q] quit
```

```
$ onf list
Profiles:

  <profile name> (active)
    colors.ini → /home/ciao/.config/polybar/colors.ini
    bg-saved.cfg → /home/ciao/.config/nitrogen/bg-saved.cfg
    hook: nitrogen --restore
    hook: killall polybar && polybar &
```

---

## how it works

- `onf new` snapshots your files into `~/.config/onf/profiles/<name>/`
- each file gets a friendly alias so the store stays readable
- `onf apply` removes the originals and replaces them with symlinks to the stored copies
- hooks run automatically after applying — good for `nitrogen --restore`, restarting polybar, applying gtk themes, etc.
- state lives in `~/.config/onf/config.toml` — plain toml, always readable

---

## built with

- [`clap`](https://github.com/clap-rs/clap) — cli
- [`serde`](https://serde.rs) + [`toml`](https://github.com/toml-rs/toml) — config
- [`dirs`](https://github.com/dirs-dev/dirs-rs) — platform paths
- [`colored`](https://github.com/colored-rs/colored) — terminal output
