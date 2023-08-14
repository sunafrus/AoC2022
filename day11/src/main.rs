use miette::GraphicalReportHandler;
use nom_supreme::{
    error::{BaseErrorKind, ErrorTree, GenericErrorTree},
    final_parser::final_parser,
};

mod parse;
use parse::{parse_all_monkeys, Span, Monkey};

#[derive(thiserror::Error, Debug, miette::Diagnostic)]
#[error("bad input")]
struct BadInput {
    #[source_code]
    src: &'static str,

    #[label("{kind}")]
    bad_bit: miette::SourceSpan,

    kind: BaseErrorKind<&'static str, Box<dyn std::error::Error + Send + Sync>>,
}

fn main() {
    let input_static = concat!(include_str!("input.txt"), "\r\n");
    let input = Span::new(input_static);

    let monkeys_res: Result<_, ErrorTree<Span>> =
        final_parser(parse_all_monkeys::<ErrorTree<Span>>)(input);
    let monkeys = match monkeys_res {
        Ok(monkeys) => monkeys,
        Err(e) => {
            match e {
                GenericErrorTree::Base { location, kind } => {
                    let offset = location.location_offset().into();
                    let err = BadInput {
                        src: input_static,
                        bad_bit: miette::SourceSpan::new(offset, 0.into()),
                        kind,
                    };
                    let mut s = String::new();
                    GraphicalReportHandler::new()
                        .render_report(&mut s, &err)
                        .unwrap();
                    println!("{s}");
                }
                GenericErrorTree::Stack { .. } => todo!("stack"),
                GenericErrorTree::Alt(_) => todo!("alt"),
            }
            return;
        }
    };

    let divisor_product = monkeys.iter().map(|m| m.divisor).product::<u64>();
    dbg!(divisor_product);
    
    let mut monkeys = monkeys;
    for _ in 0..10_000 {
        do_round(&mut monkeys, divisor_product);
    }

    let mut all_inspect_counts = monkeys
        .iter()
        .map(|m| m.items_inspected)
        .collect::<Vec<_>>();

    all_inspect_counts.sort_by_key(|&c| std::cmp::Reverse(c));

    let monkey_business = all_inspect_counts.into_iter().take(2).product::<u64>();

    dbg!(monkey_business);
}

fn do_round(monkeys: &mut [Monkey], divisor_product: u64) {
    let num_monkeys = monkeys.len();

    for i in 0..num_monkeys {
        let mc;

        {
            let monkey = &mut monkeys[i];
            mc = monkey.clone();
            monkey.items_inspected += mc.items.len() as u64;
        }

        for mut item in mc.items.iter().copied() {
            item %= divisor_product;
            item = mc.operation.eval(item);

            if item % mc.divisor == 0 {
                monkeys[mc.receiver_if_true].items.push(item);
            } else {
                monkeys[mc.receiver_if_false].items.push(item);
            }
        }
        monkeys[i].items.clear();
    }
}