#!/bin/bash

packages=(
    "git" 
    "curl"
    "gnome-keyring"
    "just"
    "docker"
    "gcompat"
    "libstdc++"
    "docker-compose"
)

for value in "${packages[@]}"
do
  echo "Installing $value"

  sudo apk add $value
done

echo "Configuring docker permissions..."
sudo addgroup $(whoami) docker

echo "Starting rust initilization"

rustup component add rust-docs rust-std

echo "Complete!"