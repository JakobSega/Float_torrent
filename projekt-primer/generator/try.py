import requests
projects = requests.get("http://127.0.0.1:7878/project").json()
db = {}


for project in projects:
    name = project["name"]
    db[name] = {"Info" : project, "Sequences" : {}}
    url = "http://" + project["ip"] + ":" + str(project["port"]) + "/sequence"
    
    seqs = requests.get(url).json()
    for seq in seqs:
        seq_name = seq["name"]
        db[name]["Sequences"][seq_name] = seq

print(db)
"""
db izgleda nekoliko takole : 

{'Binarni Banditi & AmongUS': {'Info': {'name': 'Binarni Banditi & AmongUS', 'ip': '127.0.0.1', 'port': 12346}, 
'Sequences': {
    'Arithmetic_Imposter': {'name': 'Arithmetic_Imposter', 'description': 'Arithmetic sequence', 'parameters': 2, 'sequences': 0},
    'Constant_Imposter': {'name': 'Constant_Imposter', 'description': 'Constant sequence', 'parameters': 1, 'sequences': 0}, 
    'Lin Comb_Imposter': {'name': 'Lin Comb_Imposter', 'description': '', 'parameters': 3, 'sequences': 2}}}, 

'Binarni Banditi & Elves': {'Info': {'name': 'Binarni Banditi & Elves', 'ip': '127.0.0.1', 'port': 12347}, 
'Sequences': {
    'Arithmetic_Elves': {'name': 'Arithmetic', 'description': 'Arithmetic sequence', 'parameters': 2, 'sequences': 0}, 
--->'Constant_Imposter': {'name': 'Constant', 'description': 'Constant sequence', 'parameters': 1, 'sequences': 0}, 
    'Lin Comb_Elves': {'name': 'Lin Comb', 'description': '', 'parameters': 3, 'sequences': 2}}}},

'Binarni Banditi': {'Info': {'name': 'Binarni Banditi', 'ip': '127.0.0.1', 'port': 12345}, 
'Sequences': {
    'Arithmetic': {'name': 'Arithmetic', 'description': 'Arithmetic sequence', 'parameters': 2, 'sequences': 0}, 
    'Constant': {'name': 'Constant', 'description': 'Constant sequence', 'parameters': 1, 'sequences': 0}, 
    'Lin Comb': {'name': 'Lin Comb', 'description': '', 'parameters': 3, 'sequences': 2}}}}
"""
#---> opozarja na primer, ko je isto zaporedje na dveh različnih serverjih. 

ZAP = "Constant_Imposter"
SERVER = "Binarni Banditi"

#c_0 je vrednost konstanega zap, a_0 in d sta parametra za aritmetično
c_0 = 0
a_0 = 2.0
d = 3.0

#Za definicijo ranga
start = 0 
end = 3
step = 1

#Želimo poklicati SERVER in dobiti ZAP...
project = db[SERVER]["Info"]
url = ("http://" + project["ip"] + ":" + str(project["port"]) + "/sequence/") + ZAP


parameters = None
if "Constant" in ZAP:
    parameters = [c_0]
elif "Arithmetic" in ZAP:
    parameters = [a_0, d]


body = {
        "range": {
            "from": start,
            "to": end,
            "step": step,
        },
        "parameters": parameters,
        "sequences": [
        ],
    }
    
r = requests.post(url, json=body)
print(r.json())