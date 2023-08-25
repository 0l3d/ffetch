curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
1
git clone https://github.com/0l3d/ffetch.git
mkdir ~/.config/ffetch
cd ffetch
mv config.yml ~/.config/ffetch
mv ascii_arts ~/.config/ffetch
cargo install
cd ..
rm -rf ./ffetch