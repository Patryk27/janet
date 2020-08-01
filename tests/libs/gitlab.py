import json
import re
from functools import partial
from http.server import BaseHTTPRequestHandler, HTTPServer
from threading import Thread
from typing import Dict, List

import requests


class GitLabNamespace:
    def __init__(self, id: int, name: str, full_path: str):
        self.id = id
        self.name = name
        self.full_path = full_path


class GitLabProject:
    def __init__(self, id: int, namespace: GitLabNamespace):
        self.id = id
        self.namespace = namespace


class GitLabMergeRequestNoteExpectation:
    def __init__(self, project_id: int, merge_request_iid: int, note: str):
        self.project_id = project_id
        self.merge_request_iid = merge_request_iid
        self.note = note


class GitLabModelEncoder(json.JSONEncoder):
    def default(self, o):
        return o.__dict__


class GitLabState:
    namespaces: Dict[int, GitLabNamespace] = {}
    projects: Dict[int, GitLabProject] = {}
    merge_request_note_expectations: List[GitLabMergeRequestNoteExpectation] = []

    def add_namespace(self, id: int, name: str, full_path: str) -> None:
        self.namespaces[id] = GitLabNamespace(id, name, full_path)

    def add_project(self, id: int, namespace_id: int) -> None:
        if namespace_id not in self.namespaces:
            raise Exception(f"Namespace not found: {namespace_id}")

        namespace = self.namespaces[namespace_id]

        self.projects[id] = GitLabProject(id, namespace)

    def expect_merge_request_note(self, project_id: int, merge_request_iid: int, note: str):
        self.merge_request_note_expectations.append(
            GitLabMergeRequestNoteExpectation(project_id, merge_request_iid, note))


class GitLabRequestHandler(BaseHTTPRequestHandler):
    def __init__(self, state, *args, **kwargs):
        self.state = state

        super().__init__(*args, **kwargs)

    # noinspection PyPep8Naming
    def do_GET(self):
        if self.path == "/":
            self.send_response(requests.codes.ok)
            self.end_headers()
            self.wfile.write(b"")
            return

        matches = re.match("^/api/v4/projects/(\\d+)$", self.path)

        if matches:
            project_id = int(matches.group(1))

            if project_id in self.state.projects:
                project = self.state.projects[project_id]

                self.send_response(requests.codes.ok)
                self.end_headers()
                self.wfile.write(json.dumps(project, cls=GitLabModelEncoder).encode("utf-8"))
            else:
                self.send_response(requests.codes.not_found)
                self.end_headers()
                self.wfile.write(b"")

            return

    # noinspection PyPep8Naming
    def do_POST(self):
        matches = re.match("^/api/v4/projects/(\\d+)/merge_requests/(\\d+)/notes$", self.path)

        if matches:
            project_id = int(matches.group(1))
            merge_request_iid = int(matches.group(2))

            # TODO
            for note in self.state.merge_request_note_expectations:
                pass

            self.send_response(requests.codes.ok)
            self.end_headers()
            self.wfile.write(b"")
            return


class GitLab:
    def __init__(self):
        self.state = GitLabState()

    def __enter__(self):
        self.server = HTTPServer(
            ("127.0.0.1", 10001),
            partial(GitLabRequestHandler, self.state)
        )

        self.thread = Thread(target=self.server.serve_forever)
        self.thread.setDaemon(True)
        self.thread.start()

        return self

    def __exit__(self, exc_type, exc_val, exc_tb):
        self.kill()

    def kill(self) -> None:
        self.server.shutdown()
