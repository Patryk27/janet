BEGIN TRANSACTION;

-- Since this feature wasn't ready for v0.1 anyway, it's perfectly fine to drop
-- the entire table
DROP TABLE reminders;

CREATE TABLE reminders
(
    id                TEXT     NOT NULL PRIMARY KEY,
    user_id           TEXT     NOT NULL,
    merge_request_id  TEXT     NOT NULL,
    ext_discussion_id TEXT     NOT NULL,
    message           TEXT,
    remind_at         DATETIME NOT NULL,
    created_at        DATETIME NOT NULL DEFAULT (datetime('now', 'localtime')),
    FOREIGN KEY (user_id) REFERENCES users (id),
    FOREIGN KEY (merge_request_id) REFERENCES merge_requests (id)
) WITHOUT ROWID;

CREATE INDEX idx_reminders_remind_at ON reminders (remind_at);

COMMIT;
