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


@test_case("Smoke tests / Merge request dependencies")
def test(gitlab: GitLab, janet: Janet):
    gitlab.state.add_user(id=20, username="foo")
    gitlab.state.add_user(id=21, username="bar")
    gitlab.state.add_user(id=25, username="zar")
    gitlab.state.add_namespace(id=1, name="test", full_path="test")
    gitlab.state.add_project(id=100, namespace_id=1)

    for author_id in [20, 21, 25]:
        janet.spoof_gitlab_webhook_event({
            "event_type": "note",
            "project": {
                "id": 100,
                "namespace": "test",
            },
            "merge_request": {
                "iid": 1,
            },
            "object_attributes": {
                "author_id": author_id,
                "description": "@janet +depends on !2"
            }
        })

    gitlab.expect.merge_request_note_created(project_id=100, merge_request_iid=1, note="@foo yass!")
    gitlab.expect.merge_request_note_created(project_id=100, merge_request_iid=1, note="@bar yass!")
    gitlab.expect.merge_request_note_created(project_id=100, merge_request_iid=1, note="@zar yass!")

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

    time.sleep(1)


if not TestCase.all()[0].run(executable, cwd):
    sys.exit(1)
