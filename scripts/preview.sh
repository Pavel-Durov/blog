#!/bin/bash -e

for _ in {1..10}
do
    echo " "
done

echo "$ title"
echo ""
ascii-themes generate --font='Cyberlarge' --themeName='Dark+' --text="Progressive"
ascii-themes generate --font='Cyberlarge' --themeName='Dark+' --text="Delivery"
echo ""
echo ""
echo "$ subtitle"
echo "Application deployment strategies: configurable and gradual deployments with minimised downtime."

for _ in {1..10}
do
    echo " "
done
