#!/bin/bash -e

for _ in {1..10}
do
    echo " "
done

FONT="ANSI Regular"
THEME="Dark+"

ascii-themes generate --font="${FONT}" --themeName=${THEME} --text="Cross Platform"
ascii-themes generate --font="${FONT}" --themeName=${THEME} --text="CLI applications"
ascii-themes generate --font="${FONT}" --themeName=${THEME} --text="With GO and Cobra"

# echo ""
# echo ""
# echo "$ subtitle"
# echo ""
# echo "  Application deployment strategies: configurable and gradual deployments with minimised downtime."

for _ in {1..10}
do
    echo " "
done
