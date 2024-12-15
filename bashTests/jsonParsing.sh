#!/usr/bin/env bash

temp=$(jq '.main.temp' $1);
feels=$(jq '.main.feels_like' $1);



echo "Temp is $temp but feels like $feels";
