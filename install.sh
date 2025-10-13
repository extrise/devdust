#!/usr/bin/env bash

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Print colored messages
print_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Check if Rust is installed
check_rust() {
    if ! command -v cargo &> /dev/null; then
        print_error "Rust is not installed!"
        print_info "Please install Rust from https://rustup.rs/"
        exit 1
    fi
    print_success "Rust is installed ($(rustc --version))"
}

# Build the project
build_project() {
    print_info "Building devdust in release mode..."
    cargo build --release -p devdust
    
    if [ $? -eq 0 ]; then
        print_success "Build completed successfully!"
    else
        print_error "Build failed!"
        exit 1
    fi
}

# Install the binary
install_binary() {
    local install_dir="${1:-$HOME/.local/bin}"
    
    # Create install directory if it doesn't exist
    if [ ! -d "$install_dir" ]; then
        print_info "Creating install directory: $install_dir"
        mkdir -p "$install_dir"
    fi
    
    # Copy the binary
    print_info "Installing devdust to $install_dir..."
    cp target/release/devdust "$install_dir/"
    chmod +x "$install_dir/devdust"
    
    print_success "devdust installed to $install_dir/devdust"
    
    # Check if install directory is in PATH
    if [[ ":$PATH:" != *":$install_dir:"* ]]; then
        print_warning "$install_dir is not in your PATH"
        print_info "Add the following line to your ~/.bashrc or ~/.zshrc:"
        echo ""
        echo "    export PATH=\"$install_dir:\$PATH\""
        echo ""
    else
        print_success "$install_dir is already in your PATH"
    fi
}

# Run tests
run_tests() {
    print_info "Running tests..."
    cargo test
    
    if [ $? -eq 0 ]; then
        print_success "All tests passed!"
    else
        print_warning "Some tests failed, but continuing with installation..."
    fi
}

# Main installation flow
main() {
    echo ""
    echo "╔═══════════════════════════════════════╗"
    echo "║   devdust Local Build & Install       ║"
    echo "╚═══════════════════════════════════════╝"
    echo ""
    
    # Parse command line arguments
    INSTALL_DIR="$HOME/.local/bin"
    RUN_TESTS=false
    SKIP_BUILD=false
    
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
            --help)
                echo "Usage: ./install.sh [OPTIONS]"
                echo ""
                echo "Options:"
                echo "  --install-dir DIR    Install directory (default: ~/.local/bin)"
                echo "  --test               Run tests before installing"
                echo "  --skip-build         Skip building (use existing binary)"
                echo "  --help               Show this help message"
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
    
    # Check prerequisites
    check_rust
    
    # Run tests if requested
    if [ "$RUN_TESTS" = true ]; then
        run_tests
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
    
    # Install the binary
    install_binary "$INSTALL_DIR"
    
    echo ""
    print_success "Installation complete!"
    print_info "Run 'devdust --help' to get started"
    echo ""
}

# Run main function
main "$@"
