from typing import Dict, Tuple


class GitLabNamespace:
    def __init__(self, id: int, name: str, full_path: str) -> None:
        self.id = id
        self.name = name
        self.full_path = full_path


class GitLabProject:
    def __init__(self, id: int, namespace: GitLabNamespace) -> None:
        self.id = id
        self.namespace = namespace


class GitLabMergeRequest:
    def __init__(self, project_id: int, iid: int, web_url: str) -> None:
        self.project_id = project_id
        self.iid = iid
        self.web_url = web_url


class GitLabUser:
    def __init__(self, id: int, username: str) -> None:
        self.id = id
        self.username = username


class GitLabState:
    namespaces: Dict[int, GitLabNamespace] = {}
    projects: Dict[int, GitLabProject] = {}
    merge_requests: Dict[Tuple[int, int], GitLabMergeRequest] = {}
    users: Dict[int, GitLabUser] = {}

    def add_namespace(self, id: int, name: str, full_path: str) -> None:
        self.namespaces[id] = GitLabNamespace(id, name, full_path)

    def add_project(self, id: int, namespace_id: int) -> None:
        if namespace_id not in self.namespaces:
            raise Exception(f"Namespace not found: {namespace_id}")

        namespace = self.namespaces[namespace_id]

        self.projects[id] = GitLabProject(id, namespace)

    def add_merge_request(self, project_id: int, iid: int, web_url: str) -> None:
        if project_id not in self.projects:
            raise Exception(f"Project not found: {project_id}")

        self.merge_requests[(project_id, iid)] = GitLabMergeRequest(project_id, iid, web_url)

    def add_user(self, id: int, username: str) -> None:
        self.users[id] = GitLabUser(id, username)
