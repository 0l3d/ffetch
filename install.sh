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
      git clone --depth=1 "$REPO" "$DIR"
      cd "$DIR"
    fi

    case "$choice" in
      1) cargo build --release ;;
      2) cargo build --profile size ;;
      3) cargo build --profile performance ;;
      *) echo "Invalid build profile"; exit 1 ;;
    esac

    if [ "$choice" -eq 1 ]; then
      profile_dir="release"
    elif [ "$choice" -eq 2 ]; then
      profile_dir="size"
    elif [ "$choice" -eq 3 ]; then
      profile_dir="performance"
    fi

    BINARY_PATH="$(pwd)/target/$profile_dir/ffetch"
    echo "Build complete: $BINARY_PATH"
    cd ..
    ;;

  2)
    echo "Downloading latest release binary..."

    DOWNLOAD_URL=$(curl -s https://api.github.com/repos/0l3d/ffetch/releases/latest \
      | grep -oP '"browser_download_url": "\K[^"]*ffetch-[^"]+-linux[^"]+\.tar\.gz')

    if [ -z "$DOWNLOAD_URL" ]; then
      echo "Error: Could not find release download URL."
      exit 1
    fi

    echo "Download URL: $DOWNLOAD_URL"
    curl -L "$DOWNLOAD_URL" -o ffetch.tar.gz
    tar -xzf ffetch.tar.gz
    chmod +x ffetch
    BINARY_PATH="$(pwd)/ffetch"
    echo "Download complete: $BINARY_PATH"
    rm ffetch.tar.gz

    git clone "$REPO" "$DIR"
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
  *) CONFIG_FILE="ffetch-middle.conf" ;;
esac

if [ -f "$CONFIG_FILE" ]; then
  cp "$CONFIG_FILE" ~/.config/ffetch/ffetch.conf
elif [ -f "$DIR/$CONFIG_FILE" ]; then
  cp "$DIR/$CONFIG_FILE" ~/.config/ffetch/ffetch.conf
else
  echo "Warning: Configuration file not found"
fi

echo ""
echo "Fetching ASCII art list from remote repository..."
ASCII_LIST=$(curl -s https://api.github.com/repos/0l3d/ffetch/contents/ascii | grep '"name":' | cut -d '"' -f 4)

if [ -z "$ASCII_LIST" ]; then
  echo "No ASCII files found in remote repo."
else
  mapfile -t ascii_array <<< "$ASCII_LIST"
  for i in "${!ascii_array[@]}"; do
    name=$(basename "${ascii_array[$i]}" .txt)
    echo "$((i+1))) $name"
  done

  echo -n "Select ASCII art [1-${#ascii_array[@]}]: "
  read ascii_choice

  if [[ "$ascii_choice" =~ ^[0-9]+$ ]] && [ "$ascii_choice" -ge 1 ] && [ "$ascii_choice" -le "${#ascii_array[@]}" ]; then
    ascii_file="${ascii_array[$((ascii_choice - 1))]}"
    curl -s -L "https://raw.githubusercontent.com/0l3d/ffetch/master/ascii/$ascii_file" -o ~/.config/ffetch/ascii.txt
    echo "ASCII art saved to ~/.config/ffetch/ascii.txt"
  else
    echo "Invalid choice. Skipping ASCII art."
  fi
fi

echo ""
echo -n "Move ffetch to /usr/local/bin? [y/N]: "
read move_binary

case "$move_binary" in
  [Yy]*)
    sudo cp "$BINARY_PATH" /usr/local/bin/ffetch
    echo "ffetch moved to /usr/local/bin/"
    echo "You can now run: ffetch"
    ;;
  *)
    echo "Installation complete!"
    echo "Binary location: $BINARY_PATH"
    echo "To move manually: sudo cp $BINARY_PATH /usr/local/bin/"
    ;;
esac

