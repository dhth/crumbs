CREATE TABLE sessions (
    id INTEGER PRIMARY KEY NOT NULL,
    agent_name TEXT NOT NULL,
    title TEXT NOT NULL,
    project_name TEXT NOT NULL,
    path TEXT NOT NULL,
    branch TEXT,
    state TEXT NOT NULL,
    archived_at INTEGER,
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL
);

CREATE TABLE crumbs (
    id INTEGER PRIMARY KEY NOT NULL,
    session_id INTEGER NOT NULL,
    message TEXT NOT NULL,
    state TEXT,
    confidence INTEGER CHECK (confidence >= 0 AND confidence <= 100),
    created_at INTEGER NOT NULL,
    FOREIGN KEY (session_id) REFERENCES sessions(id) ON DELETE CASCADE
);
