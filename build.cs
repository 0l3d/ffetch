using System;
using System.Diagnostics;
using System.IO;
using System.Runtime.CompilerServices;

static class Build
{
    static int Main(string[] args)
    {
        _ = args;

        string? projectRoot = FindRepoRoot();
        if (projectRoot is null)
        {
            Console.Error.WriteLine("failed to find repository root containing Cargo.toml");
            return 1;
        }

        ProcessStartInfo startInfo = new("cargo", "build --locked")
        {
            WorkingDirectory = projectRoot,
            UseShellExecute = false
        };
        startInfo.Environment["CARGO_TARGET_DIR"] = Path.Combine(projectRoot, "target");

        using Process process = Process.Start(startInfo) ?? throw new InvalidOperationException("failed to start cargo");
        process.WaitForExit();
        return process.ExitCode;
    }

    static string? FindRepoRoot([CallerFilePath] string sourcePath = "")
    {
        foreach (string candidate in new[]
        {
            AppContext.BaseDirectory,
            Directory.GetCurrentDirectory(),
            Path.GetDirectoryName(sourcePath) ?? ""
        })
        {
            string? root = FindRepoRootFrom(candidate);
            if (root is not null)
            {
                return root;
            }
        }

        return null;
    }

    static string? FindRepoRootFrom(string start)
    {
        if (string.IsNullOrWhiteSpace(start))
        {
            return null;
        }

        DirectoryInfo? dir = new(start);
        while (dir is not null)
        {
            if (File.Exists(Path.Combine(dir.FullName, "Cargo.toml")))
            {
                return dir.FullName;
            }

            dir = dir.Parent;
        }

        return null;
    }
}
