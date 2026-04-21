#!/bin/bash
set -e

# ============================================================
#   Souprune Rust Mod 构建脚本 (Linux)
# ============================================================
#
# 用途: 编译你的 Mod 为可在游戏运行的可执行文件
# 
# 此脚本可以构建以下格式:
#   • Linux 版本 (.so)      - 在 Linux 上直接运行
#   • Windows MSVC 版本 (.dll) - Windows 用户使用 (推荐)
#   • Windows GNU 版本 (.dll)  - Windows 用户使用
#
# 【新手必读】
# 如果你是第一次构建，建议直接回车选择 [1] 全套！
# 这样可以一次性构建所有平台版本，方便分享给使用不同系统的朋友。
#
# ---- 依赖安装 (仅首次需要) ----
# 
# 构建 Windows 版本需要安装以下依赖:
#
# Debian/Ubuntu:
#   sudo apt-get install mingw-w64
#   rustup target add x86_64-pc-windows-gnu x86_64-pc-windows-msvc
#
# Arch Linux:
#   sudo pacman -S mingw-w64-gcc
#   rustup target add x86_64-pc-windows-gnu x86_64-pc-windows-msvc
#
# MSVC 构建工具 (在 Linux 上交叉编译):
#   cargo install cargo-xwin
#
# ============================================================

SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
MOD_NAME="$(basename "$SCRIPT_DIR")"
MOD_SOURCE_DIR="$SCRIPT_DIR/runtime"
DESTINATION_DIR="$SCRIPT_DIR"

BUILD_MODE="debug"
CARGO_FLAGS=""
BUILD_LINUX=false
BUILD_WIN_MSVC=false
BUILD_WIN_GNU=false
BUILD_ANDROID=false
FORCE_INTERACTIVE=false

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
NC='\033[0m'

print_banner() {
    echo -e "${CYAN}╔═══════════════════════════════════════════════════════════╗${NC}"
    echo -e "${CYAN}║                                                           ║${NC}"
    echo -e "${CYAN}║      🛠️  Souprune Mod 编译构建器 (Linux)              ║${NC}"
    echo -e "${CYAN}║                                                           ║${NC}"
    echo -e "${CYAN}╚═══════════════════════════════════════════════════════════╝${NC}"
    echo
}

print_info() { echo -e "${CYAN}ℹ️  ${NC} $1"; }
print_success() { echo -e "${GREEN}✅ ${NC} $1"; }
print_warning() { echo -e "${YELLOW}⚠️  ${NC} $1"; }
print_error() { echo -e "${RED}❌ ${NC} $1"; }

print_explanation() {
    echo -e "${YELLOW}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo -e "${YELLOW}📖 这些平台版本都是啥？${NC}"
    echo -e "${YELLOW}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo
    echo -e "${GREEN}📦 Linux 版本 (.so)${NC}"
    echo "   用途: 在 Linux 系统上直接运行此 Mod"
    echo "   文件名: $MOD_NAME.so"
    echo
    echo -e "${GREEN}📦 Windows MSVC 版本 (_msvc.dll)${NC}"
    echo "   用途: Windows 用户使用（推荐，大多数 Windows 用户）"
    echo "   文件名: ${MOD_NAME}_msvc.dll"
    echo
    echo -e "${GREEN}📦 Windows GNU 版本 (_gnu.dll)${NC}"
    echo "   用途: Windows + MinGW 环境，兼容性稍好"
    echo "   文件名: ${MOD_NAME}_gnu.dll"
    echo
    echo -e "${GREEN}📦 Android 版本 (_android.so)${NC}"
    echo "   用途: 在 Android 设备上运行此 Mod (需要 aarch64 NDK)"
    echo "   文件名: ${MOD_NAME}_android.so"
    echo
    echo -e "${YELLOW}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
}

