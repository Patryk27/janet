import os
import subprocess
import time
from pathlib import Path

import requests


class Janet:
    url = "http://127.0.0.1:10000"
    database_files = ["/tmp/janet.db", "/tmp/janet.db-shm", "/tmp/janet.db-wal"]

    def __init__(self, executable: str, cwd: str) -> None:
        self.clear_database()

        self.process = subprocess.Popen(
            executable,
            cwd=cwd,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
        )

        # TODO wait for stdout to say "starting server" instead
        time.sleep(0.5)

    def kill(self) -> None:
        self.process.terminate()

    def spoof_gitlab_webhook_event(self, event) -> None:
        resp = requests.post(self.url + "/webhooks/gitlab", json=event)

        if resp.status_code != 204:
            raise Exception("Couldn't spoof GitLab webhook event - Janet said: HTTP " + str(resp.status_code))

    @staticmethod
    def clear_database() -> None:
        for file in Janet.database_files:
            if os.path.exists(file):
                os.remove(file)

        Path("/tmp/janet.db").touch()
