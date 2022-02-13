#!/bin/bash

set -e

ICON="build/macos/icon_512x512.png"

rm -rf build/macos/AppIcon.iconset
mkdir -p build/macos/AppIcon.iconset
sips -z 16 16     $ICON --out build/macos/AppIcon.iconset/icon_16x16.png
sips -z 32 32     $ICON --out build/macos/AppIcon.iconset/icon_16x16@2x.png
sips -z 32 32     $ICON --out build/macos/AppIcon.iconset/icon_32x32.png
sips -z 64 64     $ICON --out build/macos/AppIcon.iconset/icon_32x32@2x.png
sips -z 128 128   $ICON --out build/macos/AppIcon.iconset/icon_128x128.png
sips -z 256 256   $ICON --out build/macos/AppIcon.iconset/icon_128x128@2x.png
sips -z 256 256   $ICON --out build/macos/AppIcon.iconset/icon_256x256.png
sips -z 512 512   $ICON --out build/macos/AppIcon.iconset/icon_256x256@2x.png
sips -z 512 512   $ICON --out build/macos/AppIcon.iconset/icon_512x512.png
cp $ICON build/macos/AppIcon.iconset/icon_512x512@2x.png
iconutil -c icns build/macos/AppIcon.iconset
mkdir -p build/macos/src/Game.app/Contents/Resources
mv build/macos/AppIcon.icns build/macos/src/Game.app/Contents/Resources/
