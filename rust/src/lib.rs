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

struct StatementData<'a> {
    customer: String,
    performances: Vec<EnrichedPerformance<'a>>,
}

struct EnrichedPerformance<'a> {
    perf: Performance,
    plays: &'a Plays,
}
impl<'a> EnrichedPerformance<'a> {
    fn new(perf: Performance, plays: &'a Plays) -> Self {
        EnrichedPerformance { perf, plays }
    }
    fn amount(&self) -> u64 {
        let mut result;
        match self.play().p_type.as_str() {
            "tragedy" => {
                result = 40000;
                if self.perf.audience > 30 {
                    result += 1000 * (self.perf.audience - 30);
                }
            }
            "comedy" => {
                result = 30000;
                if self.perf.audience > 20 {
                    result += 10000 + 500 * (self.perf.audience - 20);
                }
                result += 300 * self.perf.audience;
            }
            play_type => {
                panic!("unknown type:{}", play_type);
            }
        }
        result
    }

    fn volume_credits(&self) -> u64 {
        let mut result = max(self.perf.audience - 30, 0);
        // add extra credit for every ten comedy attendees
        if "comedy" == self.play().p_type {
            result += (self.perf.audience as f64 / 5.0).floor() as u64;
        }
        result
    }

    fn play(&self) -> &Play {
        &self.plays[&self.perf.play_id]
    }

    fn audience(&self) -> u64 {
        self.perf.audience
    }
}

impl<'a> StatementData<'a> {
    fn total_amount(&self) -> u64 {
        let mut result = 0;
        for perf in &self.performances {
            result += perf.amount();
        }
        result
    }

    fn total_volume_credits(self) -> u64 {
        let mut result = 0;
        for perf in &self.performances {
            // add volume credits
            result += perf.volume_credits();
        }
        result
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
        performances: invoice
            .performances
            .iter()
            .map(|perf| EnrichedPerformance::new(perf.clone(), &plays))
            .collect(),
    };
    render_plain_statement(data)
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
