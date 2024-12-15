import datetime
import json
today = datetime.date.today()
dict = {
    "name": "Bruh",
    "roll": "bruh2",
    "fdsa": "gfedas"
}

json_object = json.dumps(dict, indent=4)
fileName = "weatherReport" + str(today)
with open(fileName+".txt", 'w') as file:
    file.write(json_object)