import java.io.File;
import java.io.IOException;
import java.util.Map;

class Build {
    public static void main(String[] args) throws IOException, InterruptedException {
        File projectRoot = new File(System.getProperty("user.dir")).getCanonicalFile();
        File targetDir = new File(projectRoot, "target");

        ProcessBuilder builder = new ProcessBuilder("cargo", "build", "--locked");
        builder.directory(projectRoot);
        builder.inheritIO();

        Map<String, String> env = builder.environment();
        env.put("CARGO_TARGET_DIR", targetDir.getAbsolutePath());

        Process process = builder.start();
        System.exit(process.waitFor());
    }
}
