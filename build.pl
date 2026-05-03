#!/usr/bin/env perl

use strict;
use warnings;
use Cwd qw(abs_path);
use File::Basename qw(dirname);
use File::Spec;

my $project_root = dirname(abs_path(__FILE__));
$ENV{"CARGO_TARGET_DIR"} = File::Spec->catdir($project_root, "target");

chdir $project_root or die "failed to change directory to $project_root: $!\n";

system("cargo", "build", "--locked");
my $status = $?;

if ($status == -1) {
    die "failed to start cargo: $!\n";
}

exit($status >> 8);
