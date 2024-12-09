use std::collections::VecDeque;

use itertools::Itertools;

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

    pub fn build_file(&self) -> Vec<Option<usize>> {
        let mut file: Vec<Option<usize>> = Vec::with_capacity(self.blocks);
        for bit in (0..self.blocks).map(|_| self.id) {
            file.push(Some(bit));
        }
        for _ in 0..self.free {
            file.push(None);
        }
        return file;
    }

    pub fn size(&self) -> usize {
        self.blocks + self.free
    }
}

#[derive(Debug)]
pub struct DiskMap {
    files: Vec<FileMap>,
    size: usize,
    disk: VecDeque<Option<usize>>,
}

impl DiskMap {
    pub fn from_map(map: &str) -> Self {
        let mut files: Vec<FileMap> = Vec::new();
        let mut disk: VecDeque<Option<usize>> = VecDeque::new();
        let mut size: usize = 0;
        for (idx, (blocks, free)) in map
            .chars()
            .chunks(2)
            .into_iter()
            .map(|mut chunk| {
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
            })
            .enumerate()
        {
            let file = FileMap::new(idx, blocks, free);
            size += file.size();
            for b in file.build_file().iter() {
                disk.push_back(*b);
            }
            files.push(file);
        }

        Self { files, size, disk }
    }

    pub fn defragment(&mut self) {}

    fn fragment_step(&mut self) {}

    pub fn formatted_disk(&self) -> String {
        self.disk.iter().map(|v| {
            match v {
                Some(v) => char::from_digit(*v as u32, 10).unwrap(),
                None => '.',
            }
        }).collect::<String>()
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
        println!("{}", map.formatted_disk());
    }
}