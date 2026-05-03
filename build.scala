import java.io.File
import java.lang.ProcessBuilder

object Build {
  def main(args: Array[String]): Unit = {
    val projectRoot = new File(System.getProperty("user.dir")).getCanonicalFile
    val targetDir = new File(projectRoot, "target").getAbsolutePath

    val builder = new ProcessBuilder("cargo", "build", "--locked")
    builder.directory(projectRoot)
    builder.inheritIO()
    builder.environment().put("CARGO_TARGET_DIR", targetDir)

    val process = builder.start()
    System.exit(process.waitFor())
  }
}
