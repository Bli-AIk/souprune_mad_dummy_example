<#
.SYNOPSIS
    Souprune Rust Mod 构建脚本 (Windows)

.DESCRIPTION
    此脚本用于构建 Souprune Rust Mod，支持交互式菜单。
    可以构建 Windows (MSVC/GNU) 和 Linux 版本！
    
    【新手必读】
    如果你是第一次构建，建议直接回车选择 [1] 全套！
    这样可以一次性构建所有平台版本，方便分享给使用不同系统的朋友。

.PARAMETER Release
    构建 release (发布) 版本，性能更好

.PARAMETER All
    构建全套 (Windows MSVC + GNU + Linux)

.PARAMETER Linux
    构建 Linux 版本 (需要交叉编译工具链)

.PARAMETER MSVC
    仅构建 Windows MSVC 版本

.PARAMETER GNU
    仅构建 Windows GNU 版本

.PARAMETER Interactive
    强制显示交互式菜单

.PARAMETER Help
    显示帮助信息

.EXAMPLE
    .\win_builder.ps1
    显示交互式菜单

.EXAMPLE
    .\win_builder.ps1 -All
    构建全套

.EXAMPLE
    .\win_builder.ps1 -Release -All
    构建全套 release 版本
#>

param (
    [Switch]$Release,
    [Switch]$All,
    [Switch]$Linux,
    [Switch]$Interactive,
    [Switch]$Help,
    [Switch]$MSVC,
    [Switch]$GNU
)

$ErrorActionPreference = "Stop"

# Configuration
$ScriptDir = $PSScriptRoot
$ModName = Split-Path $ScriptDir -Leaf
$ModSourceDir = Join-Path $ScriptDir "code\mod_example"
$DestinationDir = $ScriptDir
$BuildMode = if ($Release) { "release" } else { "debug" }
$CargoFlags = if ($Release) { "--release" } else { "" }

$BuildWindowsMSVC = $false
$BuildWindowsGNU = $false
$BuildLinux = $false

function Write-ColorOutput {
    param([string]$Message, [string]$Color = "White")
    $colors = @{
        "Red" = [ConsoleColor]::Red
        "Green" = [ConsoleColor]::Green
        "Yellow" = [ConsoleColor]::Yellow
        "Cyan" = [ConsoleColor]::Cyan
        "Magenta" = [ConsoleColor]::Magenta
        "White" = [ConsoleColor]::White
    }
    Write-Host $Message -ForegroundColor $colors[$Color]
}

function Write-Banner {
    Write-Host ""
    Write-ColorOutput "╔═══════════════════════════════════════════════════════════╗" "Cyan"
    Write-ColorOutput "║                                                           ║" "Cyan"
    Write-ColorOutput "║         🛠️  Souprune Mod 编译构建器 (Windows)          ║" "Cyan"
    Write-ColorOutput "║                                                           ║" "Cyan"
    Write-ColorOutput "╚═══════════════════════════════════════════════════════════╝" "Cyan"
    Write-Host ""
}

function Write-Explanation {
    Write-Host "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━" -ForegroundColor Yellow
    Write-Host "📖 这些平台版本都是啥？" -ForegroundColor Yellow
    Write-Host "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━" -ForegroundColor Yellow
    Write-Host ""
    Write-Host "📦 Windows MSVC 版本 (_msvc.dll)" -ForegroundColor Green
    Write-Host "   用途: 在 Windows 系统上使用 (推荐，大多数 Windows 用户)"
    Write-Host "   文件名: ${ModName}_msvc.dll"
    Write-Host ""
    Write-Host "📦 Windows GNU 版本 (_gnu.dll)" -ForegroundColor Green
    Write-Host "   用途: Windows + MinGW 环境"
    Write-Host "   文件名: ${ModName}_gnu.dll"
    Write-Host ""
    Write-Host "📦 Linux 版本 (.so)" -ForegroundColor Green
    Write-Host "   用途: 在 Linux 系统上使用 (需要交叉编译)"
    Write-Host "   文件名: ${ModName}.so"
    Write-Host ""
    Write-Host "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━" -ForegroundColor Yellow
}

