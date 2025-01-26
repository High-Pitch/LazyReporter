date=$(date -u +%F)
api=$(cat ~/Documents/api.txt)
curl "$api" > $date-WeatherReport.json
