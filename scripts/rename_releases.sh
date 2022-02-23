#!/bin/bash
echo 'renaming gum to gum-$VERSION-$SYSTEM format'
mkdir -p release
echo "version - $GUM_VERSION"
for BUILD in build*; do
  SYSTEM=$(echo $BUILD | sed -e 's/build-//g')
  echo "system - $SYSTEM"
  cp $BUILD/gum release/gum-$GUM_VERSION-$SYSTEM
done
ls release
