import requests
ZAP = "arithmetic"
projects = requests.get("http://127.0.0.1:7878/project").json()
#projects = requests.get("http://0.0.0.0:7878/project").json()
print(projects)
##
for j in projects:
    if j["name"] == "Matija & Filip":
        url = "http://" + j["ip"] + ":" + str(j["port"]) + "/sequence"
        print(url)
        seqs = requests.get(url).json()
        assert "Arithmetic" in [j["name"] for j in seqs]
        k = 10
        z = 0
        if ZAP.lower() == "arithmetic":
            j = 0
            body = {
                "range": {
                    "from": j * 100,
                    "to": (j + 1) * 100,
                    "step": 1,
                },
                "parameters": [z, k],
                "sequences": [
                ],
            }
            r = requests.post(url + "/Arithmetic", json=body)
            # print(r)
            print(r.json())
        if ZAP.lower() == "constant":
            j = 0
            body = {
                "range": {
                    "from": j * 100,
                    "to": (j + 1) * 100,
                    "step": 1,
                },
                "parameters": [z],
                "sequences": [
                ],
            }
            r = requests.post(url + "/Constant", json=body)
            # print(r)
            print(r.json())
        
        break


for j in projects:
    if j["name"] == "Matija & Filip":
        url = "http://" + j["ip"] + ":" + str(j["port"]) + "/sequence"
        print(url)
        seqs = requests.get(url).json()
        assert "Constant" in [j["name"] for j in seqs]
        k = 10
        z = 0
        if True:
            j = 0
            body = {
                "range": {
                    "from": j * 100,
                    "to": (j + 1) * 100,
                    "step": 1,
                },
                "parameters": [z],
                "sequences": [
                ],
            }
            r = requests.post(url + "/Constant", json=body)
            # print(r)
            print(r.json())
        break
else:
    print("Matija & Filip not found")
    exit(1)
    exit(1)
