use rust_decimal::Decimal;
use rust_decimal_macros::*;
use structopt::StructOpt;
use tera::{Context, Tera};

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
    purchase_lawyer_fee: Decimal,

    #[structopt(long)]
    sale_lawyer_fee: Decimal,

    #[structopt(long)]
    title_insurance: Decimal,

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

fn main() -> anyhow::Result<()> {
    let command_line_options = CommandLineOptions::from_args();

    let real_estate_commission =
        command_line_options.sale * (command_line_options.real_estate_commission / dec!(100));
    let real_estate_commission_tax =
        real_estate_commission * (command_line_options.hst / dec!(100));

    let land_transfer_tax = toronto_land_transfer_tax::tax(command_line_options.purchase)?;

    let outstanding_principal = command_line_options.purchase
        + land_transfer_tax
        + real_estate_commission
        + real_estate_commission_tax
        + command_line_options.purchase_lawyer_fee
        + command_line_options.sale_lawyer_fee
        + command_line_options.title_insurance
        + command_line_options.line_of_credit.unwrap_or(dec!(0))
        + command_line_options.outstanding_mortgage
        + command_line_options.mortgage_penalty.unwrap_or(dec!(0))
        - command_line_options.sale;

    let mortgage = canadian_mortgage::CanadianMortgage::new(
        command_line_options.interest_rate,
        command_line_options.amortization_period,
        canadian_mortgage::PaymentFrequency::Monthly,
    )
    .unwrap();

    let monthly_payment = mortgage.payment(outstanding_principal)?;

    let mut tera = Tera::default();
    tera.add_raw_template("report", include_str!("templates/report.tmpl"))?;

    // Using the tera Context struct
    let mut context = Context::new();
    context.insert(
        "purchase",
        &command_line_options
            .purchase
            .round_dp_with_strategy(2, rust_decimal::RoundingStrategy::RoundHalfUp),
    );
    context.insert(
        "land_transfer_tax",
        &land_transfer_tax.round_dp_with_strategy(2, rust_decimal::RoundingStrategy::RoundHalfUp),
    );
    context.insert(
        "commission",
        &real_estate_commission
            .round_dp_with_strategy(2, rust_decimal::RoundingStrategy::RoundHalfUp),
    );
    context.insert(
        "commission_tax",
        &real_estate_commission_tax
            .round_dp_with_strategy(2, rust_decimal::RoundingStrategy::RoundHalfUp),
    );
    context.insert(
        "purchase_lawyer_fee",
        &command_line_options
            .purchase_lawyer_fee
            .round_dp_with_strategy(2, rust_decimal::RoundingStrategy::RoundHalfUp),
    );
    context.insert(
        "sale_lawyer_fee",
        &command_line_options
            .sale_lawyer_fee
            .round_dp_with_strategy(2, rust_decimal::RoundingStrategy::RoundHalfUp),
    );
    context.insert(
        "title_insurance",
        &command_line_options
            .title_insurance
            .round_dp_with_strategy(2, rust_decimal::RoundingStrategy::RoundHalfUp),
    );
    context.insert(
        "outstanding_mortgage",
        &command_line_options
            .outstanding_mortgage
            .round_dp_with_strategy(2, rust_decimal::RoundingStrategy::RoundHalfUp),
    );
    context.insert(
        "mortgage_penalty",
        &command_line_options
            .mortgage_penalty
            .unwrap_or(dec!(0))
            .round_dp_with_strategy(2, rust_decimal::RoundingStrategy::RoundHalfUp),
    );
    context.insert(
        "sale",
        &command_line_options
            .sale
            .round_dp_with_strategy(2, rust_decimal::RoundingStrategy::RoundHalfUp),
    );
    context.insert(
        "interest_rate",
        &command_line_options
            .interest_rate
            .round_dp_with_strategy(2, rust_decimal::RoundingStrategy::RoundHalfUp),
    );
    context.insert(
        "amortization_period",
        &command_line_options.amortization_period,
    );
    context.insert(
        "principal",
        &outstanding_principal
            .round_dp_with_strategy(2, rust_decimal::RoundingStrategy::RoundHalfUp),
    );
    context.insert(
        "payment",
        &monthly_payment.round_dp_with_strategy(2, rust_decimal::RoundingStrategy::RoundHalfUp),
    );

    println!("{}", tera.render("report", &context)?);

    Ok(())
}
