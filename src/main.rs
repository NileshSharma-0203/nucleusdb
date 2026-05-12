mod buffer;
mod catalog;
mod execution;
mod index;
mod sql;
mod storage;
mod transaction;

use execution::optimizer::Optimizer;
use execution::plan::PlanNode;

fn print_plan(plan: &PlanNode) {
    match plan {
        PlanNode::SeqScan { table } => {
            println!("Chosen Plan: SeqScan on {}", table);
        }

        PlanNode::IndexScan {
            table,
            column,
            value,
        } => {
            println!(
                "Chosen Plan: IndexScan on {} using {} = {}",
                table,
                column,
                value
            );
        }
    }
}

fn main() {
    println!("Small table:");

    let small_plan = Optimizer::choose_plan(
        "users",
        8,
        true,
        "id",
        7,
    );

    print_plan(&small_plan);

    println!();
    println!("Large table:");

    let large_plan = Optimizer::choose_plan(
        "users",
        1_000_000,
        true,
        "id",
        7,
    );

    print_plan(&large_plan);
}