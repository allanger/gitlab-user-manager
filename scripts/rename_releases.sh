#!/bin/bash
echo 'renaming gum to gum-$VERSION-$SYSTEM format'
mkdir -p release
echp "version - $GUM_VERSION"
for BUILD in build*; do
  echo "system - $SYSTEM"
  SYSTEM=$echo $BUILD | sed -e 's/build-//g')
  mv $BUILD/gum release/gum-$GUM_VERSION-$SYSTEM
done
ls release
