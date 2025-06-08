#!/bin/bash
REPO="https://github.com/0l3d/ffetch.git"
DIR="ffetch"

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
