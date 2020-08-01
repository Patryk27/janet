from typing import List


class GitLabExpectation:
    handled = False

    def handle(self) -> None:
        self.handled = True

    def ensure_handled(self) -> None:
        if not self.handled:
            raise Exception(str(self))


class GitLabMergeRequestNoteExpectation(GitLabExpectation):
    def __init__(self, project_id: int, merge_request_iid: int, note: str) -> None:
        self.project_id = project_id
        self.merge_request_iid = merge_request_iid
        self.note = note

    def __str__(self) -> str:
        return f"Expected merge request note to be created: " + \
               f"project_id={self.project_id}, " + \
               f"merge_request_iid={self.merge_request_iid}, " + \
               f"note={self.note}"

    def matches(self, project_id: int, merge_request_iid: int, note: str) -> bool:
        return self.project_id == project_id and \
               self.merge_request_iid == merge_request_iid and \
               self.note == note


class GitLabExpectations:
    merge_request_notes: List[GitLabMergeRequestNoteExpectation] = []

    def ensure_handled(self) -> None:
        for merge_request_note in self.merge_request_notes:
            merge_request_note.ensure_handled()

    def merge_request_note_created(self, project_id: int, merge_request_iid: int, note: str) -> None:
        self.merge_request_notes.append(GitLabMergeRequestNoteExpectation(project_id, merge_request_iid, note))
