## Rusty-starter

Rusty-starter is a Rust-based application designed to manage and execute profiles defined in the `config.toml` file, offering enhanced startup application management for Windows.

### Key Features

- **Profile Management**: Select from a variety of profiles defined in `config.toml`.
- **Path Execution**: Execute commands specified within each profile.
- **User-Friendly Interface**: Utilizes fuzzy selection for intuitive profile management.

### Example `config.toml` which should be in the same directory as your executable.
```
[[profiles]]
profile_name = "Explorer & Firefox"
paths = ["C:/Windows/explorer.exe", "C:/Program Files/Mozilla Firefox/firefox.exe"]

[[profiles]]
profile_name = "Explorer"
paths = ["C:/Windows/explorer.exe"]

[[profiles]]
profile_name = "Firefox"
paths = ["C:/Program Files/Mozilla Firefox/firefox.exe"]
```

### Tested on & works on:
Windows 11, 
NixOS
