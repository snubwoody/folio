ALTER TABLE categories
ADD COLUMN is_income_stream BOOLEAN DEFAULT false;

INSERT INTO categories(id,title,created_at,is_income_stream)
SELECT id,title,created_at,true
FROM income_streams;
