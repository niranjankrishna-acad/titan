#!/bin/bash

# Get the current directory
DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"

# Read the template and replace placeholders
sed -e "s|__EXECSTART__|$DIR/target/release/titan|" \
    -e "s|__WORKINGDIR__|$DIR|" \
    $DIR/titan.service.template > $DIR/titan.service

echo "titan.service file has been created/updated."
