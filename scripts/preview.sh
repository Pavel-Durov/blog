#!/bin/bash -e

for _ in {1..10}
do
    echo " "
done

FONT="Banner4"
THEME="Dark+"

ascii-themes generate --font="${FONT}" --themeName=${THEME} --text="   Loops"
ascii-themes generate --font="${FONT}" --themeName=${THEME} --text="   and"
ascii-themes generate --font="${FONT}" --themeName=${THEME} --text="   Opcodes   in   Lua"

# echo ""
# echo ""
# echo "$ subtitle"
# echo ""
# echo "  Application deployment strategies: configurable and gradual deployments with minimised downtime."

for _ in {1..10}
do
    echo " "
done
