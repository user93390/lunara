#!/bin/bash

packages=(
    "git" 
    "curl"
    "gnome-keyring"
    "just"
    "docker"
    "docker-compose"
)

for value in "${packages[@]}"
do
  echo "Installing $value"

  sudo apk add "$value"
done

OPENSSL_NO_VENDOR=1 cargo install cargo-leptos

echo "Configuring docker permissions..."
sudo addgroup "$(whoami)" docker

echo "Complete!"