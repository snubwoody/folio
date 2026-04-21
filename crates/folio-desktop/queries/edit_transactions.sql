UPDATE transactions
SET
    amount = COALESCE($1,amount),
    note = COALESCE($2,note),
    transaction_date = COALESCE($3,transaction_date),
    from_account_id = COALESCE($4,from_account_id),
    to_account_id = COALESCE($5,to_account_id),
    category_id = COALESCE($6,category_id)
WHERE id = $7
RETURNING *;
