#!/bin/bash -e

for _ in {1..10}
do
    echo " "
done

FONT="NV Script"
THEME="Dark+"

ascii-themes generate --font="${FONT}" --themeName=${THEME} --text="Programming"
ascii-themes generate --font="${FONT}" --themeName=${THEME} --text="Languages"
ascii-themes generate --font="${FONT}" --themeName=${THEME} --text="Composition"

# echo ""
# echo ""
# echo "$ subtitle"
# echo ""
# echo "  Application deployment strategies: configurable and gradual deployments with minimised downtime."

for _ in {1..10}
do
    echo " "
done
