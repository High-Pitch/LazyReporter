#!/usr/bin/env bash
#Enabling the notify-send to work
#eval "export $(egrep -z DBUS_SESSION_BUS_ADDRESS /proc/$(pgrep -u $LOGNAME gnome-session)/environ)";
#Run python script and establish variables for file system
usr=$(whoami)
date=$(date +%F)
date+="WeatherReport.json"
msg=$(python3 /home/$usr/code/weatherReport/weatherReport.py);
#Display the message
DISPLAY=:0 notify-send -u critical -i ~/Pictures/emotes/coltonGlasses.png "Weather Report" "$msg"
mv $date /home/$usr/Documents/weatherData
