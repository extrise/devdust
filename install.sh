#!/bin/bash
# =============================================================================
# devdust Installation Script
# =============================================================================
# This script provides an interactive menu system to install/uninstall devdust
# Supports: Linux, macOS, Windows (WSL/Git Bash/MSYS2)
# =============================================================================

# Exit on error
set -e

# =============================================================================
# Color Definitions for Terminal Output
# =============================================================================
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
MAGENTA='\033[0;35m'
NC='\033[0m' # No Color

# =============================================================================
# Utility Functions for Colored Output
# =============================================================================

# Print informational message in blue
print_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

# Print success message in green
print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

# Print warning message in yellow
print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

# Print error message in red
print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Print step message in cyan
print_step() {
    echo -e "${CYAN}[STEP]${NC} $1"
}

# =============================================================================
# OS Detection Function
# =============================================================================
# Detects the operating system and sets OS_TYPE variable
# Supported: linux, macos, windows (WSL/Git Bash/MSYS2)
# =============================================================================
detect_os() {
    case "$(uname -s)" in
        Linux*)
            # Check if running in WSL (Windows Subsystem for Linux)
            if grep -qi microsoft /proc/version 2>/dev/null; then
                OS_TYPE="wsl"
                print_info "Detected: Windows Subsystem for Linux (WSL)"
            else
                OS_TYPE="linux"
                print_info "Detected: Linux"
            fi
            ;;
        Darwin*)
            OS_TYPE="macos"
            print_info "Detected: macOS"
            ;;
        CYGWIN*|MINGW*|MSYS*)
            OS_TYPE="windows"
            print_info "Detected: Windows (Git Bash/MSYS2)"
            ;;
        *)
            OS_TYPE="unknown"
            print_warning "Unknown OS: $(uname -s)"
            print_info "Attempting to continue with Linux defaults..."
            OS_TYPE="linux"
            ;;
    esac
}

# =============================================================================
# Rust Installation Check and Auto-Install
# =============================================================================
# Checks if Rust is installed, if not, installs it based on OS
# =============================================================================
check_and_install_rust() {
    print_step "Checking for Rust installation..."
    
    # Check if cargo command exists
    if command -v cargo &> /dev/null; then
        local rust_version=$(rustc --version)
        print_success "Rust is already installed: $rust_version"
        return 0
    fi
    
    # Rust not found, offer to install
    print_warning "Rust is not installed on your system"
    echo ""
    echo -ne "${YELLOW}Would you like to install Rust now? [Y/n]:${NC} "
    read -r install_choice
    
    # Default to yes if user just presses Enter
    install_choice=${install_choice:-Y}
    
    if [[ ! "$install_choice" =~ ^[Yy]$ ]]; then
        print_error "Rust is required to build devdust"
        print_info "Please install Rust manually from: https://rustup.rs/"
        exit 1
    fi
    
    echo ""
    print_info "Installing Rust for your operating system..."
    
    # Install Rust based on OS type
    case "$OS_TYPE" in
        linux|macos|wsl)
            # Unix-like systems: use rustup installer
            print_info "Downloading and running rustup installer..."
            curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
            
            # Source the cargo environment
            if [ -f "$HOME/.cargo/env" ]; then
                source "$HOME/.cargo/env"
                print_success "Rust installed successfully!"
            else
                print_error "Rust installation completed but environment not found"
                print_info "Please restart your terminal and run this script again"
                exit 1
            fi
            ;;
        windows)
            # Windows (Git Bash/MSYS2): use rustup installer
            print_info "Downloading rustup installer for Windows..."
            print_warning "You may need to run the installer manually"
            
            # Try to download and run rustup-init.exe
            if command -v curl &> /dev/null; then
                curl --proto '=https' --tlsv1.2 -sSf https://win.rustup.rs/x86_64 -o rustup-init.exe
                print_info "Running rustup-init.exe..."
                ./rustup-init.exe -y
                rm rustup-init.exe
                
                # Add cargo to PATH for current session
                export PATH="$HOME/.cargo/bin:$PATH"
                print_success "Rust installed successfully!"
            else
                print_error "curl not found. Please install Rust manually"
                print_info "Visit: https://rustup.rs/"
                exit 1
            fi
            ;;
        *)
            print_error "Automatic Rust installation not supported for this OS"
            print_info "Please install Rust manually from: https://rustup.rs/"
            exit 1
            ;;
    esac
    
    # Verify installation
    if command -v cargo &> /dev/null; then
        print_success "Rust verification passed: $(rustc --version)"
    else
        print_error "Rust installation failed or not in PATH"
        print_info "Please restart your terminal and try again"
        exit 1
    fi
}

