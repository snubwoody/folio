INSERT INTO transactions(id,amount,from_account_id,to_account_id,category_id,transaction_date,created_at,note)
SELECT id,amount,account_id,NULL, category_id,transaction_date,created_at,NULL
FROM expenses;
