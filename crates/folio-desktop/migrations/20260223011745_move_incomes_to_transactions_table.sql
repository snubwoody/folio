INSERT INTO transactions(id,amount,to_account_id,from_account_id,category_id,transaction_date,created_at,note)
SELECT id,amount,account_id,NULL,income_stream,transaction_date,created_at,NULL
FROM incomes;