# =============================================================================
# Build Project Function
# =============================================================================
# Compiles devdust in release mode using cargo
# =============================================================================
build_project() {
    print_step "Building devdust in release mode..."
    echo ""
    
    # Build the devdust CLI package in release mode
    cargo build --release -p devdust
    
    # Check if build was successful
    if [ $? -eq 0 ]; then
        print_success "Build completed successfully!"
        
        # Show binary size
        if [ -f "target/release/devdust" ]; then
            local binary_size=$(du -h target/release/devdust | cut -f1)
            print_info "Binary size: $binary_size"
        fi
    else
        print_error "Build failed!"
        print_info "Please check the error messages above"
        exit 1
    fi
}

# =============================================================================
# Install Binary Function
# =============================================================================
# Copies the compiled binary to the installation directory
# Makes it globally accessible by adding to PATH
# =============================================================================
install_binary() {
    local install_dir="${1:-$HOME/.local/bin}"
    
    print_step "Installing devdust to $install_dir..."
    
    # Create install directory if it doesn't exist
    if [ ! -d "$install_dir" ]; then
        print_info "Creating install directory: $install_dir"
        mkdir -p "$install_dir"
    fi
    
    # Check if binary exists
    if [ ! -f "target/release/devdust" ]; then
        print_error "Binary not found at target/release/devdust"
        print_info "Please build the project first"
        exit 1
    fi
    
    # Copy the binary to install directory
    cp target/release/devdust "$install_dir/"
    
    # Make binary executable
    chmod +x "$install_dir/devdust"
    
    print_success "devdust installed to $install_dir/devdust"
    
    # Check if install directory is in PATH
    if [[ ":$PATH:" != *":$install_dir:"* ]]; then
        print_warning "$install_dir is not in your PATH"
        echo ""
        print_info "To use 'devdust' command globally, add this line to your shell config:"
        echo ""
        
        # Provide OS-specific instructions
        case "$OS_TYPE" in
            linux|wsl)
                echo -e "    ${GREEN}# For Bash (add to ~/.bashrc):${NC}"
                echo "    export PATH=\"$install_dir:\$PATH\""
                echo ""
                echo -e "    ${GREEN}# For Zsh (add to ~/.zshrc):${NC}"
                echo "    export PATH=\"$install_dir:\$PATH\""
                ;;
            macos)
                echo -e "    ${GREEN}# For Bash (add to ~/.bash_profile):${NC}"
                echo "    export PATH=\"$install_dir:\$PATH\""
                echo ""
                echo -e "    ${GREEN}# For Zsh (add to ~/.zshrc):${NC}"
                echo "    export PATH=\"$install_dir:\$PATH\""
                ;;
            windows)
                echo -e "    ${GREEN}# For Git Bash (add to ~/.bashrc):${NC}"
                echo "    export PATH=\"$install_dir:\$PATH\""
                ;;
        esac
        
        echo ""
        print_info "Then restart your terminal or run: source ~/.bashrc (or ~/.zshrc)"
    else
        print_success "$install_dir is already in your PATH"
        print_success "You can now use 'devdust' command globally!"
    fi
}

# =============================================================================
# Run Tests Function
# =============================================================================
# Executes the test suite for devdust
# =============================================================================
run_tests() {
    print_step "Running tests..."
    echo ""
    
    cargo test
    
    if [ $? -eq 0 ]; then
        print_success "All tests passed!"
    else
        print_warning "Some tests failed, but continuing with installation..."
    fi
}

