BEGIN TRANSACTION;

CREATE TABLE logs
(
    event      TEXT     NOT NULL,
    payload    TEXT     NOT NULL,
    created_at DATETIME NOT NULL DEFAULT (datetime('now', 'localtime'))
);

CREATE TABLE merge_request_dependencies
(
    id                           TEXT     NOT NULL PRIMARY KEY,
    user_id                      INT      NOT NULL,
    source_project_id            INT      NOT NULL,
    source_merge_request_iid     INT      NOT NULL,
    source_discussion_id         TEXT,
    dependency_project_id        INT      NOT NULL,
    dependency_merge_request_iid INT      NOT NULL,
    checked_at                   DATETIME NOT NULL DEFAULT (datetime('now', 'localtime')),
    created_at                   DATETIME NOT NULL DEFAULT (datetime('now', 'localtime'))
) WITHOUT ROWID;

CREATE UNIQUE INDEX idx_source ON merge_request_dependencies (user_id, source_project_id, source_merge_request_iid,
                                                              source_discussion_id);

CREATE TABLE reminders
(
    id                TEXT     NOT NULL PRIMARY KEY,
    user_id           INT      NOT NULL,
    project_id        INT      NOT NULL,
    merge_request_iid INT      NOT NULL,
    remind_at         DATETIME NOT NULL,
    created_at        DATETIME NOT NULL DEFAULT (datetime('now', 'localtime'))
) WITHOUT ROWID;

COMMIT;
