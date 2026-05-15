CREATE TABLE IF NOT EXISTS schools (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    lrco_id TEXT NOT NULL UNIQUE
);

CREATE TABLE IF NOT EXISTS classes (
    id TEXT PRIMARY KEY,
    school_id TEXT NOT NULL,
    subject TEXT NOT NULL,
    schedule TEXT NOT NULL,
    lrco_id TEXT NOT NULL UNIQUE,
    FOREIGN KEY(school_id) REFERENCES schools(id)
);

CREATE TABLE IF NOT EXISTS assessments (
    id TEXT PRIMARY KEY,
    class_id TEXT NOT NULL,
    date TEXT NOT NULL,
    type TEXT NOT NULL,
    lrco_id TEXT NOT NULL UNIQUE,
    FOREIGN KEY(class_id) REFERENCES classes(id)
);
