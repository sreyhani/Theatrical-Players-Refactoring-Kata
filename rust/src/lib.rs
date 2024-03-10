use std::{cmp::max, collections::HashMap};

use currency_rs::{Currency, CurrencyOpts};
use serde::Deserialize;
use serde_json::{Value, from_str, from_value};

#[derive(Deserialize)]
struct Performance {
    #[serde(alias = "playID")]
    play_id: String,
    audience: u64,
}

#[derive(Deserialize)]
struct Invoice {
    customer: String,
    performances: Vec<Performance>,
}

#[derive(Deserialize)]
struct Play {
    name: String,
    #[serde(alias = "type")]
    p_type: String,
}

type Plays = HashMap<String, Play>;

fn usd(penny: u64) -> Currency {
    let otp = CurrencyOpts::new().set_symbol("$").set_precision(2);
    Currency::new_float(penny as f64 / 100.0, Some(otp))
}

pub fn statement(invoice: Value, plays: Value) -> String {
    let invoice: Invoice = from_value(invoice).unwrap();
    let plays: Plays  = from_value(plays).unwrap();
    render_plain_statement(invoice, plays)
}
fn render_plain_statement(invoice: Invoice, plays: Plays) -> String {

    let mut result = format!("Statement for {}\n", invoice.customer);

    for perf in &invoice.performances {
        // print line for this order
        result += &format!(
            " {}: {} ({} seats)\n",
            play_for(&plays, perf).name,
            usd(amount_for(&plays, perf)).format(),
            perf.audience
        );
    }
    result += &format!(
        "Amount owed is {}\n",
        usd(total_amount(&invoice, &plays)).format()
    );
    result += &format!(
        "You earned {} credits\n",
        total_volume_credits(&invoice, &plays)
    );
    result
}

fn play_for<'a>(plays: &'a Plays, perf: &'a Performance) -> &'a Play {
    &plays[&perf.play_id]
}

fn total_amount(invoice: &Invoice, plays: &Plays) -> u64 {
    let mut result = 0;
    for perf in &invoice.performances {
        result += amount_for(plays, &perf);
    }
    result
}

fn total_volume_credits(invoice: &Invoice, plays: &Plays) -> u64 {
    let mut result = 0;
    for perf in &invoice.performances {
        // add volume credits
        result += volume_credits_for(plays, perf);
    }
    result
}

fn volume_credits_for(plays: &Plays, perf: &Performance) -> u64 {
    let mut result = max(perf.audience - 30, 0);
    // add extra credit for every ten comedy attendees
    if "comedy" == play_for(plays, perf).p_type {
        result += (perf.audience as f64 / 5.0).floor() as u64;
    }
    result
}

fn amount_for(plays: &Plays, perf: &Performance) -> u64 {
    let mut result;
    match play_for(plays, perf).p_type.as_str() {
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
