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

    #[structopt(short, long)]
    land_transfer_tax: Decimal,

    #[structopt(short, long)]
    real_estate_agent_commission: Decimal,

    #[structopt(short, long)]
    hst: Decimal,

    #[structopt(long)]
    lawyer_fees: Decimal,

    #[structopt(short, long)]
    outstanding_mortgage: Decimal,

    #[structopt(long)]
    line_of_credit: Decimal,
}

fn main() {
    let mortgage = canadian_mortgage::CanadianMortgage::new(
        dec!(850000),
        dec!(1.79),
        30,
        canadian_mortgage::PaymentFrequency::Monthly,
    )
    .unwrap();

    println!("mortgage payment: {}", mortgage.payment().unwrap());
    println!(
        "what can I affort? {}",
        mortgage.affordability(dec!(4000)).unwrap()
    );

    let command_line_options = CommandLineOptions::from_args();

    let outgoing = command_line_options.purchase
        + (command_line_options.sale * command_line_options.real_estate_agent_commission)
        + (command_line_options.sale
            * (dec!(1) + (command_line_options.real_estate_agent_commission / dec!(100)))
            * (dec!(1) + (command_line_options.hst / dec!(100))))
        + command_line_options.land_transfer_tax
        + command_line_options.lawyer_fees
        + command_line_options.line_of_credit
        + command_line_options.outstanding_mortgage;

    let incoming = command_line_options.sale;

    println!("balance: {}", incoming - outgoing);
}
