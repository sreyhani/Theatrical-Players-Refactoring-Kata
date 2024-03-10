use std::cmp::max;

use crate::types::{Invoice, Performance, Play, Plays};

trait PerformanceCalculator {
    fn amount(&self) -> u64;
}

struct ComedyPerformanceCalculator<'a> {
    perf: &'a Performance,
    play: &'a Play,
}
impl<'a> PerformanceCalculator for ComedyPerformanceCalculator<'a> {
    fn amount(&self) -> u64 {
        0
    }
}

struct TragedyPerformanceCalculator<'a> {
    perf: &'a Performance,
    play: &'a Play,
}
impl<'a> PerformanceCalculator for TragedyPerformanceCalculator<'a> {
    fn amount(&self) -> u64 {
        0
    }
}

pub struct StatementData<'a> {
    pub customer: String,
    pub performances: Vec<EnrichedPerformance<'a>>,
}

pub struct EnrichedPerformance<'a> {
    calculator: Box<dyn PerformanceCalculator + 'a>,
    perf:  &'a Performance,
    plays: &'a Plays,
}

impl<'a> EnrichedPerformance<'a> {
    pub fn new(perf: &'a Performance, plays: &'a Plays) -> Self {
        let calculator = Self::create_performance_calculator(perf, &plays[&perf.play_id]);
        EnrichedPerformance {
            perf,
            plays,
            calculator,
        }
    }
    fn create_performance_calculator(perf: &'a Performance, play: &'a Play) -> Box<dyn PerformanceCalculator + 'a> {
        match play.p_type.as_str() {
            "tragedy" => Box::new(TragedyPerformanceCalculator { perf, play }),
            "comedy" => Box::new(ComedyPerformanceCalculator { perf, play }),
            play_type => panic!("unknown type:{}", play_type),
        }
    }
    pub fn amount(&self) -> u64 {
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

    pub fn volume_credits(&self) -> u64 {
        let mut result = max(self.perf.audience - 30, 0);
        // add extra credit for every ten comedy attendees
        if "comedy" == self.play().p_type {
            result += (self.perf.audience as f64 / 5.0).floor() as u64;
        }
        result
    }

    pub fn play(&self) -> &Play {
        &self.plays[&self.perf.play_id]
    }

    pub fn audience(&self) -> u64 {
        self.perf.audience
    }
}

impl<'a> StatementData<'a> {
    pub fn total_amount(&self) -> u64 {
        let mut result = 0;
        for perf in &self.performances {
            result += perf.amount();
        }
        result
    }

    pub fn total_volume_credits(self) -> u64 {
        let mut result = 0;
        for perf in &self.performances {
            // add volume credits
            result += perf.volume_credits();
        }
        result
    }
}

pub fn create_statement_data<'a>(invoice: &'a Invoice, plays: &'a Plays) -> StatementData<'a> {
    let data = StatementData {
        customer: invoice.customer.clone(),
        performances: invoice
            .performances
            .iter()
            .map(|perf| EnrichedPerformance::new(&perf, &plays))
            .collect(),
    };
    data
}
