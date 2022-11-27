#!/usr/bin/env bash

BASE_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )

if ! command -v rustup &> /dev/null
then
  echo "Rustup not detected! Do you want to install Rustup and Rust now? (y/N)"
  read yn

  case $yn in
    [yY]*) curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh;;
    [nN]*) echo "Aborting setup."; exit ;;
  esac
fi

rustup component add clippy
rustup component add rustfmt

ln -s $BASE_DIR/.git_hooks/* $BASE_DIR/.git/hooks/