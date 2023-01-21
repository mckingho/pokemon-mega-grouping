mod core;
mod group_by_types;
mod min_mega_pokemon_type_coverage;
mod pokemon;
mod r#type;

use group_by_types::print_per_type_ascending;
use min_mega_pokemon_type_coverage::find_min_mega_for_type_coverage;
use min_mega_pokemon_type_coverage::find_min_mega_for_type_coverage_pogo_primals;

fn main() {
    println!("# List of Mega by types");
    print_per_type_ascending();

    println!("# Minimum Mega to cover all types");
    find_min_mega_for_type_coverage();

    println!("# Minimum type coverage with Primals in Pokemon Go");
    find_min_mega_for_type_coverage_pogo_primals();
}
