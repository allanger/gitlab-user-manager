#!/bin/bash
case "$(uname)" in

"Darwin")
  SYSTEM="apple-darwin"
  case $(uname -m) in
  "arm64")
    TARGET="aarch64-$SYSTEM"
    ;;
  "x68_64")
    TARGET="x86_64-$SYSTEM"
    ;;
  *)
    echo "Unsuported target"
    exit 1
    ;;
  esac
  ;;
"Linux")
  SYSTEM="unknown-linux-gnu"
  case $(uname -m) in
  "x68_64")
    TARGET="x86_64-$SYSTEM"
    ;;
  *)
    echo "Unsuported target"
    exit 1
    ;;
  esac
  ;;
*)
  echo "Signal number $1 is not processed"
  exit 1
  ;;
esac
LATEST_VERSION=$(curl https://allanger.github.io/gitlab-user-manager/)
curl "https://github.com/allanger/gitlab-user-manager/releases/download/$LATEST_VERSION/gum-$LATEST_VERSION-$TARGET"