function Show-Menu {
    Clear-Host
    Write-Banner
    
    Write-Host "🎯 请选择要构建的版本:" -ForegroundColor Cyan
    Write-Host ""
    Write-Host "  [1] (Win MSVC 🚀 全套构建 + Win GNU + Linux) ⭐ 推荐" -ForegroundColor Green
    Write-Host "         一次构建所有平台版本，分享给所有人！"
    Write-Host ""
    Write-Host "  [2] 🪟 仅 Windows MSVC 版本"
    Write-Host "         只构建 Windows MSVC 用的 .dll 文件"
    Write-Host ""
    Write-Host "  [3] 🪟 仅 Windows GNU 版本"
    Write-Host "         只构建 Windows GNU 用的 .dll 文件"
    Write-Host ""
    Write-Host "  [4] 🐧 仅 Linux 版本"
    Write-Host "         只构建 Linux 用的 .so 文件 (需要交叉编译工具链)"
    Write-Host ""
    Write-Host "  [5] 🪟 Windows MSVC + GNU"
    Write-Host "         两个 Windows 版本"
    Write-Host ""
    Write-Host "  [6] 🪟+🐧 Windows MSVC + Linux"
    Write-Host "         Windows MSVC + Linux"
    Write-Host ""
    Write-Explanation
    
    Write-Host "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━" -ForegroundColor Yellow
    Write-Host "💡 小贴士: 如果不确定选什么，直接回车选择 [1] 全套就对了！" -ForegroundColor Green
    Write-Host "   这样可以构建所有平台版本，不用担心遗漏。" -ForegroundColor Green
    Write-Host ""
    Write-Host "使用 -Help 参数可以查看命令行选项"
    Write-Host ""
    
    $choice = Read-Host "请输入选项 (1-6) [直接回车 = 1]"
    
    switch ($choice) {
        "1" { 
            $script:BuildWindowsMSVC = $true; $script:BuildWindowsGNU = $true; $script:BuildLinux = $true 
            Write-Host ""
            Write-Host "已选择: 全套构建 (Win MSVC + Win GNU + Linux)" -ForegroundColor Green
        }
        "2" { 
            $script:BuildWindowsMSVC = $true 
            Write-Host ""
            Write-Host "已选择: 仅 Windows MSVC 版本" -ForegroundColor Green
        }
        "3" { 
            $script:BuildWindowsGNU = $true 
            Write-Host ""
            Write-Host "已选择: 仅 Windows GNU 版本" -ForegroundColor Green
        }
        "4" { 
            $script:BuildLinux = $true 
            Write-Host ""
            Write-Host "已选择: 仅 Linux 版本" -ForegroundColor Green
        }
        "5" { 
            $script:BuildWindowsMSVC = $true; $script:BuildWindowsGNU = $true
            Write-Host ""
            Write-Host "已选择: Windows MSVC + GNU" -ForegroundColor Green
        }
        "6" { 
            $script:BuildWindowsMSVC = $true; $script:BuildLinux = $true
            Write-Host ""
            Write-Host "已选择: Windows MSVC + Linux" -ForegroundColor Green
        }
        ""  { 
            $script:BuildWindowsMSVC = $true; $script:BuildWindowsGNU = $true; $script:BuildLinux = $true 
            Write-Host ""
            Write-Host "已选择: 全套构建 (Win MSVC + Win GNU + Linux)" -ForegroundColor Green
        }
        default { 
            Write-Warning "无效选择，将使用全套构建..."
            $script:BuildWindowsMSVC = $true
            $script:BuildWindowsGNU = $true
            $script:BuildLinux = $true
        }
    }
    
    Start-Sleep -Seconds 1
}

# Show help and exit if requested
if ($Help) {
    Write-Host "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━" -ForegroundColor Cyan
    Write-Host " Souprune Mod 构建器 - 帮助信息" -ForegroundColor Cyan
    Write-Host "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━" -ForegroundColor Cyan
    Write-Host ""
    Write-Host "用法: .\win_builder.ps1 [选项]"
    Write-Host ""
    Write-Host "选项:"
    Write-Host "  -Release          构建 release (发布) 版本，性能更好"
    Write-Host "  -All              构建全套 (Win MSVC + Win GNU + Linux)"
    Write-Host "  -Linux            仅构建 Linux 版本"
    Write-Host "  -MSVC             仅构建 Windows MSVC 版本"
    Write-Host "  -GNU              仅构建 Windows GNU 版本"
    Write-Host "  -Interactive      强制显示交互式菜单"
    Write-Host "  -Help             显示此帮助"
    Write-Host ""
    Write-Host "示例:"
    Write-Host "  .\win_builder.ps1              # 显示交互式菜单"
    Write-Host "  .\win_builder.ps1 -All         # 构建全套"
    Write-Host "  .\win_builder.ps1 -Release -All# 构建全套 release 版本"
    Write-Host "  .\win_builder.ps1 -MSVC         # 仅构建 Windows MSVC"
    Write-Host ""
    Write-Host "提示: 无参数运行将显示交互式菜单，建议新手使用！" -ForegroundColor Yellow
    exit 0
}

