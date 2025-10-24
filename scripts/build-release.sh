#!/bin/bash

# Build script for System Stats macOS app
# Creates DMG files for both Intel (x86_64) and Apple Silicon (aarch64)

set -e

BOLD='\033[1m'
GREEN='\033[0;32m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
MAGENTA='\033[0;35m'
NC='\033[0m'

TOP_LEFT="╭"
TOP_RIGHT="╮"
BOTTOM_LEFT="╰"
BOTTOM_RIGHT="╯"
HORIZONTAL="─"
VERTICAL="│"

print_header() {
    local text="$1"
    local length=${#text}
    local total_width=60
    local padding=$(( (total_width - length - 2) / 2 ))
    
    echo ""
    echo -e "${CYAN}${TOP_LEFT}$(printf '%*s' $total_width | tr ' ' "$HORIZONTAL")${TOP_RIGHT}${NC}"
    printf "${CYAN}${VERTICAL}${NC}%*s" $padding
    echo -e "${BOLD}${BLUE}$text${NC}$(printf '%*s' $padding)${CYAN}${VERTICAL}${NC}"
    echo -e "${CYAN}${BOTTOM_LEFT}$(printf '%*s' $total_width | tr ' ' "$HORIZONTAL")${BOTTOM_RIGHT}${NC}"
    echo ""
}

print_step() {
    echo -e "${BOLD}${MAGENTA}▶${NC} ${BOLD}$1${NC}"
    echo ""
}

print_success() {
    echo -e "  ${GREEN}✓${NC} $1"
}

print_error() {
    echo -e "  ${RED}✗${NC} $1"
}

print_info() {
    echo -e "  ${CYAN}ℹ${NC} $1"
}

print_separator() {
    echo -e "${CYAN}$(printf '%*s' 62 | tr ' ' "─")${NC}"
}

clear
print_header "System Stats - Release Build"

echo -e "${BOLD}Building for:${NC}"
print_info "Apple Silicon (aarch64-apple-darwin)"
print_info "Intel (x86_64-apple-darwin)"
echo ""
print_separator

# Check if running on macOS
print_step "Checking prerequisites..."

if [[ "$OSTYPE" != "darwin"* ]]; then
    print_error "This script must be run on macOS"
    exit 1
fi 
print_success "Running on macOS"

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    print_error "Rust/Cargo is not installed"
    echo ""
    echo -e "${YELLOW}Please install Rust from https://rustup.rs/${NC}"
    exit 1
fi
print_success "Rust/Cargo installed"

# Check if Tauri CLI is available
if ! cargo tauri --version &> /dev/null; then
    echo ""
    print_info "Tauri CLI not found. Installing..."
    cargo install tauri-cli --version "^2.0" --locked
    print_success "Tauri CLI installed"
else
    print_success "Tauri CLI available"
fi

echo ""
print_separator

# Clean previous builds
print_step "Cleaning previous builds..."
cargo clean 2>/dev/null || true
print_success "Build cache cleaned"

echo ""
print_separator

# Create release directory if it doesn't exist
mkdir -p release

# Build for Apple Silicon (aarch64)
print_step "Building for Apple Silicon (aarch64)"
echo -e "${CYAN}${HORIZONTAL}${HORIZONTAL}${HORIZONTAL}${HORIZONTAL}${HORIZONTAL}${HORIZONTAL}${HORIZONTAL}${HORIZONTAL}${HORIZONTAL}${HORIZONTAL}${HORIZONTAL}${HORIZONTAL}${HORIZONTAL}${HORIZONTAL}${HORIZONTAL}${HORIZONTAL}${HORIZONTAL}${HORIZONTAL}${HORIZONTAL}${HORIZONTAL}${HORIZONTAL}${HORIZONTAL}${HORIZONTAL}${HORIZONTAL}${HORIZONTAL}${HORIZONTAL}${HORIZONTAL}${HORIZONTAL}${HORIZONTAL}${HORIZONTAL}${HORIZONTAL}${HORIZONTAL}${HORIZONTAL}${HORIZONTAL}${HORIZONTAL}${HORIZONTAL}${HORIZONTAL}${HORIZONTAL}${HORIZONTAL}${HORIZONTAL}${HORIZONTAL}${HORIZONTAL}${HORIZONTAL}${HORIZONTAL}${HORIZONTAL}${HORIZONTAL}${HORIZONTAL}${HORIZONTAL}${HORIZONTAL}${HORIZONTAL}${HORIZONTAL}${HORIZONTAL}${HORIZONTAL}${HORIZONTAL}${HORIZONTAL}${HORIZONTAL}${HORIZONTAL}${HORIZONTAL}${HORIZONTAL}${HORIZONTAL}${HORIZONTAL}${HORIZONTAL}${NC}"
echo ""

START_TIME=$(date +%s)
cargo tauri build --target aarch64-apple-darwin
END_TIME=$(date +%s)
DURATION=$((END_TIME - START_TIME))

echo ""
print_success "Apple Silicon build completed in ${DURATION}s"

echo ""
print_separator

# Build for Intel (x86_64)
print_step "Building for Intel (x86_64)"
echo -e "${CYAN}${HORIZONTAL}${HORIZONTAL}${HORIZONTAL}${HORIZONTAL}${HORIZONTAL}${HORIZONTAL}${HORIZONTAL}${HORIZONTAL}${HORIZONTAL}${HORIZONTAL}${HORIZONTAL}${HORIZONTAL}${HORIZONTAL}${HORIZONTAL}${HORIZONTAL}${HORIZONTAL}${HORIZONTAL}${HORIZONTAL}${HORIZONTAL}${HORIZONTAL}${HORIZONTAL}${HORIZONTAL}${HORIZONTAL}${HORIZONTAL}${HORIZONTAL}${HORIZONTAL}${HORIZONTAL}${HORIZONTAL}${HORIZONTAL}${HORIZONTAL}${HORIZONTAL}${HORIZONTAL}${HORIZONTAL}${HORIZONTAL}${HORIZONTAL}${HORIZONTAL}${HORIZONTAL}${HORIZONTAL}${HORIZONTAL}${HORIZONTAL}${HORIZONTAL}${HORIZONTAL}${HORIZONTAL}${HORIZONTAL}${HORIZONTAL}${HORIZONTAL}${HORIZONTAL}${HORIZONTAL}${HORIZONTAL}${HORIZONTAL}${HORIZONTAL}${HORIZONTAL}${HORIZONTAL}${HORIZONTAL}${HORIZONTAL}${HORIZONTAL}${HORIZONTAL}${HORIZONTAL}${HORIZONTAL}${HORIZONTAL}${HORIZONTAL}${HORIZONTAL}${NC}"
echo ""

START_TIME=$(date +%s)
cargo tauri build --target x86_64-apple-darwin
END_TIME=$(date +%s)
DURATION=$((END_TIME - START_TIME))

echo ""
print_success "Intel build completed in ${DURATION}s"

echo ""
print_separator

# Copy DMG files to release directory
print_step "Packaging release files..."

AARCH64_DMG=$(find target/aarch64-apple-darwin/release/bundle/dmg -name "*.dmg" 2>/dev/null | head -n 1)
X86_64_DMG=$(find target/x86_64-apple-darwin/release/bundle/dmg -name "*.dmg" 2>/dev/null | head -n 1)

ARM_SUCCESS=false
INTEL_SUCCESS=false

if [ -f "$AARCH64_DMG" ]; then
    cp "$AARCH64_DMG" "release/system-stats-aarch64.dmg"
    print_success "Apple Silicon DMG → release/system-stats-aarch64.dmg"
    ARM_SUCCESS=true
else
    print_error "Apple Silicon DMG not found"
fi

if [ -f "$X86_64_DMG" ]; then
    cp "$X86_64_DMG" "release/system-stats-x86_64.dmg"
    print_success "Intel DMG → release/system-stats-x86_64.dmg"
    INTEL_SUCCESS=true
else
    print_error "Intel DMG not found"
fi

echo ""
print_separator

# Remove quarantine flags and rebuild DMGs
print_step "Removing quarantine flags..."

# Remove quarantine from .app bundles in the target directories
AARCH64_APP=$(find target/aarch64-apple-darwin/release/bundle/macos -name "*.app" 2>/dev/null | head -n 1)
X86_64_APP=$(find target/x86_64-apple-darwin/release/bundle/macos -name "*.app" 2>/dev/null | head -n 1)

if [ -d "$AARCH64_APP" ]; then
    xattr -dr com.apple.quarantine "$AARCH64_APP" 2>/dev/null || true
    print_success "Removed quarantine from Apple Silicon .app bundle"
    
    # Rebuild DMG with cleaned app
    if [ -f "release/system-stats-aarch64.dmg" ]; then
        print_info "Rebuilding Apple Silicon DMG with clean app..."
        
        # Create temporary directory
        TEMP_DIR=$(mktemp -d)
        cp -R "$AARCH64_APP" "$TEMP_DIR/"
        
        # Create new DMG
        rm -f "release/system-stats-aarch64.dmg"
        hdiutil create -volname "System Stats" \
            -srcfolder "$TEMP_DIR" \
            -ov -format UDZO \
            "release/system-stats-aarch64.dmg" > /dev/null 2>&1
        
        # Clean up
        rm -rf "$TEMP_DIR"
        
        # Remove quarantine from the new DMG file itself
        xattr -dr com.apple.quarantine "release/system-stats-aarch64.dmg" 2>/dev/null || true
        print_success "Rebuilt Apple Silicon DMG without quarantine"
    fi
fi

if [ -d "$X86_64_APP" ]; then
    xattr -dr com.apple.quarantine "$X86_64_APP" 2>/dev/null || true
    print_success "Removed quarantine from Intel .app bundle"
    
    # Rebuild DMG with cleaned app
    if [ -f "release/system-stats-x86_64.dmg" ]; then
        print_info "Rebuilding Intel DMG with clean app..."
        
        # Create temporary directory
        TEMP_DIR=$(mktemp -d)
        cp -R "$X86_64_APP" "$TEMP_DIR/"
        
        # Create new DMG
        rm -f "release/system-stats-x86_64.dmg"
        hdiutil create -volname "System Stats" \
            -srcfolder "$TEMP_DIR" \
            -ov -format UDZO \
            "release/system-stats-x86_64.dmg" > /dev/null 2>&1
        
        # Clean up
        rm -rf "$TEMP_DIR"
        
        # Remove quarantine from the new DMG file itself
        xattr -dr com.apple.quarantine "release/system-stats-x86_64.dmg" 2>/dev/null || true
        print_success "Rebuilt Intel DMG without quarantine"
    fi
fi

echo ""
print_separator

# Verify quarantine flags have been removed
print_step "Verifying quarantine removal..."

VERIFICATION_FAILED=false

# Function to check DMG and its contents
verify_dmg() {
    local DMG_PATH="$1"
    local ARCH_NAME="$2"
    
    # Check DMG file itself
    if xattr -p com.apple.quarantine "$DMG_PATH" 2>/dev/null; then
        print_error "$ARCH_NAME DMG file has quarantine flag!"
        return 1
    fi
    
    # Mount DMG and check .app inside
    print_info "Mounting $ARCH_NAME DMG to verify contents..."
    MOUNT_POINT=$(hdiutil attach "$DMG_PATH" -nobrowse -readonly | grep Volumes | sed 's/.*\/Volumes/\/Volumes/')
    
    if [ -n "$MOUNT_POINT" ]; then
        # Find .app in mounted volume
        APP_IN_DMG=$(find "$MOUNT_POINT" -name "*.app" -maxdepth 1 2>/dev/null | head -n 1)
        
        if [ -d "$APP_IN_DMG" ]; then
            # Check for any quarantine flags in the app bundle
            if xattr -lr "$APP_IN_DMG" 2>/dev/null | grep -q "com.apple.quarantine"; then
                print_error "$ARCH_NAME app inside DMG has quarantine flags!"
                hdiutil detach "$MOUNT_POINT" -quiet 2>/dev/null || true
                return 1
            fi
        fi
        
        # Unmount
        hdiutil detach "$MOUNT_POINT" -quiet 2>/dev/null || true
    fi
    
    print_success "$ARCH_NAME DMG verified - no quarantine flags"
    return 0
}

if [ -f "release/system-stats-aarch64.dmg" ]; then
    if ! verify_dmg "release/system-stats-aarch64.dmg" "Apple Silicon"; then
        VERIFICATION_FAILED=true
    fi
fi

if [ -f "release/system-stats-x86_64.dmg" ]; then
    if ! verify_dmg "release/system-stats-x86_64.dmg" "Intel"; then
        VERIFICATION_FAILED=true
    fi
fi

if [ "$VERIFICATION_FAILED" = true ]; then
    echo ""
    print_error "Quarantine verification failed! DMGs may trigger Gatekeeper warnings."
    exit 1
fi

echo ""
print_separator

# Display results
echo ""
print_header "Build Complete!"

if [ "$ARM_SUCCESS" = true ] || [ "$INTEL_SUCCESS" = true ]; then
    echo -e "${BOLD}${GREEN}Release files:${NC}"
    echo ""
    
    if [ -f "release/system-stats-aarch64.dmg" ]; then
        SIZE_ARM=$(du -h "release/system-stats-aarch64.dmg" | cut -f1)
        echo -e "  ${GREEN}●${NC} ${BOLD}system-stats-aarch64.dmg${NC}"
        echo -e "    ${CYAN}├─${NC} Architecture: Apple Silicon (ARM64)"
        echo -e "    ${CYAN}└─${NC} Size: ${SIZE_ARM}"
        echo ""
    fi
    
    if [ -f "release/system-stats-x86_64.dmg" ]; then
        SIZE_INTEL=$(du -h "release/system-stats-x86_64.dmg" | cut -f1)
        echo -e "  ${GREEN}●${NC} ${BOLD}system-stats-x86_64.dmg${NC}"
        echo -e "    ${CYAN}├─${NC} Architecture: Intel (x86_64)"
        echo -e "    ${CYAN}└─${NC} Size: ${SIZE_INTEL}"
        echo ""
    fi
    
    echo -e "${CYAN}${TOP_LEFT}${HORIZONTAL}${HORIZONTAL} ${NC}${BOLD}Distribution Guide${NC} ${CYAN}${HORIZONTAL}${HORIZONTAL}${HORIZONTAL}${HORIZONTAL}${HORIZONTAL}${HORIZONTAL}${HORIZONTAL}${HORIZONTAL}${HORIZONTAL}${HORIZONTAL}${HORIZONTAL}${HORIZONTAL}${HORIZONTAL}${HORIZONTAL}${HORIZONTAL}${HORIZONTAL}${HORIZONTAL}${HORIZONTAL}${HORIZONTAL}${HORIZONTAL}${HORIZONTAL}${HORIZONTAL}${HORIZONTAL}${HORIZONTAL}${HORIZONTAL}${HORIZONTAL}${HORIZONTAL}${HORIZONTAL}${HORIZONTAL}${HORIZONTAL}${HORIZONTAL}${HORIZONTAL}${HORIZONTAL}${HORIZONTAL}${HORIZONTAL}${HORIZONTAL}${HORIZONTAL}${HORIZONTAL}${TOP_RIGHT}${NC}"
    echo -e "${CYAN}${VERTICAL}${NC}                                                            ${CYAN}${VERTICAL}${NC}"
*    echo -e "${CYAN}${VERTICAL}${NC}  ${YELLOW}Apple Silicon DMG${NC} - For M1, M2, M3, M4 Macs            ${CYAN}${VERTICAL}${NC}"
    echo -e "${CYAN}${VERTICAL}${NC}  ${YELLOW}Intel DMG${NC}         - For Intel-based Macs               ${CYAN}${VERTICAL}${NC}"
    echo -e "${CYAN}${VERTICAL}${NC}                                                            ${CYAN}${VERTICAL}${NC}"
    echo -e "${CYAN}${BOTTOM_LEFT}$(printf '%*s' 60 | tr ' ' "$HORIZONTAL")${BOTTOM_RIGHT}${NC}"
    echo ""
else
    echo -e "${RED}No DMG files were created successfully.${NC}"
    echo ""
fi