show_menu() {
    print_banner
    
    echo -e "${CYAN}🎯 请选择要构建的版本:${NC}"
    echo
    echo -e "  ${GREEN}[1]${NC} 🚀 全套构建 (Linux + Win MSVC + Win GNU) ${GREEN}⭐ 推荐${NC}"
    echo "         一次构建所有平台版本，分享给所有人！"
    echo
    echo -e "  ${GREEN}[2]${NC} 🐧 仅 Linux 版本"
    echo "         只构建 Linux 用的 .so 文件"
    echo
    echo -e "  ${GREEN}[3]${NC} 🪟 仅 Windows MSVC 版本"
    echo "         只构建 Windows MSVC 用的 .dll 文件"
    echo
    echo -e "  ${GREEN}[4]${NC} 🪟 仅 Windows GNU 版本"
    echo "         只构建 Windows GNU 用的 .dll 文件"
    echo
    echo -e "  ${GREEN}[5]${NC} 🐧+🪟 Linux + Windows GNU"
    echo "         Linux + Windows GNU (跳过 MSVC)"
    echo
    echo -e "  ${GREEN}[6]${NC} 🐧+🪟 Linux + Windows MSVC"
    echo "         Linux + Windows MSVC (跳过 GNU)"
    echo
    echo -e "  ${GREEN}[7]${NC} 🤖 Android aarch64 版本"
    echo "         构建 Android 用的 .so 文件 (需要 Android NDK)"
    echo
    echo -e "${YELLOW}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    print_explanation
    echo -e "${YELLOW}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo
    echo -e "${GREEN}💡 小贴士:${NC} 如果不确定选什么，直接回车选择 [1] 全套就对了！"
    echo -e "${GREEN}   这样可以构建所有平台版本，不用担心遗漏。${NC}"
    echo
    echo -e "使用 ${CYAN}--help${NC} 参数可以查看命令行选项"
    echo
    
    read -p "请输入选项 (1-7) [直接回车 = 1]: " choice
    
    case "$choice" in
        1|"")
            BUILD_LINUX=true
            BUILD_WIN_MSVC=true
            BUILD_WIN_GNU=true
            echo
            print_info "已选择: 全套构建 (Linux + Win MSVC + Win GNU)"
            ;;
        2)
            BUILD_LINUX=true
            echo
            print_info "已选择: 仅 Linux 版本"
            ;;
        3)
            BUILD_WIN_MSVC=true
            echo
            print_info "已选择: 仅 Windows MSVC 版本"
            ;;
        4)
            BUILD_WIN_GNU=true
            echo
            print_info "已选择: 仅 Windows GNU 版本"
            ;;
        5)
            BUILD_LINUX=true
            BUILD_WIN_GNU=true
            echo
            print_info "已选择: Linux + Windows GNU"
            ;;
        6)
            BUILD_LINUX=true
            BUILD_WIN_MSVC=true
            echo
            print_info "已选择: Linux + Windows MSVC"
            ;;
        7)
            BUILD_ANDROID=true
            echo
            print_info "已选择: Android aarch64 版本"
            ;;
        *)
            print_warning "无效选择，将使用全套构建..."
            BUILD_LINUX=true
            BUILD_WIN_MSVC=true
            BUILD_WIN_GNU=true
            ;;
    esac
    
    sleep 1
}

while [[ "$#" -gt 0 ]]; do
    case $1 in
        --release|-r)
            BUILD_MODE="release"
            CARGO_FLAGS="--release"
            ;;
        --linux|-l)
            BUILD_LINUX=true
            ;;
        --win-msvc|-wm)
            BUILD_WIN_MSVC=true
            ;;
        --win-gnu|-wg)
            BUILD_WIN_GNU=true
            ;;
        --win|-w)
            BUILD_WIN_MSVC=true
            BUILD_WIN_GNU=true
            ;;
        --all|-a)
            BUILD_LINUX=true
            BUILD_WIN_MSVC=true
            BUILD_WIN_GNU=true
            ;;
        --android|-an)
            BUILD_ANDROID=true
            ;;
        --help|-h)
            echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
            echo " Souprune Mod 构建器 - 帮助信息"
            echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
            echo
            echo "用法: $0 [选项]"
            echo
            echo "选项:"
            echo "  -r, --release          构建 release (发布) 版本，性能更好"
            echo "  -l, --linux            仅构建 Linux 版本"
            echo "  -w, --win              构建 Windows 版本 (MSVC + GNU)"
            echo "  -wm, --win-msvc        仅构建 Windows MSVC 版本"
            echo "  -wg, --win-gnu         仅构建 Windows GNU 版本"
            echo "  -a, --all              构建全套 (Linux + Win MSVC + Win GNU)"
            echo "  -an, --android         仅构建 Android aarch64 版本"
            echo "  -i, --interactive      强制显示交互式菜单"
            echo "  -h, --help             显示此帮助"
            echo
            echo "示例:"
            echo "  $0                     # 显示交互式菜单"
            echo "  $0 -a                  # 构建全套"
            echo "  $0 -r -a               # 构建全套 release 版本"
            echo "  $0 -l                  # 仅构建 Linux 版本"
            echo
            echo "提示: 无参数运行将显示交互式菜单，建议新手使用！"
            exit 0
            ;;
        -i|--interactive)
            FORCE_INTERACTIVE=true
            ;;
        *)
            print_error "未知参数: $1"
            echo "使用 --help 查看帮助"
            exit 1
            ;;
    esac
    shift
done

