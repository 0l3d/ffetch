# F-Fetch
F-Fetch targets low systems. Written in Rust. It's very simple, designed so you can pick it up and replace it.

## First Look 

~/.config/ffetch/config.yaml
```yml
ascii_path : "/home/user/.config/ffetch/ascii_arts/debian.txt"
#all components : user.host,platform,os.name,memory,cpu,uptime,user.name,host.name,kernel.version,de,packages
components : "user.host,platform,os.name,memory,cpu,uptime,packages"
```
Supported distros : Debian, Fedora and Arch Linux


Example
```sh
       _,met$$$$$gg.            xold@xold
    ,g$$$$$$$$$$$$$$$P.         Platform :          Linux
  ,g$$P"        """Y$$.".       OS Name :           Debian
 ,$$P'              `$$$.       Memory :            9536 / 12441 MB
',$$P       ,ggs.     `$$b:     CPU :               Intel(R) blabla | x86_64
`d$$'     ,$P"'   .    $$$      Uptime :            2 hours, 56 minutes
 $$P      d$'     ,    $$P      Packages :          855
 $$:      $$.   -    ,d$$'      GPU :               Intel blabla
 $$;      Y$b._   _,d$P'        
 Y$$.    `.`"Y$$$$P"'           
 `$$b      "-.__                
  `Y$$                          
   `Y$$.                        
     `$$b.                      
       `Y$$b.                   
          `"Y$b._               
              `"""              
```

* If you are using wm, it will say Unknown in the desktop section.

It is far from colors and visuals! Focused on function only!
