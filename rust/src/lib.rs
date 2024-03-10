use std::{cmp::max, collections::HashMap};

use currency_rs::{Currency, CurrencyOpts};
use serde::Deserialize;
use serde_json::{from_value, Value};

#[derive(Deserialize, Clone)]
struct Performance {
    #[serde(alias = "playID")]
    play_id: String,
    audience: u64,
}

#[derive(Deserialize, Clone)]
struct Invoice {
    customer: String,
    performances: Vec<Performance>,
}

#[derive(Deserialize, Clone)]
struct Play {
    name: String,
    #[serde(alias = "type")]
    p_type: String,
}

type Plays = HashMap<String, Play>;

struct StatementData {
    customer: String,
    performances: Vec<Performance>,
    plays: Plays,
}

impl StatementData {
    fn total_amount(&self) -> u64 {
        let mut result = 0;
        for perf in &self.performances {
            result += self.amount_for(&perf);
        }
        result
    }

    fn amount_for(&self, perf: &Performance) -> u64 {
        let mut result;
        match self.play_for(perf).p_type.as_str() {
            "tragedy" => {
                result = 40000;
                if perf.audience > 30 {
                    result += 1000 * (perf.audience - 30);
                }
            }
            "comedy" => {
                result = 30000;
                if perf.audience > 20 {
                    result += 10000 + 500 * (perf.audience - 20);
                }
                result += 300 * perf.audience;
            }
            play_type => {
                panic!("unknown type:{}", play_type);
            }
        }
        result
    }

    fn total_volume_credits(self) -> u64 {
        let mut result = 0;
        for perf in &self.performances {
            // add volume credits
            result += self.volume_credits_for(perf);
        }
        result
    }

    fn volume_credits_for(&self, perf: &Performance) -> u64 {
        let mut result = max(perf.audience - 30, 0);
        // add extra credit for every ten comedy attendees
        if "comedy" == self.play_for(perf).p_type {
            result += (perf.audience as f64 / 5.0).floor() as u64;
        }
        result
    }

    fn play_for(&self, perf: &Performance) -> &Play {
        &self.plays[&perf.play_id]
    }
}

fn usd(penny: u64) -> Currency {
    let otp = CurrencyOpts::new().set_symbol("$").set_precision(2);
    Currency::new_float(penny as f64 / 100.0, Some(otp))
}

pub fn statement(invoice: Value, plays: Value) -> String {
    let invoice: Invoice = from_value(invoice).unwrap();
    let plays: Plays = from_value(plays).unwrap();
    let data = StatementData {
        customer: invoice.customer,
        performances: invoice.performances,
        plays: plays,
    };
    render_plain_statement(data)
}

fn render_plain_statement(data: StatementData) -> String {
    let mut result = format!("Statement for {}\n", data.customer);

    for perf in &data.performances {
        // print line for this order
        result += &format!(
            " {}: {} ({} seats)\n",
            data.play_for(perf).name,
            usd(data.amount_for(perf)).format(),
            perf.audience
        );
    }
    result += &format!("Amount owed is {}\n", usd(data.total_amount()).format());
    result += &format!("You earned {} credits\n", data.total_volume_credits());
    result
}
