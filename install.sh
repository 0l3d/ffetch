#!/bin/bash

REPO="https://github.com/0l3d/ffetch.git"
DIR="ffetch"
BINARY_PATH=""
DEFAULT_INSTALL_PATH="/usr/local/bin"

echo "Select installation method:"
echo "1) Build from source (Git version)"
echo "2) Download release binary"
echo "3) Update binary only"
echo "s) Skip installation"
echo -n "Choice [1-3,s]: "
read install_choice

case "$install_choice" in
  1)
    echo "Select build profile:"
    echo "1) Balanced (Balanced size & performance)"
    echo "2) Size (Size optimized)"
    echo "3) Performance (Performance optimized)"
    echo "s) Skip build"
    echo -n "Choice [1-3,s]: "
    read choice

    if [ "$choice" = "s" ]; then
      echo "Skipping build step."
      BINARY_PATH=""
    else
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
    fi
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
    
    mkdir -p "$DIR"
    cd "$DIR"
    curl -L "$DOWNLOAD_URL" -o ffetch.tar.gz
    tar -xzf ffetch.tar.gz
    chmod +x ffetch
    BINARY_PATH="$(pwd)/ffetch"
    echo "Download complete: $BINARY_PATH"
    rm ffetch.tar.gz
    cd ..
    ;;

  3)
    echo "Updating binary only..."
    DOWNLOAD_URL=$(curl -s https://api.github.com/repos/0l3d/ffetch/releases/latest \
      | grep -oP '"browser_download_url": "\K[^"]*ffetch-[^"]+-linux[^"]+\.tar\.gz')

    if [ -z "$DOWNLOAD_URL" ]; then
      echo "Error: Could not find release download URL."
      exit 1
    fi

    echo "Download URL: $DOWNLOAD_URL"
    
    mkdir -p "$DIR"
    cd "$DIR"
    curl -L "$DOWNLOAD_URL" -o ffetch.tar.gz
    tar -xzf ffetch.tar.gz
    chmod +x ffetch
    BINARY_PATH="$(pwd)/ffetch"
    echo "Update complete: $BINARY_PATH"
    rm ffetch.tar.gz
    cd ..
    ;;

  s|S)
    echo "Skipping installation step."
    BINARY_PATH=""
    ;;

  *)
    echo "Invalid choice"
    exit 1
    ;;
esac

if [ -z "$BINARY_PATH" ]; then
  echo "No binary built or downloaded. Exiting."
  exit 0
fi

if [ "$install_choice" = "1" ] || [ "$install_choice" = "2" ]; then
  echo ""
  echo "Setting up configuration..."
  mkdir -p ~/.config/ffetch

  echo "Select configuration profile:"
  echo "1) Advanced"
  echo "2) Middle"
  echo "3) Minimal"
  echo "s) Skip configuration"
  echo -n "Choice [1-3,s]: "
  read config_choice

  case "$config_choice" in
    1) 
      CONFIG_URL="https://raw.githubusercontent.com/0l3d/ffetch/refs/heads/master/ffetch-advanced.conf"
      CONFIG_NAME="Advanced"
      ;;
    2) 
      CONFIG_URL="https://raw.githubusercontent.com/0l3d/ffetch/refs/heads/master/ffetch-middle.conf"
      CONFIG_NAME="Middle"
      ;;
    3) 
      CONFIG_URL="https://raw.githubusercontent.com/0l3d/ffetch/refs/heads/master/ffetch-minimal.conf"
      CONFIG_NAME="Minimal"
      ;;
    s|S) 
      CONFIG_URL=""
      CONFIG_NAME=""
      ;;
    *) 
      CONFIG_URL="https://raw.githubusercontent.com/0l3d/ffetch/refs/heads/master/ffetch-middle.conf"
      CONFIG_NAME="Middle (default)"
      ;;
  esac

  if [ -n "$CONFIG_URL" ]; then
    echo "Downloading $CONFIG_NAME configuration..."
    if curl -s -L "$CONFIG_URL" -o ~/.config/ffetch/ffetch.conf; then
      echo "$CONFIG_NAME configuration saved to ~/.config/ffetch/ffetch.conf"
    else
      echo "Warning: Failed to download configuration file"
    fi
  else
    echo "Skipping configuration setup."
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

    echo -n "Select ASCII art [1-${#ascii_array[@]}] or s to skip: "
    read ascii_choice

    if [[ "$ascii_choice" =~ ^[0-9]+$ ]] && [ "$ascii_choice" -ge 1 ] && [ "$ascii_choice" -le "${#ascii_array[@]}" ]; then
      ascii_file="${ascii_array[$((ascii_choice - 1))]}"
      curl -s -L "https://raw.githubusercontent.com/0l3d/ffetch/master/ascii/$ascii_file" -o ~/.config/ffetch/ascii.txt
      echo "ASCII art saved to ~/.config/ffetch/ascii.txt"
    else
      echo "Skipping ASCII art selection."
    fi
  fi
fi

echo ""
echo "Installation path options:"
echo "1) Install to $DEFAULT_INSTALL_PATH (default)"
echo "2) Specify custom path"
echo "3) Keep binary in current location"
echo -n "Choice [1-3]: "
read install_path_choice

case "$install_path_choice" in
  1|"")
    if sudo cp "$BINARY_PATH" "$DEFAULT_INSTALL_PATH/ffetch"; then
      echo "ffetch installed to $DEFAULT_INSTALL_PATH/"
      echo "You can now run: ffetch"
      rm -rf "$DIR"
      echo "Cleaned up temporary directory: $DIR"
    else
      echo "Failed to install ffetch to $DEFAULT_INSTALL_PATH"
      echo "Binary remains at: $BINARY_PATH"
    fi
    ;;
  2)
    echo -n "Enter installation path (e.g., /usr/bin): "
    read custom_path
    if [ -n "$custom_path" ] && [ -d "$custom_path" ]; then
      if sudo cp "$BINARY_PATH" "$custom_path/ffetch"; then
        echo "ffetch installed to $custom_path/"
        echo "You can now run: ffetch"
        rm -rf "$DIR"
        echo "Cleaned up temporary directory: $DIR"
      else
        echo "Failed to install ffetch to $custom_path"
        echo "Binary remains at: $BINARY_PATH"
      fi
    else
      echo "Invalid path or directory does not exist"
      echo "Binary remains at: $BINARY_PATH"
    fi
    ;;
  3)
    echo "Installation complete!"
    echo "Binary location: $BINARY_PATH"
    echo "To install manually later: sudo cp $BINARY_PATH /usr/local/bin/"
    ;;
  *)
    echo "Invalid choice. Binary remains at: $BINARY_PATH"
    ;;
esac