if [[ ! -t 0 && "$FORCE_INTERACTIVE" != "true" ]]; then
    BUILD_LINUX=true
    BUILD_WIN_MSVC=true
    BUILD_WIN_GNU=true
elif [[ "$BUILD_LINUX" == "false" && "$BUILD_WIN_MSVC" == "false" && "$BUILD_WIN_GNU" == "false" && "$BUILD_ANDROID" == "false" ]]; then
    show_menu
fi

build_target() {
    local target=$1
    local cargo_output_name=$2
    local final_output_name=$3
    local custom_build_cmd=${4:-"cargo build"}
    local cargo_target_flag=""
    local target_dir="target/$BUILD_MODE"

    if [ -n "$target" ]; then
        cargo_target_flag="--target $target"
        target_dir="target/$target/$BUILD_MODE"
        echo -e "${CYAN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
        print_info "正在构建: $target (模式: $BUILD_MODE)"
    else
        echo -e "${CYAN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
        print_info "正在构建: Linux Native (模式: $BUILD_MODE)"
    fi

    pushd "$MOD_SOURCE_DIR" > /dev/null
    $custom_build_cmd $CARGO_FLAGS $cargo_target_flag
    popd > /dev/null

    local source_file="$MOD_SOURCE_DIR/$target_dir/$cargo_output_name"
    if [ ! -f "$source_file" ]; then
        source_file="$MOD_SOURCE_DIR/$target_dir/deps/$cargo_output_name"
    fi

    if [ -f "$source_file" ]; then
        print_info "复制文件到: $DESTINATION_DIR/$final_output_name"
        cp -f "$source_file" "$DESTINATION_DIR/$final_output_name"
    else
        print_error "找不到构建产物: $source_file"
        exit 1
    fi
}

check_rust_target() {
    local target=$1
    if ! rustup target list --installed | grep -q "^$target$"; then
        print_error "未安装 Rust 目标: $target"
        echo "请运行: rustup target add $target"
        return 1
    fi
    return 0
}

check_command() {
    local cmd=$1
    local install_hint=$2
    if ! command -v "$cmd" >/dev/null 2>&1; then
        print_warning "未找到命令: $cmd"
        if [ -n "$install_hint" ]; then
            echo "安装方法: $install_hint"
        fi
        return 1
    fi
    return 0
}

if [ ! -d "$MOD_SOURCE_DIR" ]; then
    print_error "找不到源代码目录: $MOD_SOURCE_DIR"
    echo
    echo "请确认你正在正确的 Mod 目录下运行此脚本。"
    echo "如果你在原始目录，可能需要先运行: ./build_csharp.sh"
    exit 1
fi

echo
echo -e "${BLUE}════════════════════════════════════════════════════════════${NC}"
echo -e "${BLUE}  📋 构建配置${NC}"
echo -e "${BLUE}════════════════════════════════════════════════════════════${NC}"
echo "  Mod 名称: $MOD_NAME"
echo "  编译模式: $BUILD_MODE"
echo
[ "$BUILD_LINUX" = true ] && echo "  ✅ Linux 版本" || echo "  ❌ Linux 版本"
[ "$BUILD_WIN_MSVC" = true ] && echo "  ✅ Windows MSVC 版本" || echo "  ❌ Windows MSVC 版本"
[ "$BUILD_WIN_GNU" = true ] && echo "  ✅ Windows GNU 版本" || echo "  ❌ Windows GNU 版本"
[ "$BUILD_ANDROID" = true ] && echo "  ✅ Android aarch64 版本" || echo "  ❌ Android aarch64 版本"
echo

COUNT=0

if [ "$BUILD_LINUX" = true ]; then
    COUNT=$((COUNT + 1))
    echo -e "${GREEN}▶ [$COUNT] 开始构建 Linux 版本...${NC}"
    build_target "" "libruntime.so" "${MOD_NAME}.so"
    print_success "Linux 版本构建完成! 🎉"
    echo
fi

if [ "$BUILD_WIN_MSVC" = true ]; then
    COUNT=$((COUNT + 1))
    echo -e "${GREEN}▶ [$COUNT] 开始构建 Windows MSVC 版本...${NC}"
    
    if command -v cargo-xwin >/dev/null 2>&1; then
        if check_rust_target "x86_64-pc-windows-msvc"; then
            export RUSTFLAGS=""
            build_target "x86_64-pc-windows-msvc" "runtime.dll" "${MOD_NAME}_msvc.dll" "cargo xwin build"
            print_success "Windows MSVC 版本构建完成! 🎉"
        else
            print_warning "跳过 MSVC 构建 (缺少 Rust Target: x86_64-pc-windows-msvc)"
            echo "   运行: rustup target add x86_64-pc-windows-msvc"
        fi
    else
        print_warning "未检测到 cargo-xwin，跳过 MSVC 构建"
        echo "   💡 安装方法: cargo install cargo-xwin"
    fi
    echo
