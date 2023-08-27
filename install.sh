git clone https://github.com/0l3d/ffetch.git
mkdir ~/.config/ffetch
cd ffetch
cat << EOF > ~/.config/ffetch/config.yml
ascii_path : "/home/$(whoami)/.config/ffetch/ascii_arts/fedora.txt"
#all components : user.host,platform,os.name,memory,cpu,uptime,user.name,host.name,kernel.version,de,packages,gpu,shell
components : "user.host,platform,os.name,shell,memory,cpu,gpu,uptime,packages"
ascii_color : "c.blue;"
colors : "c.red,c.green,c.yellow,c.blue,c.magenta,c.white,c.white,c.yellow,c.green"
EOF
mv ascii_arts ~/.config/ffetch
cargo install --path .
cd ..
rm -rf ./ffetch
