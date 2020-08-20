BEGIN TRANSACTION;

ALTER TABLE merge_requests RENAME iid TO ext_iid;
ALTER TABLE merge_requests RENAME state TO ext_state;

ALTER TABLE merge_request_dependencies RENAME discussion_ext_id TO ext_discussion_id;

COMMIT;
