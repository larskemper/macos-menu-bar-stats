#!/bin/bash

# Script to bump version across all project files

set -e

BOLD='\033[1m'
GREEN='\033[0;32m'
CYAN='\033[0;36m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m'

NEW_VERSION=$1

if [ -z "$NEW_VERSION" ]; then
    echo ""
    echo -e "${RED}Error: No version specified${NC}"
    echo ""
    echo -e "${BOLD}Usage:${NC}"
    echo -e "  ./scripts/bump-version.sh ${CYAN}<version>${NC}"
    echo ""
    echo -e "${BOLD}Example:${NC}"
    echo -e "  ./scripts/bump-version.sh 0.2.0"
    echo ""
    exit 1
fi

# Validate version format (basic check)
if ! [[ "$NEW_VERSION" =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
    echo ""
    echo -e "${RED}Error: Invalid version format${NC}"
    echo -e "Version must be in format: ${CYAN}X.Y.Z${NC} (e.g., 0.2.0)"
    echo ""
    exit 1
fi

echo ""
echo -e "${BOLD}${CYAN}╭──────────────────────────────────────────────────────╮${NC}"
echo -e "${BOLD}${CYAN}│${NC}              ${BOLD}Bumping Version${NC}                     ${BOLD}${CYAN}│${NC}"
echo -e "${BOLD}${CYAN}╰──────────────────────────────────────────────────────╯${NC}"
echo ""

# Get current version from Cargo.toml
CURRENT_VERSION=$(grep '^version = ' Cargo.toml | head -n 1 | sed 's/version = "\(.*\)"/\1/')

echo -e "${BOLD}Current version:${NC} ${YELLOW}${CURRENT_VERSION}${NC}"
echo -e "${BOLD}New version:${NC}     ${GREEN}${NEW_VERSION}${NC}"
echo ""

# Check if there are uncommitted changes
if ! git diff-index --quiet HEAD -- 2>/dev/null; then
    echo -e "${YELLOW}⚠ Warning: You have uncommitted changes${NC}"
    echo ""
    read -p "Continue anyway? (y/N): " -n 1 -r
    echo ""
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        echo -e "${RED}Aborted${NC}"
        exit 1
    fi
    echo ""
fi

echo -e "${CYAN}─────────────────────────────────────────────────────${NC}"
echo ""

# Update Cargo.toml
echo -e "${BOLD}Updating files...${NC}"
echo ""

if sed -i '' "s/^version = \".*\"/version = \"$NEW_VERSION\"/" Cargo.toml; then
    echo -e "${GREEN}✓${NC} Updated Cargo.toml"
else
    echo -e "${RED}✗${NC} Failed to update Cargo.toml"
    exit 1
fi

# Update tauri.conf.json
if sed -i '' "s/\"version\": \".*\"/\"version\": \"$NEW_VERSION\"/" tauri.conf.json; then
    echo -e "${GREEN}✓${NC} Updated tauri.conf.json"
else
    echo -e "${RED}✗${NC} Failed to update tauri.conf.json"
    exit 1
fi

# Update Cargo.lock
echo -e "${CYAN}ℹ${NC} Updating Cargo.lock..."
cargo update -p system-stats 2>/dev/null || cargo check --quiet 2>/dev/null || true
echo -e "${GREEN}✓${NC} Updated Cargo.lock"

echo ""
echo -e "${CYAN}─────────────────────────────────────────────────────${NC}"
echo ""

echo -e "${BOLD}${GREEN}✓ Version successfully bumped to ${NEW_VERSION}${NC}"
echo ""

echo -e "${BOLD}Next steps:${NC}"
echo ""
echo -e "  ${CYAN}1.${NC} Review changes:"
echo -e "     ${YELLOW}git diff${NC}"
echo ""
echo -e "  ${CYAN}2.${NC} Commit changes:"
echo -e "     ${YELLOW}git add Cargo.toml tauri.conf.json Cargo.lock${NC}"
echo -e "     ${YELLOW}git commit -m \"Bump version to ${NEW_VERSION}\"${NC}"
echo ""
echo -e "  ${CYAN}3.${NC} Create release:"
echo -e "     ${YELLOW}./scripts/build-release.sh${NC}"
echo ""
echo -e "  ${CYAN}4.${NC} Create tag and push:"
echo -e "     ${YELLOW}git tag -a v${NEW_VERSION} -m \"Release v${NEW_VERSION}\"${NC}"
echo -e "     ${YELLOW}git push origin main --tags${NC}"
echo ""
echo -e "  ${CYAN}5.${NC} Or let GitHub Actions handle the release:"
echo -e "     ${YELLOW}git push origin v${NEW_VERSION}${NC}"
echo ""

