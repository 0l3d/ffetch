cat << EOF > ~/.config/ffetch/config.yml
ascii_path : "/home/$(whoami)/.config/ffetch/ascii_arts/debian.txt"
#all components : user.host,platform,os.name,memory,cpu,uptime,user.name,host.name,kernel.version,de,packages,gpu,shell
components : "user.host,platform,os.name,shell,memory,cpu,gpu,uptime,packages"
EOF
