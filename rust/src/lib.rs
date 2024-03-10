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

fn usd(penny: u64) -> Currency {
    let otp = CurrencyOpts::new().set_symbol("$").set_precision(2);
    Currency::new_float(penny as f64 / 100.0, Some(otp))
}

pub fn statement(invoice: Value, plays: Value) -> String {

    let mut result = format!("Statement for {}\n", invoice["customer"].as_str().unwrap());

    for perf in invoice["performances"].as_array().unwrap() {
        // print line for this order
        result += &format!(
            " {}: {} ({} seats)\n",
            play_for(&plays, perf)["name"].as_str().unwrap(),
            usd(amount_for(&plays, perf)).format(),
            perf["audience"].as_u64().unwrap()
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

fn play_for<'a>(plays: &'a Value, perf: &'a Value) -> &'a Value {
    &plays[perf["playID"].as_str().unwrap()]
}

fn total_amount(invoice: &Value, plays: &Value) -> u64 {
    let mut result = 0;
    for perf in invoice["performances"].as_array().unwrap() {
        result += amount_for(plays, perf);
    }
    result
}

fn total_volume_credits(invoice: &Value, plays: &Value) -> u64 {
    let mut result = 0;
    for perf in invoice["performances"].as_array().unwrap() {
        // add volume credits
        result += volume_credits_for(plays, perf);
    }
    result
}

fn volume_credits_for(plays: &Value, perf: &Value) -> u64 {
    let mut result = max(perf["audience"].as_u64().unwrap() - 30, 0);
    // add extra credit for every ten comedy attendees
    if "comedy" == play_for(plays, perf)["type"].as_str().unwrap() {
        result += (perf["audience"].as_f64().unwrap() / 5.0).floor() as u64;
    }
    result
}

fn amount_for(plays: &Value, perf: &Value) -> u64 {
    let mut result;
    match play_for(plays, perf)["type"].as_str().unwrap() {
        "tragedy" => {
            result = 40000;
            if perf["audience"].as_u64().unwrap() > 30 {
                result += 1000 * (perf["audience"].as_u64().unwrap() - 30);
            }
        }
        "comedy" => {
            result = 30000;
            if perf["audience"].as_u64().unwrap() > 20 {
                result += 10000 + 500 * (perf["audience"].as_u64().unwrap() - 20);
            }
            result += 300 * perf["audience"].as_u64().unwrap();
        }
        play_type => {
            panic!("unknown type:{}", play_type);
        }
    }
    result
}
