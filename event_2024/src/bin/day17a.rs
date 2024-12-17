use event_2024::shared::day17::Computer;

fn main() {
    let mut computer = Computer::new();
    computer.run(vec![0, 1, 2, 3])
}

#[cfg(test)]
mod tests {}
