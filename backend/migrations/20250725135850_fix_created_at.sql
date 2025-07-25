
ALTER TABLE lessons 
    DROP COLUMN created_at;

ALTER TABLE lessons 
    ADD COLUMN created_at TIMESTAMP NOT NULL DEFAULT now();