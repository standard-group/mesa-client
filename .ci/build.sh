#!/bin/sh

echo "mesa build tool (non-interactive)"
BUILD_TYPE="${MESA_BUILD_TYPE:-release}"

case "$BUILD_TYPE" in
  release)
    echo "Building Stable..."
    ;;
  beta)
    echo "Building Beta..."
    ;;
  nightly)
    echo "Building Nightly..."
    ;;
  debug)
    echo "Building Debug..."
    ;;
  internal)
    echo "Building Dev Build..."
    ;;
  *)
    echo "Invalid MESA_BUILD_TYPE: $BUILD_TYPE"
    exit 1
    ;;
esac

echo ""
echo "MESA_BUILD_TYPE=$BUILD_TYPE"
echo "Running: pnpm tauri build"
echo ""

pnpm tauri build

unset MESA_BUILD_TYPE
