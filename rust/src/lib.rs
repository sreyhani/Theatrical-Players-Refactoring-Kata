use std::cmp::max;

use currency_rs::{Currency, CurrencyOpts};
use serde_json::Value;

fn usd(penny: u64) -> Currency {
    let otp = CurrencyOpts::new().set_symbol("$").set_precision(2);
    Currency::new_float(penny as f64 / 100.0, Some(otp))
}

pub fn statement(invoice: Value, plays: Value) -> String {
    let play_for = |perf: &Value| -> &Value { &plays[perf["playID"].as_str().unwrap()] };

    let amount_for = |perf: &Value, play: &Value| -> u64 {
        let mut result;
        match play["type"].as_str().unwrap() {
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
    };

    let volume_credits_for = |perf: &Value| -> u64 {
        let mut result = 0;
        // add volume credits
        result += max(perf["audience"].as_u64().unwrap() - 30, 0);
        // add extra credit for every ten comedy attendees
        if "comedy" == play_for(perf)["type"].as_str().unwrap() {
            result += (perf["audience"].as_f64().unwrap() / 5.0).floor() as u64;
        }
        result
    };

    let total_amount = || -> u64 {
        let mut result = 0;
        for perf in invoice["performances"].as_array().unwrap() {
            result += amount_for(perf, play_for(perf));
        }
        result
    };

    let total_volume_credits = || -> u64 {
        let mut result = 0;
        for perf in invoice["performances"].as_array().unwrap() {
            result += volume_credits_for(perf);
        }
        result
    };
    
    let mut result = format!("Statement for {}\n", invoice["customer"].as_str().unwrap());
    for perf in invoice["performances"].as_array().unwrap() {
        // print line for this order
        result += &format!(
            " {}: {} ({} seats)\n",
            play_for(perf)["name"].as_str().unwrap(),
            usd(amount_for(perf, play_for(perf))).format(),
            perf["audience"].as_u64().unwrap()
        );
    }
    result += &format!("Amount owed is {}\n", usd(total_amount()).format());
    result += &format!("You earned {} credits\n", total_volume_credits());
    result
}
