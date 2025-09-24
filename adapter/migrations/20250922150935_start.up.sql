-- Add up migration script here
-- 1. updated_atを自動更新する関数の作成
CREATE OR REPLACE FUNCTION set_updated_at() RETURNS trigger AS '
BEGIN
    new.updated_at := ''now'';
    return new;
END;
' LANGUAGE 'plpgsql';

-- 2. booksテーブルの作成
CREATE TABLE IF NOT EXISTS books (
    bookd_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    title VARCHAR(255) NOT NULL,
    author VARCHAR(255) NOT NULL,
    isbn VARCHAR(255) NOT NULL,
    description VARCHAR(1024) NOT NULL,
    created_at TIMESTAMP(3) WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP(3),
    updated_at TIMESTAMP(3) WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP(3)
);

-- 3. booksテーブルへのトリガーの追加
CREATE TRIGGER books_updated_at_trigger
    BEFORE UPDATE ON books FOR EACH ROW
    EXECUTE PROCEDURE set_updated_at();
