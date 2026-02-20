

#[derive(Debug, Clone)]
pub enum TransactionType {
    Credit,
    Debit,
}



#[derive(Debug, Clone)]
pub struct Expense {
    pub id: u8,
    pub name: String,
    pub amount: f64,
    pub tx_type: TransactionType,
}