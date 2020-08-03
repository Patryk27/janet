#!/usr/bin/env python3

import sys

from libs.gitlab import *
from libs.janet import *
from libs.test_case import test_case, TestCase

if len(sys.argv) <= 1:
    executable = "target/debug/janet"
elif len(sys.argv) == 2:
    executable = sys.argv[1]
else:
    raise Exception("Invalid numbers of command-line arguments - expected zero or one")

executable = os.path.realpath(executable)
cwd = os.path.dirname(os.path.realpath(__file__))


# --- #


@test_case("Merge request dependencies / Smoke test")
def test(gitlab: GitLab, janet: Janet):
    gitlab.state.add_namespace(id=1, name="test", full_path="test")
    gitlab.state.add_project(id=100, namespace_id=1)
    gitlab.state.add_merge_request(id=250, project_id=100, iid=1, web_url="https://first-merge-request")
    gitlab.state.add_merge_request(id=251, project_id=100, iid=2, web_url="https://second-merge-request")

    for (author_id, author_username) in [(20, "foo"), (21, "bar")]:
        gitlab.state.add_user(id=author_id, username=author_username)

        gitlab.expect.merge_request_note_created(
            project_id=100,
            merge_request_iid=1,
            discussion_id=f"abcd{author_id}",
            note=f"@{author_username} :+1:",
        )

        janet.spoof_gitlab_webhook_event({
            "event_type": "note",
            "project": {
                "id": 100,
                "namespace": "test",
            },
            "merge_request": {
                "id": 250,
                "iid": 1,
            },
            "object_attributes": {
                "author_id": author_id,
                "description": "@janet +depends on !2",
                "discussion_id": f"abcd{author_id}",
            }
        })

    for (author_id, author_username) in [(20, "foo"), (21, "bar")]:
        gitlab.expect.merge_request_note_created(
            project_id=100,
            merge_request_iid=1,
            discussion_id=f"abcd{author_id}",
            note=f"@{author_username} related merge request https://second-merge-request has been closed",
        )

    time.sleep(2)

    janet.spoof_gitlab_webhook_event({
        "event_type": "merge_request",
        "project": {
            "id": 100,
            "namespace": "test",
        },
        "object_attributes": {
            "action": "close",
            "iid": 2
        }
    })

    time.sleep(2)


for case in TestCase.all():
    if not case.run(executable, cwd):
        sys.exit(1)