# Determine build targets
$hasExplicitTarget = $All -or $MSVC -or $GNU -or $Linux

if ($Interactive -or (-not $hasExplicitTarget -and -not $IsInteractive)) {
    if (-not $IsInteractive) {
        # Non-interactive mode: build all by default
        $BuildWindowsMSVC = $true
        $BuildWindowsGNU = $true
        $BuildLinux = $true
    } else {
        Show-Menu
    }
} else {
    # Command line parameters
    if ($All) {
        $BuildWindowsMSVC = $true
        $BuildWindowsGNU = $true
        $BuildLinux = $true
    } else {
        if ($MSVC) { $BuildWindowsMSVC = $true }
        if ($GNU) { $BuildWindowsGNU = $true }
    }
    if ($Linux) { $BuildLinux = $true }
}

# If no target specified, show menu
if (-not $BuildWindowsMSVC -and -not $BuildWindowsGNU -and -not $BuildLinux) {
    Show-Menu
}

function Test-Command {
    param ($Command)
    return (Get-Command $Command -ErrorAction SilentlyContinue) -ne $null
}

function Test-RustTarget {
    param ($Target)
    $Installed = rustup target list --installed
    return $Installed -contains $Target
}

function Build-Target {
    param (
        [string]$Target,
        [string]$CargoOutputName,
        [string]$FinalOutputName,
        [string]$EnvVarName,
        [string]$EnvVarValue
    )

    $TargetDir = "target/$BuildMode"
    if (-not [string]::IsNullOrEmpty($Target)) {
        $TargetDir = "target/$Target/$BuildMode"
        Write-Host "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━" -ForegroundColor Cyan
        Write-Host "正在构建: $Target (模式: $BuildMode)" -ForegroundColor Cyan
    } else {
        Write-Host "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━" -ForegroundColor Cyan
        Write-Host "正在构建: Native (模式: $BuildMode)" -ForegroundColor Cyan
    }
    
    if (-not (Test-RustTarget $Target)) {
        Write-Host "Rust target '$Target' 未安装，正在尝试安装..." -ForegroundColor Yellow
        rustup target add $Target
        if ($LASTEXITCODE -ne 0) {
            Write-Error "安装 target '$Target' 失败。"
        }
    }

    Push-Location $ModSourceDir

    if (-not [string]::IsNullOrEmpty($EnvVarName)) {
        $OldEnvValue = (Get-Item env:$EnvVarName -ErrorAction SilentlyContinue).Value
        Set-Item env:$EnvVarName $EnvVarValue
    }

    $BuildArgs = @("build")
    if ($Release) { $BuildArgs += "--release" }
    if (-not [string]::IsNullOrEmpty($Target)) { $BuildArgs += "--target", $Target }

    Write-Host "执行命令: cargo $($BuildArgs -join ' ')" -ForegroundColor DarkGray
    cargo $BuildArgs
    if ($LASTEXITCODE -ne 0) {
        Pop-Location
        Write-Error "构建失败: $Target"
    }

    if (-not [string]::IsNullOrEmpty($EnvVarName)) {
        if ($OldEnvValue -eq $null) {
            Remove-Item env:$EnvVarName
        } else {
            Set-Item env:$EnvVarName $OldEnvValue
        }
    }

    Pop-Location

    $SourcePath = Join-Path $ModSourceDir "$TargetDir/$CargoOutputName"
    if (-not (Test-Path $SourcePath)) {
        $SourcePath = Join-Path $ModSourceDir "$TargetDir/deps/$CargoOutputName"
    }

    if (Test-Path $SourcePath) {
        $DestPath = Join-Path $DestinationDir $FinalOutputName
        Write-Host "复制文件到: $DestPath" -ForegroundColor Green
        Copy-Item -Force $SourcePath $DestPath
    } else {
        Write-Error "找不到构建产物: $SourcePath"
    }
}

if (-not (Test-Path $ModSourceDir)) {
    Write-Error "找不到源代码目录: $ModSourceDir"
    Write-Host ""
    Write-Host "请确认你正在正确的 Mod 目录下运行此脚本。" -ForegroundColor Yellow
    exit 1
}

if (-not (Test-Command "cargo")) {
    Write-Error "找不到 Cargo (Rust) 命令"
    Write-Host "请安装 Rust: https://rustup.rs/" -ForegroundColor Yellow
    exit 1
}

