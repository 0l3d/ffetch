#!/bin/bash
REPO="https://github.com/0l3d/ffetch.git"
DIR="ffetch"

echo "Select installation method:"
echo "1) Build from source (Git version)"
echo "2) Download release binary"
read -rp "Choice [1-2]: " install_choice

case "$install_choice" in
  1)
    echo "Select build profile:"
    echo "1) Balanced (Balanced size & performance)"
    echo "2) Size (Size optimized)"
    echo "3) Performance (Performance optimized)"
    read -rp "Choice [1-3]: " choice

    if [ -d "$DIR" ]; then
      cd "$DIR"
      git pull
    else
      git clone "$REPO"
      cd "$DIR"
    fi

    case "$choice" in
      1) cargo build --release ;;
      2) cargo build --profile size ;;
      3) cargo build --profile performance ;;
      *) echo "Invalid choice"; exit 1 ;;
    esac

    profile_dir=""
    if [ "$choice" -eq 1 ]; then
      profile_dir="release"
    elif [ "$choice" -eq 2 ]; then
      profile_dir="size"
    elif [ "$choice" -eq 3 ]; then
      profile_dir="performance"
    fi

    echo "Build done. Binary: $(pwd)/target/$profile_dir/ffetch"
    ;;

  2)
    # Check if curl is available
    if ! command -v curl &> /dev/null; then
      echo "Error: curl is required but not installed"
      echo "Please install curl: sudo apt install curl"
      exit 1
    fi

    echo "Downloading latest release..."

    # Get latest release download URL
    DOWNLOAD_URL=$(curl -s https://api.github.com/repos/0l3d/ffetch/releases/latest | grep "browser_download_url.*linux.tar.gz" | cut -d '"' -f 4)

    if [ -z "$DOWNLOAD_URL" ]; then
      echo "Error: Could not find release download URL"
      exit 1
    fi

    echo "Downloading from: $DOWNLOAD_URL"

    # Download and extract
    curl -L "$DOWNLOAD_URL" -o ffetch-linux.tar.gz
    tar -xzf ffetch-linux.tar.gz

    # Make executable
    chmod +x ffetch

    echo "Download complete. Binary: $(pwd)/ffetch"
    echo "You can move it to your PATH: sudo mv ffetch /usr/local/bin/"

    # Cleanup
    rm ffetch-linux.tar.gz
    ;;

  *)
    echo "Invalid choice"
    exit 1
    ;;
esac
