#[derive(Debug, Clone)]
pub enum TransactionType {
    Credit,
    Debit,
}

#[derive(Debug, Clone)]
pub struct Expense {
    pub id: u8,
    pub title: String,
    pub amount: u32,
    pub tx_type: TransactionType,
}
