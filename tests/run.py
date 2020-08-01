#!/usr/bin/env python3

import os
import sys
from pathlib import Path

from libs.gitlab import *
from libs.janet import *

if len(sys.argv) > 1:
    executable = sys.argv[1]
else:
    executable = "target/debug/janet"

executable = os.path.realpath(executable)
cwd = os.path.dirname(os.path.realpath(__file__))

# --- #

with GitLab() as gitlab, Janet(executable, cwd) as janet:
    if os.path.exists("/tmp/janet.db"):
        os.remove("/tmp/janet.db")

    Path("/tmp/janet.db").touch()

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

    gitlab.state.expect_merge_request_note(project_id=100, merge_request_iid=1, note="@someone yass")

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
