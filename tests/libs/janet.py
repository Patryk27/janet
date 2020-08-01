import subprocess
import time

import requests


class Janet:
    addr = "http://127.0.0.1:10000"

    def __init__(self, executable: str, cwd: str):
        self.executable = executable
        self.cwd = cwd

    def __enter__(self):
        self.process = subprocess.Popen(self.executable, cwd=self.cwd)

        # TODO wait for stdout to say "starting server"
        time.sleep(0.5)

        return self

    def __exit__(self, exc_type, exc_val, exc_tb):
        self.kill()

    def wait(self) -> None:
        self.process.wait()

    def kill(self) -> None:
        self.process.terminate()

    def spoof_gitlab_webhook_event(self, event) -> None:
        resp = requests.post(self.addr + "/webhooks/gitlab", json=event)

        if resp.status_code != 204:
            raise Exception("Couldn't spoof GitLab notification - Janet said: HTTP " + str(resp.status_code))
