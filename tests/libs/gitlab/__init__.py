from functools import partial
from http.server import HTTPServer
from threading import Thread

from libs.gitlab.expectations import *
from libs.gitlab.request_handler import *
from libs.gitlab.state import *


class GitLab:
    state = GitLabState()
    expect = GitLabExpectations()

    def __init__(self):
        self.server = HTTPServer(
            ("127.0.0.1", 10001),
            partial(GitLabRequestHandler, self.state, self.expect)
        )

        self.thread = Thread(target=self.server.serve_forever)
        self.thread.setDaemon(True)
        self.thread.start()

    def kill(self) -> None:
        self.server.shutdown()
