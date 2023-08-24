Note that ffetch is currently under development. If you want to test, follow these steps:

1. Clone Repo
```
git clone https://github.com/0l3d/ffetch.git
```

2. Enter to folder
```
cd ffetch
```

3. Dependencies
```
Rust
Cargo
```

4. Build
```
cargo build
```

5. Installation
```
cargo install --path .z
```

6. Copying files
```
mkdir ~/.config/ffetch
cp -r ascii_arts/ ~/.config/ffetch/ascii_arts
cp config.yml ~/.config/ffetch/
sudo cp ffetch/target/debug/ffetch /bin/ffetch
```

7. Read Config File
Config file in `~/.config/ffetch/config.yml`. Change user to your username in first line.

8. Launch ffetch
Type `ffetch` in terminal.
