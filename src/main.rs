use rust_decimal::Decimal;
use rust_decimal_macros::*;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "house-buyer")]
struct CommandLineOptions {
    #[structopt(short, long)]
    purchase: Decimal,

    #[structopt(short, long)]
    sale: Decimal,

    #[structopt(short, long, default_value = "5")]
    real_estate_agent_commission: Decimal,

    #[structopt(short, long, default_value = "13")]
    hst: Decimal,

    #[structopt(long)]
    lawyer_fees: Decimal,

    #[structopt(short, long)]
    outstanding_mortgage: Decimal,

    #[structopt(long)]
    line_of_credit: Decimal,

    #[structopt(short, long)]
    mortgage_principal: Option<Decimal>,

    #[structopt(short, long)]
    interest_rate: Decimal,

    #[structopt(short, long)]
    amortization_period: u64,

    #[structopt(long)]
    mortgage_penalty: Option<Decimal>,
}

fn main() {
    let command_line_options = CommandLineOptions::from_args();

    let penalty = command_line_options.mortgage_penalty.unwrap_or(dec!(0));

    let outgoing = command_line_options.purchase
        + toronto_land_transfer_tax::tax(command_line_options.purchase).unwrap()
        + (command_line_options.sale
            * (command_line_options.real_estate_agent_commission / dec!(100)))
        + (command_line_options.sale * (command_line_options.hst / dec!(100)))
        + command_line_options.lawyer_fees
        + command_line_options.line_of_credit
        + command_line_options.outstanding_mortgage
        + penalty;

    let incoming = command_line_options.sale;

    let required_mortgage = outgoing - incoming;

    let mortgage = canadian_mortgage::CanadianMortgage::new(
        command_line_options.interest_rate,
        command_line_options.amortization_period,
        canadian_mortgage::PaymentFrequency::Monthly,
    )
    .unwrap();

    println!(
        "mortgage payment: {}",
        mortgage.payment(required_mortgage).unwrap()
    );
}
