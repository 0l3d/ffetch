#!/bin/bash
REPO="https://github.com/0l3d/ffetch.git"
DIR="ffetch"

echo "Select installation method:"
echo "1) Build from source (Git version)"
echo "2) Download release binary"
echo -n "Choice [1-2]: "
read install_choice

case "$install_choice" in
  1)
    echo "Select build profile:"
    echo "1) Balanced (Balanced size & performance)"
    echo "2) Size (Size optimized)"
    echo "3) Performance (Performance optimized)"
    echo -n "Choice [1-3]: "
    read choice

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
    BINARY_PATH="$(pwd)/target/$profile_dir/ffetch"
    ;;

  2)
    echo "Downloading latest release..."

    DOWNLOAD_URL=$(curl -s https://api.github.com/repos/0l3d/ffetch/releases/latest | grep "browser_download_url.*linux.tar.gz" | cut -d '"' -f 4)

    if [ -z "$DOWNLOAD_URL" ]; then
      echo "Error: Could not find release download URL"
      exit 1
    fi

    echo "Downloading from: $DOWNLOAD_URL"

    curl -L "$DOWNLOAD_URL" -o ffetch-linux.tar.gz
    tar -xzf ffetch-linux.tar.gz

    chmod +x ffetch

    echo "Download complete. Binary: $(pwd)/ffetch"
    BINARY_PATH="$(pwd)/ffetch"

    rm ffetch-linux.tar.gz
    ;;

  *)
    echo "Invalid choice"
    exit 1
    ;;
esac

echo ""
echo "Setting up configuration..."
mkdir -p ~/.config/ffetch

echo "Select configuration profile:"
echo "1) Advanced"
echo "2) Middle"
echo "3) Minimal"
echo -n "Choice [1-3]: "
read config_choice

case "$config_choice" in
  1) CONFIG_FILE="ffetch-advanced.conf" ;;
  2) CONFIG_FILE="ffetch-middle.conf" ;;
  3) CONFIG_FILE="ffetch-minimal.conf" ;;
  *) echo "Invalid choice, using middle config"; CONFIG_FILE="ffetch-middle.conf" ;;
esac

if [ -f "$CONFIG_FILE" ]; then
  cp "$CONFIG_FILE" ~/.config/ffetch/ffetch.conf
  echo "Configuration copied: $CONFIG_FILE -> ~/.config/ffetch/ffetch.conf"
else
  echo "Warning: Configuration file $CONFIG_FILE not found"
fi

if [ -d "ascii" ]; then
  echo ""
  echo "Select ASCII art:"
  ascii_files=(ascii/*.txt)
  if [ ${#ascii_files[@]} -gt 0 ] && [ -f "${ascii_files[0]}" ]; then
    for i in "${!ascii_files[@]}"; do
      filename=$(basename "${ascii_files[i]}" .txt)
      echo "$((i+1))) $filename"
    done
    echo -n "Choice [1-${#ascii_files[@]}]: "
    read ascii_choice

    if [[ "$ascii_choice" =~ ^[0-9]+$ ]] && [ "$ascii_choice" -ge 1 ] && [ "$ascii_choice" -le "${#ascii_files[@]}" ]; then
      selected_ascii="${ascii_files[$((ascii_choice-1))]}"
      cp "$selected_ascii" ~/.config/ffetch/ascii.txt
      echo "ASCII art copied: $(basename "$selected_ascii") -> ~/.config/ffetch/ascii.txt"
    else
      echo "Invalid choice, skipping ASCII setup"
    fi
  else
    echo "No ASCII files found in ascii/ directory"
  fi
else
  echo "ASCII directory not found, skipping ASCII setup"
fi

echo ""
echo -n "Move ffetch to /usr/local/bin? [y/N]: "
read move_binary

case "$move_binary" in
  [Yy]|[Yy][Ee][Ss])
    sudo cp "$BINARY_PATH" /usr/local/bin/ffetch
    echo "ffetch moved to /usr/local/bin/"
    echo ""
    echo "Installation complete! You can now run: ffetch"
    ;;
  *)
    echo ""
    echo "Installation complete!"
    echo "Binary location: $BINARY_PATH"
    echo "You can run: $BINARY_PATH"
    echo "Or move it manually: sudo cp $BINARY_PATH /usr/local/bin/"
    ;;
esac
