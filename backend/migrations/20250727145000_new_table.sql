-- Темы урока (отдельно, как список галочек)
CREATE TABLE lesson_topics (
    id SERIAL PRIMARY KEY,
    lesson_id INTEGER NOT NULL REFERENCES lessons(id) ON DELETE CASCADE,
    topic TEXT NOT NULL
);

-- Теория урока (одна на урок)
CREATE TABLE lesson_theory (
    id SERIAL PRIMARY KEY,
    lesson_id INTEGER NOT NULL UNIQUE REFERENCES lessons(id) ON DELETE CASCADE,
    content TEXT NOT NULL
);

-- Домашнее задание
CREATE TABLE lesson_homework (
    id SERIAL PRIMARY KEY,
    lesson_id INTEGER NOT NULL UNIQUE REFERENCES lessons(id) ON DELETE CASCADE,
    task TEXT NOT NULL
);

DROP TABLE lesson_notes;