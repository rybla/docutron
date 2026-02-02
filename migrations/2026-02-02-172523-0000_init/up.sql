-- SQLite

-------------------------------------------------------------------------------
-- documents
-------------------------------------------------------------------------------

CREATE TABLE documents (
    -- required
    id INTEGER PRIMARY KEY NOT NULL,
    added_date DATE NOT NULL,
    -- default
    bookmark_count INTEGER NOT NULL DEFAULT 0,
    -- optional
    url TEXT,
    source TEXT,
    title TEXT,
    published_date TEXT,
    summary TEXT,
    fetch_error TEXT,
    summary_error TEXT
);

-------------------------------------------------------------------------------
-- authors
-------------------------------------------------------------------------------

CREATE TABLE authors (
    id INTEGER PRIMARY KEY NOT NULL,
    name TEXT,
    website_url TEXT,
    github_username TEXT,
    x_username TEXT
);

-- relates many document to many authors
CREATE TABLE document_authors (
    document_id INTEGER NOT NULL,
    author_id INTEGER NOT NULL,
    PRIMARY KEY (document_id, author_id),
    FOREIGN KEY (document_id) REFERENCES documents (id) ON DELETE CASCADE,
    FOREIGN KEY (author_id) REFERENCES authors (id) ON DELETE CASCADE
);

-------------------------------------------------------------------------------
--- tags
-------------------------------------------------------------------------------

CREATE TABLE tags (
    id INTEGER PRIMARY KEY NOT NULL,
    label TEXT NOT NULL
);

-- tag groups
CREATE TABLE tag_groups (
    id INTEGER PRIMARY KEY NOT NULL,
    name TEXT NOT NULL
);

-- relates one tag group to many tags
CREATE TABLE tag_group_tags (
    tag_group_id INTEGER NOT NULL,
    tag_id INTEGER NOT NULL,
    PRIMARY KEY (tag_group_id, tag_id),
    FOREIGN KEY (tag_group_id) REFERENCES tag_groups (id) ON DELETE CASCADE,
    FOREIGN KEY (tag_id) REFERENCES tags (id) ON DELETE CASCADE
);

-- relates many document to many tags
CREATE TABLE document_tags (
    document_id INTEGER NOT NULL,
    tag_id INTEGER NOT NULL,
    PRIMARY KEY (document_id, tag_id),
    FOREIGN KEY (document_id) REFERENCES documents (id) ON DELETE CASCADE,
    FOREIGN KEY (tag_id) REFERENCES tags (id) ON DELETE CASCADE
);

-- relates many author to many tags
CREATE TABLE author_tags (
    author_id INTEGER NOT NULL,
    tag_id INTEGER NOT NULL,
    PRIMARY KEY (author_id, tag_id),
    FOREIGN KEY (author_id) REFERENCES authors (id) ON DELETE CASCADE,
    FOREIGN KEY (tag_id) REFERENCES tags (id) ON DELETE CASCADE
);
