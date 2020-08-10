BEGIN TRANSACTION;

CREATE TABLE logs
(
    event      TEXT     NOT NULL,
    payload    TEXT     NOT NULL,
    created_at DATETIME NOT NULL DEFAULT (datetime('now', 'localtime'))
);

--

CREATE TABLE users
(
    id         TEXT     NOT NULL PRIMARY KEY,
    ext_id     INT      NOT NULL,
    created_at DATETIME NOT NULL DEFAULT (datetime('now', 'localtime'))
) WITHOUT ROWID;

CREATE UNIQUE INDEX idx_users_ext_id ON users (ext_id);

--

CREATE TABLE projects
(
    id         TEXT     NOT NULL PRIMARY KEY,
    ext_id     INT      NOT NULL,
    created_at DATETIME NOT NULL DEFAULT (datetime('now', 'localtime'))
) WITHOUT ROWID;

CREATE UNIQUE INDEX idx_projects_ext_id ON projects (ext_id);

--

CREATE TABLE merge_requests
(
    id         TEXT     NOT NULL PRIMARY KEY,
    project_id TEXT     NOT NULL,
    ext_id     INT      NOT NULL,
    iid        INT      NOT NULL,
    state      TEXT     NOT NULL,
    checked_at DATETIME NOT NULL DEFAULT (datetime('now', 'localtime')),
    created_at DATETIME NOT NULL DEFAULT (datetime('now', 'localtime')),
    FOREIGN KEY (project_id) REFERENCES projects (id)
) WITHOUT ROWID;

CREATE UNIQUE INDEX idx_merge_requests_ext_id ON merge_requests (ext_id);
CREATE UNIQUE INDEX idx_merge_requests_iid ON merge_requests (project_id, iid);
CREATE INDEX idx_merge_requests_checked_at ON merge_requests (checked_at);

--

CREATE TABLE merge_request_dependencies
(
    id                   TEXT     NOT NULL PRIMARY KEY,
    user_id              TEXT     NOT NULL,
    discussion_ext_id    TEXT     NOT NULL,
    src_merge_request_id TEXT     NOT NULL,
    dst_merge_request_id TEXT     NOT NULL,
    created_at           DATETIME NOT NULL DEFAULT (datetime('now', 'localtime')),
    FOREIGN KEY (user_id) REFERENCES users (id),
    FOREIGN KEY (src_merge_request_id) REFERENCES merge_requests (id),
    FOREIGN KEY (dst_merge_request_id) REFERENCES merge_requests (id)
) WITHOUT ROWID;

CREATE INDEX idx_merge_request_dependencies_src ON merge_request_dependencies (discussion_ext_id, src_merge_request_id);
CREATE INDEX idx_merge_request_dependencies_dst ON merge_request_dependencies (dst_merge_request_id);

--

CREATE TABLE reminders
(
    id               TEXT     NOT NULL PRIMARY KEY,
    user_id          TEXT     NOT NULL,
    merge_request_id TEXT     NOT NULL,
    remind_at        DATETIME NOT NULL,
    created_at       DATETIME NOT NULL DEFAULT (datetime('now', 'localtime')),
    FOREIGN KEY (user_id) REFERENCES users (id),
    FOREIGN KEY (merge_request_id) REFERENCES merge_requests (id)
) WITHOUT ROWID;

CREATE INDEX idx_reminders_remind_at ON reminders (remind_at);

COMMIT;
