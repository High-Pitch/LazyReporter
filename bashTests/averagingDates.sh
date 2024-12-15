#!/usr/bin/env bash

rainAverage=0;
temp=0;
date=$(date +%Y-%m-%d);
echo "$date";
for ((i = 0; i < 8; i++)); do
	jq '.list' 5dayHansIslandWeather.json | jq ".[$i].pop" | read temp;
	rainAverage=$(($rainAverage + temp));
	echo "$rainAverage";
done
echo "$rainAverage";
