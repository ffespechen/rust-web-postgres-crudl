-- migrations/0001_create_books.sql

CREATE TABLE IF NOT EXISTS books (
    id UUID PRIMARY KEY,
    title TEXT NOT NULL,
    author TEXT NOT NULL,
    published_date INTEGER NOT NULL
);