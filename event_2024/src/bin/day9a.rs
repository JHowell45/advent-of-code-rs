use itertools::Itertools;

fn main() {}

#[derive(Debug)]
pub struct FileMap {
    id: usize,
    blocks: usize,
    free: usize,
}

impl FileMap {
    pub fn new(id: usize, blocks: usize, free: usize) -> Self {
        Self { id, blocks, free }
    }
}

#[derive(Debug)]
pub struct DiskMap {
    files: Vec<FileMap>,
}

impl DiskMap {
    pub fn from_map(map: &str) -> Self {
        let mut files: Vec<FileMap> = Vec::new();
        for (idx, (blocks, free)) in map.chars().chunks(2).into_iter().map(|mut chunk| {
            (
                match chunk.next() {
                    Some(n) => n.to_string().parse::<usize>().unwrap(),
                    None => 0,
                },
                match chunk.next() {
                    Some(n) => n.to_string().parse::<usize>().unwrap(),
                    None => 0,
                },
            )
        }).enumerate() {
            files.push(FileMap::new(idx, blocks, free));
        }

        Self { files }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("2333133121414131402")]
    fn example(#[case] disk_map: &str) {
        let map = DiskMap::from_map(disk_map);
        println!("{map:#?}");
    }
}
