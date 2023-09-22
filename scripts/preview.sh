#!/bin/bash -e

for _ in {1..10}
do
    echo " "
done

FONT="Jacky"
THEME=""

ascii-themes generate --font="${FONT}" --themeName=${THEME} --text='   SDK DX '
ascii-themes generate --font="${FONT}" --themeName=${THEME} --text="   and"
ascii-themes generate --font="${FONT}" --themeName=${THEME} --text="   Code Evolution"

# echo ""
# echo ""
# echo "$ subtitle"
# echo ""
# echo "  Application deployment strategies: configurable and gradual deployments with minimised downtime."

for _ in {1..10}
do
    echo " "
done
