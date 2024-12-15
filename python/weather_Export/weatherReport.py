#!/usr/bin/env python3

import requests
import json
import math
import datetime
import api

code = 0
data = []
today = datetime.date.today()
#Weather recommendation markers
precLow = 0.4
precHigh = 0.6
tempLow = 7
tempMid = 15
tempHigh = 20


def getData():
    #API pull and response
    global code
    try:
        response = requests.get(api.key)
        code = 0
        if response.status_code == 200:
            return response
    except:
        code = 3
        return None

def processData(response):
    #Averaging data and appending to list used in recommender
    global code
    try:
        temp = 0
        pop = 0
        amountToAverage = 6
        returnList = []
        #Grab the next 24 hours of temp and precepitation and average it
        for x in range(amountToAverage):
            data = response.json()
            temp += data["list"][x]["main"]["temp"]
            pop += data["list"][x]["pop"]
        highOf = data["list"][0]["main"]["temp_max"]
        pop /= amountToAverage
        temp /= amountToAverage
        #Add to returning list
        returnList.append(temp)
        returnList.append(pop)
        returnList.append(highOf)
        returnList.append(data)
        code = 0
        return returnList
    except:
        code = 1
        return None

def recommender(temp, prec, highOf):
    #Constructs the recommendation string
    #Go to top to tune recommendations
    global code
    try:
        if temp == None:
            return None
        temp = round(temp)
        tempMessage = ""
        precMessage = ""

        if temp <= tempLow:
            tempMessage = "Cold! Temperature today is: " + str(temp) + "C with a high of " + str(highOf) + "C You should grab a cold weather jacket."
        elif tempLow < temp <= tempMid:
            tempMessage = "Mild Low. Temperature today is: " + str(temp) + "C with a high of " + str(highOf) + "C"
        elif tempMid < temp <= tempHigh:
            tempMessage = "Mild High. Temperature today is: " + str(temp) + "C with a high of " + str(highOf) + "C It's warm enough for a light jacket."
        elif tempHigh < temp:
            tempMessage = "Hot! Temperature today is: " + str(temp) + "C with a high of " + str(highOf) + "C It is too warm for a jacket!"
        if precLow <= prec < precHigh:
            precMessage = " ...Rain likely..."
        elif precHigh <= prec:
            precMessage = " !!!RAIN WARNING!!!"
        code = 0
        return tempMessage + precMessage
    except:
        code = 2
        return None

def fileWrite(response):
    #Creates a record
    global code
    try:
        data = response.json()
        data = json.dumps(data, indent=4)
        fileName = str(today) + "WeatherReport"
        with open(fileName+".json", 'w') as file:
            file.write(data)
        code = 0
    except:
        code = 4

#Error checking
response = getData()
if code == 3:
    print("Fail! Weather data unable to be retrieved")
else:
    fileWrite(response)
    if code == 4:
        print("Failed to save weather data!")
    data = processData(response)
    if code == 1:
        print("Fail! Weather data unable to be processed")
    else:
        message = recommender(data[0], data[1], data[2])
        if code == 2:
            print("Fail! Recommender unable to recommend")
        else:
            print(message)
