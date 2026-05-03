program Build;

{$mode objfpc}{$H+}

uses
  SysUtils, Process;

var
  ProjectRoot: string;
  TargetDir: string;
  Cargo: TProcess;

begin
  ProjectRoot := IncludeTrailingPathDelimiter(ExtractFilePath(ExpandFileName(ParamStr(0))));
  TargetDir := ProjectRoot + 'target';

  if not SetEnvironmentVariable('CARGO_TARGET_DIR', TargetDir) then
  begin
    WriteLn(StdErr, 'failed to set CARGO_TARGET_DIR');
    Halt(1);
  end;

  Cargo := TProcess.Create(nil);
  try
    Cargo.Executable := 'cargo';
    Cargo.Parameters.Add('build');
    Cargo.Parameters.Add('--locked');
    Cargo.CurrentDirectory := ProjectRoot;
    Cargo.Options := [poWaitOnExit];
    Cargo.Execute;
    Halt(Cargo.ExitStatus);
  finally
    Cargo.Free;
  end;
end.
