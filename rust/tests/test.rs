#[cfg(test)]
mod tests {
    use insta::assert_snapshot;
    use serde_json::json;
    use theatrical_players::{plain_statement, html_statement};

    #[test]
    fn example_plain_statement() {
        let invoice = json!({
            "customer": "BigCo",
            "performances": [
                {
                    "playID": "hamlet",
                    "audience": 55
                },
                {
                    "playID": "as-like",
                    "audience": 35
                },
                {
                    "playID": "othello",
                    "audience": 40
                }
            ]
        });

        let plays = json!({
            "hamlet": {
                "name": "Hamlet",
                "type": "tragedy"
            },
            "as-like": {
                "name": "As You Like It",
                "type": "comedy"
            },
            "othello": {
                "name": "Othello",
                "type": "tragedy"
            }
        });
        let result = plain_statement(invoice, plays);
        assert_snapshot!(result, @r###"
                        Statement for BigCo
                         Hamlet: $650.00 (55 seats)
                         As You Like It: $580.00 (35 seats)
                         Othello: $500.00 (40 seats)
                        Amount owed is $1,730.00
                        You earned 47 credits
                        "###);
    }

    #[test]
    fn example_html_statement() {
        let invoice = json!({
            "customer": "BigCo",
            "performances": [
                {
                    "playID": "hamlet",
                    "audience": 55
                },
                {
                    "playID": "as-like",
                    "audience": 35
                },
                {
                    "playID": "othello",
                    "audience": 40
                }
            ]
        });

        let plays = json!({
            "hamlet": {
                "name": "Hamlet",
                "type": "tragedy"
            },
            "as-like": {
                "name": "As You Like It",
                "type": "comedy"
            },
            "othello": {
                "name": "Othello",
                "type": "tragedy"
            }
        });
        let result = html_statement(invoice, plays);
        assert_snapshot!(result, @r###"
        <h1>Statement for BigCo</h1>
        <table>
        <tr><th>play</th><th>seats</th><th>cost</th></tr> <tr><td>Hamlet</td><td>55</td><td>650.00</td></tr>
         <tr><td>As You Like It</td><td>35</td><td>580.00</td></tr>
         <tr><td>Othello</td><td>40</td><td>500.00</td></tr>
        </table>
        <p>Amount owed is <em>1730.00</em></p>
        <p>You earned <em>47</em> credits</p>
        "###);
    }

    #[test]
    #[should_panic(expected = "unknown type")]
    fn statement_with_new_play_types() {
        let invoice = json!({
            "customer": "BigCoII",
            "performances":[
                {
                    "playID": "henry-v",
                    "audience": 53
                },
                {
                    "playID": "as-like",
                    "audience": 55
                }
            ]
        });

        let plays = json!({
            "henry-v": {"name": "Henry V", "type": "history"},
            "as-like": {"name": "As You Like It", "type": "pastoral"}
        });
        plain_statement(invoice, plays);
    }
}
