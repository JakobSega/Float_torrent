import requests

# Fetch projects and initialize the database
projects = requests.get("http://127.0.0.1:7878/project").json()
db = {}

for project in projects:
    name = project["name"]
    db[name] = {"Info": project, "Sequences": {}}
    url = f"http://{project['ip']}:{project['port']}/sequence"
    
    seqs = requests.get(url).json()
    for seq in seqs:
        seq_name = seq["name"]
        db[name]["Sequences"][seq_name] = seq

print("Available servers:", db.keys())

# Define request bodies
def const_body(start, end, step, c):
    return {
        "range": {"from": start, "to": end, "step": step},
        "parameters": [float(c)],  # Ensure parameter is float
        "sequences": []
    }

def arit_body(start, end, step, a_0, d):
    return {
        "range": {"from": start, "to": end, "step": step},
        "parameters": [float(a_0), float(d)],  # Ensure parameters are float
        "sequences": []
    }

def geo_body(start, end, step, a, q):
    return {
        "range": {"from": start, "to": end, "step": step},
        "parameters": [float(a), float(q)],  # Ensure parameters are float
        "sequences": []
    }

def drop_body(start, end, step, shift, seq):
    return {
        "range": {"from": start, "to": end, "step": step},
        "parameters": [float(shift)],  # Ensure parameter is float
        "sequences": [seq]
    }

def lin_body(start, end, step, l_0, l_1, l_2, seq1, seq2):
    return {
        "range": {"from": start, "to": end, "step": step},
        "parameters": [float(l_0), float(l_1), float(l_2)],  # Ensure parameters are float
        "sequences": [seq1, seq2]
    }

def gen_seq(name, parameters, sequences):
    return {
        "name": name,
        "parameters": [float(p) if isinstance(p, (int, float)) else p for p in parameters],
        "sequences": sequences
    }

def story_body(start, end, step, author, genre, seq):
    return {
        "range": {"from": start, "to": end, "step": step},
        "parameters": [author, genre],  # Author and genre should be strings
        "sequences": [seq]
    }

def ai_body(start, end, step, prediction_range, seq):
    return {
        "range": {"from": start, "to": end, "step": step},
        "parameters": [float(prediction_range)],  # Ensure parameter is float
        "sequences": [seq]
    }

# Example sequences
gen_const = gen_seq(name="Constant_Imposter", parameters=[4], sequences=[])
gen_arit = gen_seq(name="Arithmetic", parameters=[10, 3], sequences=[])
gen_hof = gen_seq(name="Hofstadter_Elves", parameters=[], sequences=[])
gen_lin = gen_seq(name="LinearCombination_Imposter", parameters=[2, -1, 5], sequences=[gen_const, gen_arit])

# Define the body for different sequence requests
lin = lin_body(start=0, end=4, step=1, l_0=2, l_1=3, l_2=20, seq1=gen_hof, seq2=gen_lin)

def send(ZAP, SERVER, body):
    project = db[SERVER]["Info"]
    url = f"http://{project['ip']}:{project['port']}/sequence/{ZAP}"
    try:
        r = requests.post(url, json=body)
        r.raise_for_status()  # Raises an HTTPError if the HTTP request returned an unsuccessful status code
    except requests.RequestException as e:
        print(f"Request failed: {e}")
        return None
    return r

# Test functions
def test1():
    body = const_body(start=0, end=4, step=1, c=0.0)
    zap = "Constant"
    server = "Binarni Banditi"
    res = send(zap, server, body)
    if res:
        res = res.json()
        val = [0.0, 0.0, 0.0, 0.0, 0.0]
        if res == val:
            print("Passes test1.")
        else:
            print("Failed test1.")
            print("Expected :", val)
            print("Received :", res)

def test2():
    body = arit_body(start=0, end=4, step=1, a_0=10.0, d=2.0)
    zap = "Arithmetic"
    server = "Binarni Banditi"
    res = send(zap, server, body)
    if res:
        res = res.json()
        val = [10, 12, 14, 16, 18]
        if res == val:
            print("Passes test2.")
        else:
            print("Failed test2.")
            print("Expected :", val)
            print("Received :", res)

def test3():
    body = geo_body(start=0, end=4, step=1, a=1.0, q=2.0)
    zap = "Geometric"
    server = "Binarni Banditi"
    res = send(zap, server, body)
    if res:
        res = res.json()
        val = [1, 2, 4, 8, 16]
        if res == val:
            print("Passes test3.")
        else:
            print("Failed test3.")
            print("Expected :", val)
            print("Received :", res)

def test4():
    body = drop_body(start=0, end=4, step=1, shift=2, seq=gen_seq(name="Geometric", parameters=[1.0, 2.0], sequences=[]))
    zap = "Drop"
    server = "Binarni Banditi"
    res = send(zap, server, body)
    if res:
        res = res.json()
        val = [4.0, 8, 16, 32, 64]
        if res == val:
            print("Passes test4.")
        else:
            print("Failed test4.")
            print("Expected :", val)
            print("Received :", res)

def test5():
    body = lin_body(start=0, end=4, step=1, l_0=-1, l_1=2, l_2=10,
                    seq1=gen_seq(name="Constant", parameters=[2.0], sequences=[]),
                    seq2=gen_seq(name="Arithmetic", parameters=[3.0, 1.0], sequences=[]))
    zap = "LinearCombination"
    server = "Binarni Banditi"
    res = send(zap, server, body)
    if res:
        res = res.json()
        val = [33, 43, 53, 63, 73]
        if res == val:
            print("Passes test5.")
        else:
            print("Failed test5.")
            print("Expected :", val)
            print("Received :", res)

def test6():
    body = story_body(start=0, end=4, step=1, author="6", genre="3",
                      seq=gen_seq(name="Constant_Imposter", parameters=[4], sequences=[]))
    zap = "Story"
    server = "Binarni Banditi"
    res = send(zap, server, body)
    if res:
        res = res.json()
        if isinstance(res, list) and all(isinstance(x, (int, float)) or isinstance(x, str) for x in res):
            print("Passes test6.")
        else:
            print("Failed test6.")
            print("Response format or type is incorrect.")
            print("Received :", res)

def test7():
    body = ai_body(start=0, end=4, step=1, prediction_range=16,
                   seq=gen_seq(name="LinearCombination_Imposter", parameters=[2, -3, 10], sequences=[gen_seq(name="Geometric", parameters=[1.0, 4.0], sequences=[]), gen_seq(name="Arithmetic", parameters=[3.0, 5.0], sequences=[])]))
    zap = "Ai"
    server = "Binarni Banditi"
    res = send(zap, server, body)
    if res:
        res = res.json()
        if isinstance(res, list) and all(isinstance(x, (int, float)) for x in res):
            print("Passes test7.")
            print(res)
        else:
            print("Failed test7.")
            print("Response format or type is incorrect.")
            print("Received :", res)

def test8():
    body = ai_body(start=0, end=4, step=1, prediction_range=18,
                   seq=gen_seq(name="Geometric", parameters=[1.0, 2.0], sequences=[]))
    zap = "Ai"
    server = "Binarni Banditi"
    res = send(zap, server, body)
    if res:
        res = res.json()
        if isinstance(res, list) and all(isinstance(x, (int, float)) for x in res):
            print("Passes test8.")
            print(res)
        else:
            print("Failed test8.")
            print("Received :", res)

# Run tests
if __name__ == "__main__":
    test1()
    test2()
    test3()
    test4()
    test5()
    test6()
    test7()
    test8()