Write-Host ""
Write-Host "════════════════════════════════════════════════════════════" -ForegroundColor Blue
Write-Host "  📋 构建配置" -ForegroundColor Blue
Write-Host "════════════════════════════════════════════════════════════" -ForegroundColor Blue
Write-Host "  Mod 名称: $ModName"
Write-Host "  编译模式: $BuildMode"
Write-Host ""
if ($BuildWindowsMSVC) { Write-Host "  ✅ Windows MSVC 版本" -ForegroundColor Green } else { Write-Host "  ❌ Windows MSVC 版本" }
if ($BuildWindowsGNU) { Write-Host "  ✅ Windows GNU 版本" -ForegroundColor Green } else { Write-Host "  ❌ Windows GNU 版本" }
if ($BuildLinux) { Write-Host "  ✅ Linux 版本" -ForegroundColor Green } else { Write-Host "  ❌ Linux 版本" }
Write-Host ""

$count = 0

if ($BuildWindowsMSVC) {
    $count++
    Write-Host "▶ [$count] 开始构建 Windows MSVC 版本..." -ForegroundColor Green
    Build-Target "x86_64-pc-windows-msvc" "mod_example.dll" "${ModName}_msvc.dll"
    Write-Host "✅ Windows MSVC 版本构建完成! 🎉" -ForegroundColor Green
    Write-Host ""
}

if ($BuildWindowsGNU) {
    $count++
    Write-Host "▶ [$count] 开始构建 Windows GNU 版本..." -ForegroundColor Green
    
    if (Test-RustTarget "x86_64-pc-windows-gnu") {
        if ((Test-Command "gcc") -or (Test-Command "x86_64-w64-mingw32-gcc")) {
            Build-Target "x86_64-pc-windows-gnu" "mod_example.dll" "${ModName}_gnu.dll" "RUSTFLAGS" "-C target-feature=+crt-static"
            Write-Host "✅ Windows GNU 版本构建完成! 🎉" -ForegroundColor Green
        } else {
            Write-Warning "跳过 GNU 构建: 未找到 GCC/MinGW linker"
            Write-Host "   安装方法: 安装 MinGW-w64" -ForegroundColor Yellow
        }
    } else {
        Write-Warning "跳过 GNU 构建: Rust target 未安装"
        Write-Host "   运行: rustup target add x86_64-pc-windows-gnu" -ForegroundColor Yellow
    }
    Write-Host ""
}

if ($BuildLinux) {
    $count++
    Write-Host "▶ [$count] 开始构建 Linux 版本..." -ForegroundColor Green
    
    if (Test-RustTarget "x86_64-unknown-linux-gnu") {
        $hasCrossCompiler = (Test-Command "x86_64-linux-gnu-gcc") -or (Test-Command "x86_64-unknown-linux-gnu-gcc")
        
        if ($hasCrossCompiler) {
            $env:RUSTFLAGS = "-C linker=x86_64-linux-gnu-gcc"
            Build-Target "x86_64-unknown-linux-gnu" "libmod_example.so" "$ModName.so"
            Write-Host "✅ Linux 版本构建完成! 🎉" -ForegroundColor Green
        } else {
            Write-Warning "跳过 Linux 构建: 未找到 Linux 交叉编译器"
            Write-Host "   安装方法 (Debian/Ubuntu): sudo apt install gcc-x86-64-linux-gnu" -ForegroundColor Yellow
            Write-Host "   然后运行: rustup target add x86_64-unknown-linux-gnu" -ForegroundColor Yellow
        }
    } else {
        Write-Warning "跳过 Linux 构建: Rust target 未安装"
        Write-Host "   运行: rustup target add x86_64-unknown-linux-gnu" -ForegroundColor Yellow
    }
    Write-Host ""
}

Write-Host ""
Write-Host "╔═══════════════════════════════════════════════════════════╗" -ForegroundColor Green
Write-Host "║                                                           ║" -ForegroundColor Green
Write-Host "║              🎉 构建完成! 所有任务结束! 🎉                ║" -ForegroundColor Green
Write-Host "║                                                           ║" -ForegroundColor Green
Write-Host "╚═══════════════════════════════════════════════════════════╝" -ForegroundColor Green
Write-Host ""
Write-Host "📁 生成的文件:"
if ($BuildWindowsMSVC) { Write-Host "   • ${ModName}_msvc.dll  (Windows MSVC)" }
if ($BuildWindowsGNU) { Write-Host "   • ${ModName}_gnu.dll   (Windows GNU)" }
if ($BuildLinux) { Write-Host "   • ${ModName}.so       (Linux)" }
Write-Host ""
Write-Host "💡 下一步: 现在可以直接启动游戏测试你的 Mod 了！" -ForegroundColor Yellow
Write-Host ""
