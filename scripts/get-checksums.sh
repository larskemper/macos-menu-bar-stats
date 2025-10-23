#!/bin/bash

# Script to calculate SHA256 checksums for DMG files
# Used when updating the Homebrew cask formula

set -e

BOLD='\033[1m'
GREEN='\033[0;32m'
CYAN='\033[0;36m'
YELLOW='\033[1;33m'
NC='\033[0m'

echo ""
echo -e "${BOLD}${CYAN}╭──────────────────────────────────────────────────────╮${NC}"
echo -e "${BOLD}${CYAN}│${NC}         ${BOLD}System Stats - SHA256 Checksums${NC}          ${BOLD}${CYAN}│${NC}"
echo -e "${BOLD}${CYAN}╰──────────────────────────────────────────────────────╯${NC}"
echo ""

if [ ! -f "release/system-stats-aarch64.dmg" ]; then
    echo -e "${YELLOW}⚠ Warning: release/system-stats-aarch64.dmg not found${NC}"
    echo ""
fi

if [ ! -f "release/system-stats-x86_64.dmg" ]; then
    echo -e "${YELLOW}⚠ Warning: release/system-stats-x86_64.dmg not found${NC}"
    echo ""
fi

if [ ! -f "release/system-stats-aarch64.dmg" ] && [ ! -f "release/system-stats-x86_64.dmg" ]; then
    echo -e "${YELLOW}Run ./scripts/build-release.sh first to build the DMG files${NC}"
    echo ""
    exit 1
fi

echo -e "${BOLD}SHA256 Checksums:${NC}"
echo ""

if [ -f "release/system-stats-aarch64.dmg" ]; then
    AARCH64_SHA=$(shasum -a 256 release/system-stats-aarch64.dmg | awk '{print $1}')
    echo -e "${GREEN}●${NC} ${BOLD}Apple Silicon (aarch64):${NC}"
    echo -e "  ${AARCH64_SHA}"
    echo ""
fi

if [ -f "release/system-stats-x86_64.dmg" ]; then
    X86_64_SHA=$(shasum -a 256 release/system-stats-x86_64.dmg | awk '{print $1}')
    echo -e "${GREEN}●${NC} ${BOLD}Intel (x86_64):${NC}"
    echo -e "  ${X86_64_SHA}"
    echo ""
fi

echo -e "${CYAN}─────────────────────────────────────────────────────${NC}"
echo ""
echo -e "${BOLD}Update your Homebrew cask formula:${NC}"
echo ""
echo -e "${CYAN}sha256${NC} arm:   ${YELLOW}\"${AARCH64_SHA}\"${NC},"
echo -e "       intel: ${YELLOW}\"${X86_64_SHA}\"${NC}"
echo ""

