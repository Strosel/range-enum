use range_enum::range_enum;

#[range_enum(1..=2)]
enum Range {}

fn main() {
    let _: Range = Range::Range1;
}
