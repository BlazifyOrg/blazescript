#!/bin/sh
set -e

if ! command -v cargo >/dev/null; then
	echo "Error: Rust is required to install Blazescript (see: https://www.rust-lang.org/tools/install)." 1>&2
	exit 1
fi

bzs_install="${BZS_INSTALL:-$HOME/.bzs}"
bin_dir="$bzs_install/bin"

mkdir $bin_dir
cd $bin_dir

if [ "$OS" = "Windows_NT" ]; then
	target="blazescript-windows"
    exe="$bin_dir/blazescript.exe"
else
	case $(uname -sm) in
	"Darwin x86_64") target="blazescript-macos" ;;
	"Darwin arm64") target="blazescript-macos" ;;
	*) target="blazescript-linux" ;;
	esac
    exe="$bin_dir/blazescript"
fi

if [ $# -eq 0 ]; then
	bzs_uri="https://github.com/BlazifyOrg/blazescript/releases/latest/download/${target}"
else
	bzs_uri="https://github.com/BlazifyOrg/blazescript/releases/download/${1}/${target}"
fi

curl --fail --location --progress-bar --output "$exe" "$bzs_uri"
chmod +x "$exe"

if command -v blazescript >/dev/null; then
    echo "Run 'blazescript [file name][(.bzs)/(.bze)]' to get started"
else
    case $SHELL in
    	/bin/zsh) shell_profile=".zshrc" ;;
    	*) shell_profile=".bash_profile" ;;
    esac

    echo "Manually add the directory to your \$HOME/$shell_profile (or similar)"
    echo "  export BZS_INSTALL=\"$bzs_install\""
    echo "  export PATH=\"\$BZS_INSTALL/bin:\$PATH\""
    echo "Run '$exe [file name][(.bzs)/(.bze)]' to get started"
fi