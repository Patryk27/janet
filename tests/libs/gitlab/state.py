from typing import Dict


class GitLabUser:
    def __init__(self, id: int, username: str) -> None:
        self.id = id
        self.username = username


class GitLabNamespace:
    def __init__(self, id: int, name: str, full_path: str) -> None:
        self.id = id
        self.name = name
        self.full_path = full_path


class GitLabProject:
    def __init__(self, id: int, namespace: GitLabNamespace) -> None:
        self.id = id
        self.namespace = namespace


class GitLabState:
    users: Dict[int, GitLabUser] = {}
    namespaces: Dict[int, GitLabNamespace] = {}
    projects: Dict[int, GitLabProject] = {}

    def add_user(self, id: int, username: str) -> None:
        self.users[id] = GitLabUser(id, username)

    def add_namespace(self, id: int, name: str, full_path: str) -> None:
        self.namespaces[id] = GitLabNamespace(id, name, full_path)

    def add_project(self, id: int, namespace_id: int) -> None:
        if namespace_id not in self.namespaces:
            raise Exception(f"Namespace not found: {namespace_id}")

        namespace = self.namespaces[namespace_id]

        self.projects[id] = GitLabProject(id, namespace)
