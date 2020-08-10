use crate::*;

pub fn namespace_1() -> Namespace {
    Namespace {
        id: NamespaceId::new(1),
        name: NamespaceName::new("alpha"),
        full_path: "alpha".to_string(),
    }
}

pub fn project_10() -> Project {
    Project {
        id: ProjectId::new(10),
        namespace: namespace_1(),
    }
}

pub fn merge_request_100() -> MergeRequest {
    MergeRequest {
        id: MergeRequestId::new(100),
        project_id: ProjectId::new(10),
        iid: MergeRequestIid::new(1),
        web_url: "http://gitlab.com/merge_requests/100".to_string(),
        state: "opened".to_string(),
    }
}

pub fn merge_request_101() -> MergeRequest {
    MergeRequest {
        id: MergeRequestId::new(101),
        project_id: ProjectId::new(10),
        iid: MergeRequestIid::new(2),
        web_url: "http://gitlab.com/merge_requests/101".to_string(),
        state: "opened".to_string(),
    }
}

pub fn user_250() -> User {
    User {
        id: UserId::new(250),
        username: "someone".to_string(),
    }
}
