git clone https://github.com/0l3d/ffetch.git
mkdir ~/.config/ffetch
cd ffetch
cat << EOF > ~/.config/ffetch/config.yml
<<<<<<< HEAD
ascii_path : "/home/$(whoami)/.config/ffetch/ascii_arts/linux.txt"
=======
ascii_path : "/home/$(whoami)/.config/ffetch/ascii_arts/afplusplus.txt"
>>>>>>> 4bef7bc (Update installer script)
#all components : user.host,platform,os.name,memory,cpu,uptime,user.name,host.name,kernel.version,de,packages,gpu,shell
components : "user.host,platform,os.name,shell,memory,cpu,gpu,uptime,packages"
EOF
mv ascii_arts ~/.config/ffetch
sudo mv ./release/ffetch /usr/bin/
cd ..
rm -rf ./ffetch
<<<<<<< HEAD

echo "Installed!"
=======
>>>>>>> 4bef7bc (Update installer script)
