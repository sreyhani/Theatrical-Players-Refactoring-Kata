use std::cmp::max;

use currency_rs::{Currency, CurrencyOpts};
use serde_json::Value;

fn usd(value: f64) -> Currency {
    let otp = CurrencyOpts::new().set_symbol("$").set_precision(2);
    Currency::new_float(value, Some(otp))
}

pub fn statement(invoice: Value, plays: Value) -> String {
    let mut total_amount = 0;
    let mut volume_credits = 0;
    let mut result = format!("Statement for {}\n", invoice["customer"].as_str().unwrap());
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

    for perf in invoice["performances"].as_array().unwrap() {
        volume_credits += volume_credits_for(perf);
        // print line for this order
        result += &format!(
            " {}: {} ({} seats)\n",
            play_for(perf)["name"].as_str().unwrap(),
            usd(amount_for(perf, play_for(perf)) as f64 / 100 as f64).format(),
            perf["audience"].as_u64().unwrap()
        );
        total_amount += amount_for(perf, play_for(perf));
    }

    result += &format!(
        "Amount owed is {}\n",
        usd(total_amount as f64 / 100 as f64).format()
    );
    result += &format!("You earned {} credits\n", volume_credits);
    result
}
