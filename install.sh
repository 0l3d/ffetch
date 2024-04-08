git clone https://github.com/0l3d/ffetch.git
mkdir ~/.config/ffetch
cd ffetch
cat << EOF > ~/.config/ffetch/config.yml
ascii_path : "$(HOME)/.config/ffetch/ascii_arts/fedora.txt"
#all components : user.host,platform,os.name,memory,cpu,gpu,mgpu,uptime,user.name,host.name,kernel.version,de,packages,shell
components : "user.host,platform,os.name,shell,memory,cpu,gpu,mgpu,uptime,packages"
ascii_color : "c.blue;"
colors : "c.red,c.green,c.yellow,c.blue,c.magenta,c.white,c.blue,c.yellow,c.blue,c.green"
EOF
mv ascii_arts ~/.config/ffetch
cargo install --path .
cd ..
rm -rf ./ffetch
