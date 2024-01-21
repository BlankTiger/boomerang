import os
import time
import requests

url = "https://github.com/BlankTiger/boomerang/releases/download/master/boomerang-server-linux-aarch64"
filename = "server"
TIMEOUT_SECS = 3 * 60
# download binary and if it's not the same as the current one, replace it
while True:
    print("Downloading new server binary...")
    urllib.request.urlretrieve(url, f"{filename}-new")
    os.chmod(f"{filename}-new", 0o755)
    if os.path.exists(filename):
        # compare the two files md5 hashes
        if os.system(f"md5sum {filename} {filename}-new") == 0:
            print("Server is up to date.")
            os.remove(f"{filename}-new")
            break
        else:
            print("Server is out of date. Replacing...")
            os.remove(filename)
            os.rename(f"{filename}-new", filename)
            break
    else:
        print("Server not found. Replacing...")
        os.rename(f"{filename}-new", filename)

    time.sleep(TIMEOUT_SECS)
