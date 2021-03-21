mod min_mega_pokemon_type_coverage;
mod pokemon;
mod r#type;

use min_mega_pokemon_type_coverage::find_min_mega_for_type_coverage;
use min_mega_pokemon_type_coverage::find_min_mega_for_type_coverage_with_primals;
use min_mega_pokemon_type_coverage::print_per_type_ascending;

fn main() {
    println!("# List of Mega by types");
    print_per_type_ascending();

    println!("# Minimum Mega to cover all types");
    find_min_mega_for_type_coverage();

    println!("# Minimum Mega to cover all types (including primals)");
    find_min_mega_for_type_coverage_with_primals();
}