fi

if [ "$BUILD_WIN_GNU" = true ]; then
    COUNT=$((COUNT + 1))
    echo -e "${GREEN}▶ [$COUNT] 开始构建 Windows GNU 版本...${NC}"
    
    MISSING_DEPS=false
    
    if ! check_rust_target "x86_64-pc-windows-gnu"; then
        MISSING_DEPS=true
        echo "   需要运行: rustup target add x86_64-pc-windows-gnu"
    fi
    
    if ! check_command "x86_64-w64-mingw32-gcc" "mingw-w64"; then
        MISSING_DEPS=true
    fi
    
    if [ "$MISSING_DEPS" = false ]; then
        export RUSTFLAGS="-C target-feature=+crt-static"
        build_target "x86_64-pc-windows-gnu" "runtime.dll" "${MOD_NAME}_gnu.dll"
        print_success "Windows GNU 版本构建完成! 🎉"
    else
        print_error "缺少依赖，跳过 GNU 构建"
    fi
    echo
fi

if [ "$BUILD_ANDROID" = true ]; then
    COUNT=$((COUNT + 1))
    echo -e "${GREEN}▶ [$COUNT] 开始构建 Android aarch64 版本...${NC}"

    MISSING_DEPS=false

    if ! check_rust_target "aarch64-linux-android"; then
        print_warning "未安装 Rust 目标: aarch64-linux-android"
        echo "   需要运行: rustup target add aarch64-linux-android"
        MISSING_DEPS=true
    fi

    # Detect Android NDK
    NDK_HOME="${ANDROID_NDK_HOME:-${ANDROID_HOME:+$ANDROID_HOME/ndk}}"
    if [ -z "$NDK_HOME" ] || [ ! -d "$NDK_HOME" ]; then
        # Try common locations
        for candidate in "$HOME/Android/Sdk/ndk"/* /opt/android-ndk* "$HOME/android-ndk"*; do
            if [ -d "$candidate/toolchains/llvm/prebuilt" ]; then
                NDK_HOME="$candidate"
                break
            fi
        done
    fi

    if [ -z "$NDK_HOME" ] || [ ! -d "$NDK_HOME" ]; then
        print_error "未找到 Android NDK"
        echo "   请设置 ANDROID_NDK_HOME 环境变量，或将 NDK 安装到 ~/Android/Sdk/ndk/"
        MISSING_DEPS=true
    fi

    if [ "$MISSING_DEPS" = false ]; then
        # Find the NDK toolchain
        HOST_TAG="linux-x86_64"
        NDK_TOOLCHAIN="$NDK_HOME/toolchains/llvm/prebuilt/$HOST_TAG"
        if [ ! -d "$NDK_TOOLCHAIN" ]; then
            print_error "找不到 NDK 工具链: $NDK_TOOLCHAIN"
        else
            export CARGO_TARGET_AARCH64_LINUX_ANDROID_LINKER="$NDK_TOOLCHAIN/bin/aarch64-linux-android21-clang"
            export CC_aarch64_linux_android="$NDK_TOOLCHAIN/bin/aarch64-linux-android21-clang"
            export AR_aarch64_linux_android="$NDK_TOOLCHAIN/bin/llvm-ar"

            build_target "aarch64-linux-android" "libruntime.so" "${MOD_NAME}_android.so"
            print_success "Android aarch64 版本构建完成! 🎉"
        fi
    else
        print_error "缺少依赖，跳过 Android 构建"
    fi
    echo
fi

echo
echo -e "${GREEN}╔═══════════════════════════════════════════════════════════╗${NC}"
echo -e "${GREEN}║                                                           ║${NC}"
echo -e "${GREEN}║              🎉 构建完成! 所有任务结束! 🎉                ║${NC}"
echo -e "${GREEN}║                                                           ║${NC}"
echo -e "${GREEN}╚═══════════════════════════════════════════════════════════╝${NC}"
echo
echo "📁 生成的文件:"
[ "$BUILD_LINUX" = true ] && echo "   • ${MOD_NAME}.so           (Linux)"
[ "$BUILD_WIN_MSVC" = true ] && echo "   • ${MOD_NAME}_msvc.dll     (Windows MSVC)"
[ "$BUILD_WIN_GNU" = true ] && echo "   • ${MOD_NAME}_gnu.dll      (Windows GNU)"
[ "$BUILD_ANDROID" = true ] && echo "   • ${MOD_NAME}_android.so   (Android aarch64)"
echo
echo "💡 下一步: 现在可以直接启动游戏测试你的 Mod 了！"
echo
