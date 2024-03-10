mod statement_data;

use currency_rs::{Currency, CurrencyOpts};
use serde_json::{from_value, Value};
use statement_data::{create_statement_data, StatementData};

mod types;

fn usd(penny: u64) -> Currency {
    let otp = CurrencyOpts::new().set_symbol("$").set_precision(2);
    Currency::new_float(penny as f64 / 100.0, Some(otp))
}

pub fn statement(invoice: Value, plays: Value) -> String {
    let invoice: types::Invoice = from_value(invoice).unwrap();
    let plays: types::Plays = from_value(plays).unwrap();
    render_plain_statement(create_statement_data(&invoice, &plays))
}

fn render_plain_statement(data: StatementData) -> String {
    let mut result = format!("Statement for {}\n", data.customer);

    for perf in &data.performances {
        // print line for this order
        result += &format!(
            " {}: {} ({} seats)\n",
            perf.play().name,
            usd(perf.amount()).format(),
            perf.audience()
        );
    }
    result += &format!("Amount owed is {}\n", usd(data.total_amount()).format());
    result += &format!("You earned {} credits\n", data.total_volume_credits());
    result
}
