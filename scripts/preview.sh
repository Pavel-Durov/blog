#!/bin/bash -e

for _ in {1..10}
do
    echo " "
done

FONT="DOS Rebel"
THEME=""

ascii-themes generate --font="${FONT}" --themeName=${THEME} --text='   "Hello, World!"'
ascii-themes generate --font="${FONT}" --themeName=${THEME} --text="   with"
ascii-themes generate --font="${FONT}" --themeName=${THEME} --text="   Make and CMake"

# echo ""
# echo ""
# echo "$ subtitle"
# echo ""
# echo "  Application deployment strategies: configurable and gradual deployments with minimised downtime."

for _ in {1..10}
do
    echo " "
done
