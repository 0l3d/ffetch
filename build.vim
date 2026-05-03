let s:project_root = expand('<sfile>:p:h')
let $CARGO_TARGET_DIR = s:project_root . '/target'

execute 'cd' fnameescape(s:project_root)
!cargo build --locked

if v:shell_error
  cquit v:shell_error
endif

quitall!
