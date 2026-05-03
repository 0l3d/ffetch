#!/usr/bin/env escript
%%! -noshell

-mode(compile).

main(_) ->
    ProjectRoot = filename:absname(filename:dirname(escript:script_name())),
    TargetDir = filename:join(ProjectRoot, "target"),

    case file:set_cwd(ProjectRoot) of
        ok ->
            run_cargo(TargetDir);
        {error, Reason} ->
            io:format("failed to change directory to ~s: ~p~n", [ProjectRoot, Reason]),
            halt(1)
    end.

run_cargo(TargetDir) ->
    case os:find_executable("cargo") of
        false ->
            io:format("cargo not found~n", []),
            halt(127);
        Cargo ->
            Port = open_port(
                {spawn_executable, Cargo},
                [
                    {args, ["build", "--locked"]},
                    {env, [{"CARGO_TARGET_DIR", TargetDir}]},
                    binary,
                    exit_status,
                    stderr_to_stdout,
                    use_stdio
                ]
            ),
            wait_for_cargo(Port)
    end.

wait_for_cargo(Port) ->
    receive
        {Port, {data, Data}} ->
            io:put_chars(binary_to_list(Data)),
            wait_for_cargo(Port);
        {Port, {exit_status, Status}} ->
            halt(Status)
    end.
