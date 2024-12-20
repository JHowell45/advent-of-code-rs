use core::file_reader::get_file_contents;

fn main() {
    let (towels, racks) = get_file_contents(2024, 19).split("\n\n").collect::<[Vec<String>; 2]>();
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
}
