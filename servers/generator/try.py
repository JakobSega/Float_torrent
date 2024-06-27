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

print(db.keys())
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
--->'Constant_Imposter': {'name': 'Constant', 'description': 'Constant sequence', 'parameters': 1, 'sequences': 1}, 
    'Lin Comb_Elves': {'name': 'Lin Comb', 'description': '', 'parameters': 3, 'sequences': 2}}}},

'Binarni Banditi': {'Info': {'name': 'Binarni Banditi', 'ip': '127.0.0.1', 'port': 12345}, 
'Sequences': {
    'Arithmetic': {'name': 'Arithmetic', 'description': 'Arithmetic sequence', 'parameters': 2, 'sequences': 0}, 
    'Constant': {'name': 'Constant', 'description': 'Constant sequence', 'parameters': 1, 'sequences': 0}, 
    'Lin Comb': {'name': 'Lin Comb', 'description': '', 'parameters': 3, 'sequences': 2}}}}
"""
#---> opozarja na primer, ko je isto zaporedje na dveh različnih serverjih. Dodatno - eno ima pravilno signaturo, drugo pa (tisto na Elves ima napačno signaturo...) 

ZAP = "LinearCombination_Elves"
SERVER = "Binarni Banditi"

#Za definicijo ranga
start = 0
end = 2
step = 1

shift = 5

#####################

def const_body(start, end, step, c):
    body = {
            "range": {
                "from": start,
                "to": end,
                "step": step,
            },
            "parameters": [c],
            "sequences": [
            ],
        }
    return body

def em_body(start, end, step):
    body = {
            "range": {
                "from": start,
                "to": end,
                "step": step,
            },
            "parameters": [],
            "sequences": [
            ],
        }
    return body

def hof_body(start, end, step):
    body = {
            "range": {
                "from": start,
                "to": end,
                "step": step,
            },
            "parameters": [],
            "sequences": [
            ],
        }
    return body

def rec_body(start, end, step):
    body = {
            "range": {
                "from": start,
                "to": end,
                "step": step,
            },
            "parameters": [],
            "sequences": [
            ],
        }
    return body

def arit_body(start, end, step, a_0, d):
    body = {
            "range": {
                "from": start,
                "to": end,
                "step": step,
            },
            "parameters": [a_0, d],
            "sequences": [
            ],
        }
    return body
    
def geo_body(start, end, step, a, q):
    body = {
            "range": {
                "from": start,
                "to": end,
                "step": step,
            },
            "parameters": [a, q],
            "sequences": [
            ],
        }
    return body

def nth_root_body(start, end, step, seq):
    body = {
        "range": {
                "from": start,
                "to": end,
                "step": step,
            },
        "parameters": [],
        "sequences": [seq
        ],
        
    }
    return body

def drop_body(start, end, step, shift, seq):
    body = {
        "range": {
                "from": start,
                "to": end,
                "step": step,
            },
        "parameters": [shift],
        "sequences": [seq
        ],
        
    }
    return body

def fib_body(start, end, step, a, b):
    body = {
            "range": {
                "from": start,
                "to": end,
                "step": step,
            },
            "parameters": [a, b],
            "sequences": [
            ],
        }
    return body
##############################

def lin_body(start, end, step, l_0, l_1, l_2, seq1, seq2):
    lin_body = {
        "range": {
                "from": start,
                "to": end,
                "step": step,
            },
        "parameters": [l_0, l_1, l_2],
        "sequences": [seq1, seq2
        ],
        
    }
    return lin_body

def prod_body(start, end, step, seq1, seq2):
    prod_body = {
        "range": {
                "from": start,
                "to": end,
                "step": step,
            },
        "parameters": [],
        "sequences": [seq1, seq2
        ],
    }
    return prod_body

def sum_body(start, end, step, seq1, seq2):
    sum_body = {
        "range": {
                "from": start,
                "to": end,
                "step": step,
            },
        "parameters": [],
        "sequences": [seq1, seq2
        ],
        
    }
    return sum_body

##############################

def gen_seq(name, parameters, sequences):
    body = {
            "name" : name,
            "parameters": parameters,
            "sequences": sequences,
        }
    return body

##############################


gen_const = gen_seq(name = "Constant_Imposter", parameters = [4], sequences = [])
# 4, 4, 4, 4, 4, 4, 4
gen_arit = gen_seq(name = "Arithmetic", parameters = [10, 3], sequences = [])
# 10, 13, 16, 19, 22, 25
gen_hof = gen_seq(name = "Hofstadter_Elves", parameters = [], sequences = [])
gen_lin = gen_seq(name = "LinearCombination_Imposter", parameters = [2, -1, 5], sequences = [gen_const, gen_arit])
# 48, 63, 78, 92

lin = lin_body(start, end, step, 2, 3, 20, gen_hof, gen_lin)
# 974, 1274, 1574

body = lin_body(start=start, end=end, step=step, l_0 = 43, l_1 = 11, l_2 = 23,
                seq1 = gen_seq(name = "LinearCombination_Elves", parameters = [44, -10, -1], sequences = [
                    gen_seq(name = "Geometric_Imposter", parameters = [3, 2], sequences = []),
                    gen_seq(name="Arithmetic_Imposter", parameters = [-2, 10], sequences = []),
                    ]),
                seq2 = gen_seq(name = "Sum_Elves", parameters = [], sequences = [
                    gen_seq(name = "Drop", parameters = [11], sequences = [
                        gen_seq(name="Hofstadter", parameters = [], sequences = [])]),
                    gen_seq(name = "Constant", parameters = [3.0], sequences = [])]))

def send(ZAP, SERVER, body):
    #Želimo poklicati SERVER in dobiti ZAP...
    project = db[SERVER]["Info"]
    url = ("http://" + project["ip"] + ":" + str(project["port"]) + "/sequence/") + ZAP
 
      
    r = requests.post(url, json=body)
    return r

res = send(ZAP, SERVER, body)
print("This is proof of concept...", "It is a complex sequence from multiple servers...")
print(res.text)

def test1():
    start = 0 
    end = 4
    step = 1
    body = const_body(start, end, step, 0.0)
    zap = "Constant"
    server = "Binarni Banditi"
    res = send(zap, server, body)
    res = res.json()
    res = [float(x) for x in res]
    val = [0.0,0.0,0.0,0.0,0.0]
    if res == val:
        print("Passes test1.")
    else:
        print("Failed test1")
        print("Expected :", val)
        print("Received :", res)
    return 

def test2():
    start = 0 
    end = 4
    step = 1
    body = arit_body(start, end, step, 10.0, 2.0)
    zap = "Arithmetic"
    server = "Binarni Banditi"
    res = send(zap, server, body)
    res = res.json()
    res = [float(x) for x in res]
    val = [10, 12, 14, 16, 18]
    if res == val:
        print("Passes test2.")
    else:
        print("Failed test2")
        print("Expected :", val)
        print("Received :", res)
    return 

def test3():
    start = 0 
    end = 4
    step = 1
    body = geo_body(start, end, step, 1.0, 2.0)
    zap = "Geometric"
    server = "Binarni Banditi"
    res = send(zap, server, body)
    res = res.json()
    res = [float(x) for x in res]
    val = [1, 2, 4, 8, 16]
    if res == val:
        print("Passes test3.")
    else:
        print("Failed test3")
        print("Expected :", val)
        print("Received :", res)
    return 

def test4():
    start = 0 
    end = 4
    step = 1
    body = drop_body(start, end, step, shift = 2, seq =
        gen_seq(name = "Geometric", parameters = [1.0, 2.0], sequences = [])
    )
    zap = "Drop"
    server = "Binarni Banditi"
    res = send(zap, server, body)
    res = res.json()
    res = [float(x) for x in res]
    val = [4.0, 8, 16, 32, 64]
    if res == val:
        print("Passes test4.")
    else:
        print("Failed test4")
        print("Expected :", val)
        print("Received :", res)
    return 

def test5():
    start = 0 
    end = 4
    step = 1
    body = lin_body(start, end, step, -1, 2, 10,
        seq1 = gen_seq(name = "Constant", parameters = [2.0], sequences = []),
        seq2 = gen_seq(name = "Arithmetic", parameters = [3.0, 1.0], sequences = []))
    zap = "LinearCombination"
    server = "Binarni Banditi"
    res = send(zap, server, body)
    res = res.json()
    res = [float(x) for x in res]
    val = [33, 43, 53, 63, 73]
    if res == val:
        print("Passes test5.")
    else:
        print("Failed test5")
        print("Expected :", val)
        print("Received :", res)
    return 
def test ():
    test1()
    test2()
    test3() 
    test4()
    test5()
test()