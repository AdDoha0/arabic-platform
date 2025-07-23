ALTER TABLE lessons 
    ADD COLUMN created_at TIMESTAMP DEFAULT now();
    
ALTER TABLE lesson_notes 
    DROP COLUMN created_at;