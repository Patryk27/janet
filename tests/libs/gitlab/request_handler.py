import json
import re
from http.server import BaseHTTPRequestHandler

import requests

from libs.gitlab.expectations import GitLabExpectations
from libs.gitlab.state import GitLabState


class GitLabModelEncoder(json.JSONEncoder):
    def default(self, o):
        return o.__dict__


class GitLabRequestHandler(BaseHTTPRequestHandler):
    def __init__(self, state: GitLabState, expect: GitLabExpectations, *args, **kwargs) -> None:
        self.state = state
        self.expect = expect

        self.routes = {
            "GET": {
                "^/$":
                    handle_ping,

                "^/api/v4/projects/(\\d+)$":
                    handle_get_project,

                "^/api/v4/projects/(\\d+)/merge_requests/(\\d+)$":
                    handle_get_merge_request,

                "^/api/v4/users/(\\d+)$":
                    handle_get_user,
            },

            "POST": {
                "^/api/v4/projects/(\\d+)/merge_requests/(\\d+)/discussions/([a-zA-Z0-9]+)/notes$":
                    handle_create_merge_request_note,
            }
        }

        super().__init__(*args, **kwargs)

    def read_body(self) -> bytes:
        body_len = int(self.headers.get('content-length'))
        return self.rfile.read(body_len)

    def read_body_json(self):
        return json.loads(self.read_body())

    def handle_route(self, routes) -> None:
        for (route_regex, route_handler) in routes.items():
            matches = re.match(route_regex, self.path)

            if matches:
                route_handler(self, matches)
                return

        self.send_response(requests.codes.not_implemented)
        self.end_headers()
        self.wfile.write(b"")

    # noinspection PyPep8Naming
    def do_GET(self) -> None:
        self.handle_route(self.routes['GET'])

    # noinspection PyPep8Naming
    def do_POST(self) -> None:
        self.handle_route(self.routes['POST'])


# noinspection PyUnusedLocal
def handle_ping(self: GitLabRequestHandler, matches) -> None:
    self.send_response(requests.codes.ok)
    self.end_headers()
    self.wfile.write(b"")


def handle_get_project(self: GitLabRequestHandler, matches) -> None:
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


def handle_get_merge_request(self: GitLabRequestHandler, matches) -> None:
    project_id = int(matches.group(1))
    merge_request_iid = int(matches.group(2))

    if (project_id, merge_request_iid) in self.state.merge_requests:
        merge_request = self.state.merge_requests[(project_id, merge_request_iid)]

        self.send_response(requests.codes.ok)
        self.end_headers()
        self.wfile.write(json.dumps(merge_request, cls=GitLabModelEncoder).encode("utf-8"))
    else:
        self.send_response(requests.codes.not_found)
        self.end_headers()
        self.wfile.write(b"")


def handle_get_user(self: GitLabRequestHandler, matches) -> None:
    user_id = int(matches.group(1))

    if user_id in self.state.users:
        user = self.state.users[user_id]

        self.send_response(requests.codes.ok)
        self.end_headers()
        self.wfile.write(json.dumps(user, cls=GitLabModelEncoder).encode("utf-8"))
    else:
        self.send_response(requests.codes.not_found)
        self.end_headers()
        self.wfile.write(b"")


def handle_create_merge_request_note(self: GitLabRequestHandler, matches) -> None:
    body = self.read_body_json()

    project_id = int(matches.group(1))
    merge_request_iid = int(matches.group(2))
    discussion_id = str(matches.group(3))
    note = body['body']

    matches = [x for x in self.expect.merge_request_notes if
               x.matches(project_id, merge_request_iid, discussion_id, note)]

    if not len(matches):
        raise Exception(
            f"Janet created an unexpected merge request note: " +
            f"project_id={project_id}, " +
            f"merge_request_iid={merge_request_iid}, " +
            f"discussion_id={discussion_id}, " +
            f"note={note}"
        )

    for match in matches:
        match.handle()

    self.send_response(requests.codes.ok)
    self.end_headers()
    self.wfile.write(b"")
