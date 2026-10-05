use blob_basefee_rs::*;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let fraction = if args.iter().any(|a| a == "prague") { PRAGUE_FRACTION } else { CANCUN_FRACTION };
    if args.iter().any(|a| a == "--table") {
        println!("{:>14} {:>18} {:>18}", "excess gas", "cancun (wei)", "prague (wei)");
        for e in (0..=120_000_000u128).step_by(10_000_000) {
            println!("{:>14} {:>18} {:>18}", e, blob_base_fee(e, CANCUN_FRACTION), blob_base_fee(e, PRAGUE_FRACTION));
        }
        return;
    }
    let Some(excess) = args.first().and_then(|a| a.parse::<u128>().ok()) else {
        eprintln!("usage: blob-basefee-rs EXCESS_BLOB_GAS [--fork prague] | --table");
        std::process::exit(2);
    };
    let fee = blob_base_fee(excess, fraction);
    println!("blob base fee {fee} wei, one blob (131072 gas) = {} wei", fee * 131_072);
}
