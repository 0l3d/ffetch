#include <limits.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/wait.h>
#include <unistd.h>

static int project_root_from_argv(const char *argv0, char *out, size_t out_len) {
    char resolved[PATH_MAX];

    if (strchr(argv0, '/') == NULL) {
        if (getcwd(out, out_len) == NULL) {
            return 0;
        }
        return 1;
    }

    if (realpath(argv0, resolved) == NULL) {
        return 0;
    }

    char *slash = strrchr(resolved, '/');
    if (slash == NULL) {
        return 0;
    }

    *slash = '\0';
    if (snprintf(out, out_len, "%s", resolved) >= (int)out_len) {
        return 0;
    }

    return 1;
}

int main(int argc, char **argv) {
    (void)argc;

    char project_root[PATH_MAX];
    char target_dir[PATH_MAX];

    if (!project_root_from_argv(argv[0], project_root, sizeof(project_root))) {
        fprintf(stderr, "failed to resolve project root\n");
        return 1;
    }

    if (snprintf(target_dir, sizeof(target_dir), "%s/target", project_root) >= (int)sizeof(target_dir)) {
        fprintf(stderr, "target path is too long\n");
        return 1;
    }

    if (setenv("CARGO_TARGET_DIR", target_dir, 1) != 0) {
        perror("setenv");
        return 1;
    }

    if (chdir(project_root) != 0) {
        perror("chdir");
        return 1;
    }

    int status = system("cargo build --locked");
    if (status == -1) {
        perror("system");
        return 1;
    }

    if (WIFEXITED(status)) {
        return WEXITSTATUS(status);
    }

    return 1;
}
