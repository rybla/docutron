-- SQLite

CREATE TABLE docs (
    -- required
    id INTEGER PRIMARY KEY NOT NULL,
    added_date DATE NOT NULL,
    -- optional
    url TEXT,
    source TEXT,
    title TEXT,
    published_date TEXT,
    summary TEXT
);

CREATE TABLE tags (
    id INTEGER PRIMARY KEY NOT NULL,
    label TEXT NOT NULL
);

-- relates many doc to many tags
CREATE TABLE doc_tags (
    doc_id INTEGER NOT NULL,
    tag_id INTEGER NOT NULL,
    PRIMARY KEY (doc_id, tag_id),
    FOREIGN KEY (doc_id) REFERENCES docs (id) ON DELETE CASCADE,
    FOREIGN KEY (tag_id) REFERENCES tags (id) ON DELETE CASCADE
);
