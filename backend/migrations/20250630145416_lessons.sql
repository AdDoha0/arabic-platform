-- Пользователи (Telegram)
CREATE TABLE users  (
    id SERIAL PRIMARY KEY,
    telegram_id BIGINT UNIQUE NOT NULL,
    username TEXT,
    joined_at TIMESTAMP DEFAULT now()
);


CREATE TABLE textbooks (
    id SERIAL PRIMARY KEY,
    title TEXT NOT NULL,
    description TEXT,
    level TEXT,
    is_active BOOLEAN NOT NULL DEFAULT true 
);

-- Уроки
CREATE TABLE lessons (
    id SERIAL PRIMARY KEY,
    textbook_id INTEGER NOT NULL REFERENCES textbooks(id) ON DELETE CASCADE,
    title TEXT NOT NULL,
    description TEXT
);


-- Конспекты к уроку
CREATE TABLE lesson_notes (
    id SERIAL PRIMARY KEY,
    lesson_id INTEGER NOT NULL REFERENCES lessons(id) ON DELETE CASCADE,
    content TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT now()
);



-- YouTube-видео к уроку
CREATE TABLE lessons_videos (
    id SERIAL PRIMARY KEY,
    lesson_id INTEGER NOT NULL REFERENCES lessons(id) ON DELETE CASCADE,
    title TEXT,
    youtube_url TEXT NOT NULL
);


-- Прогресс пользователя по урокам
CREATE TYPE progress_status AS ENUM ('incomplete', 'understood', 'confused');


CREATE TABLE user_progress (
    user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    lesson_id INTEGER NOT NULL REFERENCES lessons(id) ON DELETE CASCADE,
    started_at TIMESTAMP,
    last_visited_at TIMESTAMP,
    completed_at TIMESTAMP,
    visits INTEGER DEFAULT 0,
    progress_percent INTEGER DEFAULT 0 CHECK (progress_percent >= 0 AND progress_percent <= 100),
    status progress_status DEFAULT 'incomplete',
    PRIMARY KEY (user_id, lesson_id)
);