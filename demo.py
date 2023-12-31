import subprocess
import time
import yaml

engine = subprocess.Popen(["./target/debug/gtc", "--mode", "protocol"], stdin=subprocess.PIPE,
                     stdout=subprocess.PIPE,
                     stderr=subprocess.STDOUT, universal_newlines=True, bufsize=1)

def put(command):
    engine.stdin.write(command+'\n')
    
def get():
    # using the 'isready' command (engine has to answer 'readyok')
    # to indicate current last line of stdout
    engine.stdin.write('ping\n')
    time.sleep(0)
    res = ""
    while True:
        text = engine.stdout.readline().strip("\n")
        if text.strip() == "ok":
            break
        else:
            if text != "":
                res += "\n"+text
    return res

def cmd(command):
    get()
    put(command)

    return get()

cmd("l")
cmd("b")
cmd("w")
print(cmd("m g 1b"))
print(cmd("m x b2"))
print(yaml.safe_load(cmd("state")))
print(cmd("b"))
