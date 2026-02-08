CREATE TABLE comments (
    id UUID PRIMARY KEY,
    book_id UUID NOT NULL,
    text TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT fk_book 
        FOREIGN KEY (book_id) 
        REFERENCES books(id) 
        ON DELETE CASCADE
);