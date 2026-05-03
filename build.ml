#!/usr/bin/env ocaml
#load "unix.cma";;

let project_root =
  let script =
    if Filename.is_relative Sys.argv.(0) then
      Filename.concat (Sys.getcwd ()) Sys.argv.(0)
    else Sys.argv.(0)
  in
  Filename.dirname script

let () =
  Unix.putenv "CARGO_TARGET_DIR" (Filename.concat project_root "target");
  Sys.chdir project_root;
  exit (Sys.command "cargo build --locked")