# =============================================================================
# Uninstall Binary Function
# =============================================================================
# Removes the devdust binary from the installation directory
# =============================================================================
uninstall_binary() {
    local install_dir="${1:-$HOME/.local/bin}"
    local binary_path="$install_dir/devdust"
    
    print_step "Checking for devdust installation..."
    
    # Check if binary exists
    if [ -f "$binary_path" ]; then
        print_info "Found devdust at: $binary_path"
        
        # Confirm uninstallation
        echo -ne "${YELLOW}Are you sure you want to uninstall devdust? [y/N]:${NC} "
        read -r confirm
        
        if [[ "$confirm" =~ ^[Yy]$ ]]; then
            rm "$binary_path"
            print_success "devdust has been uninstalled successfully!"
        else
            print_info "Uninstallation cancelled"
        fi
    else
        print_warning "devdust is not installed at $binary_path"
        print_info "Nothing to uninstall"
    fi
}

# =============================================================================
# Interactive Menu Display
# =============================================================================
# Shows a colorful menu with installation options
# =============================================================================
show_menu() {
    clear
    echo ""
    echo -e "${CYAN}╔═══════════════════════════════════════════════╗${NC}"
    echo -e "${CYAN}║${NC}      ${MAGENTA}⚡ Dev Dust Installation Manager ⚡${NC}      ${CYAN}║${NC}"
    echo -e "${CYAN}╚═══════════════════════════════════════════════╝${NC}"
    echo ""
    echo -e "${GREEN}  1)${NC} 📦 Install devdust"
    echo -e "${RED}  2)${NC} 🗑️  Uninstall devdust"
    echo -e "${YELLOW}  3)${NC} 🚪 Exit"
    echo ""
    echo -e "${BLUE}═══════════════════════════════════════════════${NC}"
    echo -ne "${CYAN}Enter your choice [1-3]:${NC} "
}

# =============================================================================
# Interactive Mode Function
# =============================================================================
# Provides a user-friendly menu loop for installation/uninstallation
# =============================================================================
interactive_mode() {
    local install_dir="$HOME/.local/bin"
    
    # Detect OS once at the start
    detect_os
    
    while true; do
        show_menu
        read -r choice
        
        case $choice in
            1)
                # Install devdust
                echo ""
                echo -e "${CYAN}╔═══════════════════════════════════════════════╗${NC}"
                echo -e "${CYAN}║${NC}           ${GREEN}Starting Installation${NC}               ${CYAN}║${NC}"
                echo -e "${CYAN}╚═══════════════════════════════════════════════╝${NC}"
                echo ""
                
                # Check and install Rust if needed
                check_and_install_rust
                echo ""
                
                # Build the project
                build_project
                echo ""
                
                # Install the binary
                install_binary "$install_dir"
                
                echo ""
                echo -e "${CYAN}╔═══════════════════════════════════════════════╗${NC}"
                echo -e "${CYAN}║${NC}           ${GREEN}Installation Complete!${NC}              ${CYAN}║${NC}"
                echo -e "${CYAN}╚═══════════════════════════════════════════════╝${NC}"
                echo ""
                print_info "Run 'devdust --help' to get started"
                echo ""
                
                read -p "Press Enter to continue..."
                ;;
            2)
                # Uninstall devdust
                echo ""
                echo -e "${CYAN}╔═══════════════════════════════════════════════╗${NC}"
                echo -e "${CYAN}║${NC}          ${RED}Starting Uninstallation${NC}              ${CYAN}║${NC}"
                echo -e "${CYAN}╚═══════════════════════════════════════════════╝${NC}"
                echo ""
                
                # Uninstall the binary
                uninstall_binary "$install_dir"
                
                echo ""
                
                read -p "Press Enter to continue..."
                ;;
            3)
                # Exit
                echo ""
                print_info "Thank you for using devdust installer. Goodbye!"
                echo ""
                exit 0
                ;;
            *)
                # Invalid choice
                echo ""
                print_error "Invalid choice. Please select 1, 2, or 3."
                echo ""
                sleep 2
                ;;
        esac
    done
}

