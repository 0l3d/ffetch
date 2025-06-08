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
      git clone --depth 1 "$REPO"
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
    echo "Downloading latest release..."
    DOWNLOAD_URL=$(curl -s https://api.github.com/repos/0l3d/ffetch/releases/latest | grep "browser_download_url.*linux.tar.gz" | cut -d '"' -f 4)

    if [ -z "$DOWNLOAD_URL" ]; then
      echo "Error: Could not find release URL"
      exit 1
    fi

    curl -L "$DOWNLOAD_URL" -o ffetch-linux.tar.gz
    tar -xzf ffetch-linux.tar.gz
    chmod +x ffetch
    BINARY_PATH="$(pwd)/ffetch"
    echo "Download complete: $BINARY_PATH"
    rm ffetch-linux.tar.gz

    # Shallow clone for ascii files etc.
    git clone --depth 1 "$REPO" "$DIR"
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
echo "Fetching ASCII art list from GitHub..."

ASCII_API_URL="https://api.github.com/repos/0l3d/ffetch/contents/ascii"
ascii_files=()

mapfile -t ascii_files < <(curl -s "$ASCII_API_URL" | grep -oP '(?<="name": ")[^"]+')

if [ ${#ascii_files[@]} -eq 0 ]; then
  echo "No ASCII art files found on GitHub."
else
  echo "Select ASCII art:"
  for i in "${!ascii_files[@]}"; do
    filename="${ascii_files[$i]}"
    echo "$((i+1))) ${filename%.*}"
  done

  echo -n "Choice [1-${#ascii_files[@]}]: "
  read ascii_choice

  if [[ "$ascii_choice" =~ ^[0-9]+$ ]] && [ "$ascii_choice" -ge 1 ] && [ "$ascii_choice" -le "${#ascii_files[@]}" ]; then
    selected_ascii="${ascii_files[$((ascii_choice-1))]}"
    raw_url="https://raw.githubusercontent.com/0l3d/ffetch/master/ascii/$selected_ascii"

    mkdir -p ~/.config/ffetch
    echo "Downloading $selected_ascii ..."
    curl -sL "$raw_url" -o ~/.config/ffetch/ascii.txt

    if [ $? -eq 0 ]; then
      echo "ASCII art downloaded and saved to ~/.config/ffetch/ascii.txt"
    else
      echo "Failed to download ASCII art."
    fi
  else
    echo "Invalid choice, skipping ASCII art download."
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

