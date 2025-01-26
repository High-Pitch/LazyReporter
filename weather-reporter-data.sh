date=$(date +%F)
api=$(cat ~/api.txt)
curl "$api" > $date-WeatherReport.json
mv $date-WeatherReport.json weather_data/