# =============================================================================
# Main Function
# =============================================================================
# Entry point for the script
# Handles both interactive mode and command-line arguments
# =============================================================================
main() {
    # If no arguments provided, run interactive mode
    if [ $# -eq 0 ]; then
        interactive_mode
        exit 0
    fi
    
    # Command-line mode for scripting and automation
    echo ""
    echo "╔═══════════════════════════════════════════════╗"
    echo "║     devdust Local Build & Install             ║"
    echo "╚═══════════════════════════════════════════════╝"
    echo ""
    
    # Detect OS for command-line mode
    detect_os
    echo ""
    
    # Default values for command-line options
    INSTALL_DIR="$HOME/.local/bin"
    RUN_TESTS=false
    SKIP_BUILD=false
    UNINSTALL=false
    AUTO_INSTALL_RUST=false
    
    # Parse command line arguments
    while [[ $# -gt 0 ]]; do
        case $1 in
            --install-dir)
                INSTALL_DIR="$2"
                shift 2
                ;;
            --test)
                RUN_TESTS=true
                shift
                ;;
            --skip-build)
                SKIP_BUILD=true
                shift
                ;;
            --uninstall)
                UNINSTALL=true
                shift
                ;;
            --auto-install-rust)
                AUTO_INSTALL_RUST=true
                shift
                ;;
            --help)
                echo "Usage: ./install.sh [OPTIONS]"
                echo ""
                echo "Options:"
                echo "  --install-dir DIR       Install directory (default: ~/.local/bin)"
                echo "  --test                  Run tests before installing"
                echo "  --skip-build            Skip building (use existing binary)"
                echo "  --uninstall             Uninstall devdust"
                echo "  --auto-install-rust     Automatically install Rust if not found"
                echo "  --help                  Show this help message"
                echo ""
                echo "Interactive Mode:"
                echo "  Run without arguments to use the interactive menu"
                echo ""
                echo "Examples:"
                echo "  ./install.sh                          # Interactive mode"
                echo "  ./install.sh --auto-install-rust      # Auto-install with Rust check"
                echo "  ./install.sh --install-dir /usr/local/bin  # Custom install location"
                echo "  ./install.sh --uninstall              # Remove devdust"
                echo ""
                exit 0
                ;;
            *)
                print_error "Unknown option: $1"
                echo "Use --help for usage information"
                exit 1
                ;;
        esac
    done
    
    # Handle uninstall
    if [ "$UNINSTALL" = true ]; then
        uninstall_binary "$INSTALL_DIR"
        echo ""
        print_success "Uninstallation complete!"
        echo ""
        exit 0
    fi
    
    # Check and install Rust if needed
    if [ "$AUTO_INSTALL_RUST" = true ]; then
        check_and_install_rust
    else
        # Just check, don't auto-install
        print_step "Checking for Rust installation..."
        if ! command -v cargo &> /dev/null; then
            print_error "Rust is not installed!"
            print_info "Please install Rust from https://rustup.rs/"
            print_info "Or run with --auto-install-rust flag"
            exit 1
        fi
        print_success "Rust is installed ($(rustc --version))"
    fi
    echo ""
    
    # Run tests if requested
    if [ "$RUN_TESTS" = true ]; then
        run_tests
        echo ""
    fi
    
    # Build the project
    if [ "$SKIP_BUILD" = false ]; then
        build_project
    else
        print_warning "Skipping build, using existing binary"
        if [ ! -f "target/release/devdust" ]; then
            print_error "No existing binary found at target/release/devdust"
            print_info "Run without --skip-build to build the project"
            exit 1
        fi
    fi
    echo ""
    
    # Install the binary
    install_binary "$INSTALL_DIR"
    
    echo ""
    print_success "Installation complete!"
    print_info "Run 'devdust --help' to get started"
    echo ""
}

# =============================================================================
# Script Execution Entry Point
# =============================================================================
main "$@"
