use rust_decimal::Decimal;
use rust_decimal_macros::*;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "home-buyer")]
struct CommandLineOptions {
    #[structopt(short, long)]
    amortization_period: u64,

    #[structopt(short, long, default_value = "13")]
    hst: Decimal,

    #[structopt(short, long)]
    interest_rate: Decimal,

    #[structopt(long)]
    lawyer_fees: Decimal,

    #[structopt(long)]
    line_of_credit: Option<Decimal>,

    #[structopt(long)]
    mortgage_penalty: Option<Decimal>,

    #[structopt(short, long)]
    outstanding_mortgage: Decimal,

    #[structopt(short, long)]
    purchase: Decimal,

    #[structopt(short, long, default_value = "5")]
    real_estate_commission: Decimal,

    #[structopt(short, long)]
    sale: Decimal,
}

fn main() {
    let command_line_options = CommandLineOptions::from_args();

    let real_estate_commission =
        command_line_options.sale * (command_line_options.real_estate_commission / dec!(100));
    let real_estate_commission_tax =
        real_estate_commission * (command_line_options.hst / dec!(100));

    let outgoing = command_line_options.purchase
        + toronto_land_transfer_tax::tax(command_line_options.purchase).unwrap()
        + real_estate_commission
        + real_estate_commission_tax
        + command_line_options.lawyer_fees
        + command_line_options.line_of_credit.unwrap_or(dec!(0))
        + command_line_options.outstanding_mortgage
        + command_line_options.mortgage_penalty.unwrap_or(dec!(0));

    let required_mortgage = outgoing - command_line_options.sale;

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
