use std::collections::VecDeque;

use itertools::Itertools;

#[derive(Debug, Clone, Copy)]
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
        file
    }

    pub fn size(&self) -> usize {
        self.blocks + self.free
    }
}

#[derive(Debug)]
pub struct DiskMap {
    pub files: Vec<FileMap>,
    disk: VecDeque<Option<usize>>,
}

impl DiskMap {
    pub fn from_map(map: &str) -> Self {
        let mut files: Vec<FileMap> = Vec::new();
        let mut disk: VecDeque<Option<usize>> = VecDeque::new();
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
            for b in file.build_file().iter() {
                disk.push_back(*b);
            }
            files.push(file);
        }

        Self { files, disk }
    }

    pub fn defragment(&mut self) {
        let max: usize = self.disk.len();
        for idx in 0..max {
            if self.disk.get_mut(idx).unwrap().is_none() {
                for swap_idx in (idx..max).rev() {
                    if self.disk.get_mut(swap_idx).unwrap().is_some() {
                        self.disk.swap(idx, swap_idx);
                        break;
                    }
                }
            }
        }
    }

    pub fn defragment_files(&mut self) {
        let mut new_files: Vec<FileMap> = self.files.clone();
        let len: usize = self.files.len();

        // println!("{}", Self::print_files(new_files.clone()));
        for file_idx in (1..len).rev() {
            let new_files_idx: usize = new_files.iter().position(|f| f.id == file_idx).unwrap();
            let mut file = new_files.get(new_files_idx).unwrap().clone();
            // println!("{file_idx} / {new_files_idx} : {file:?}");

            for prior_idx in 0..new_files_idx {
                let prior_file = new_files.get_mut(prior_idx).unwrap();
                if prior_file.free > 0 && file.blocks <= prior_file.free {
                    let size = file.size();
                    // println!("File: {file:?}");
                    file.free = prior_file.free - file.blocks;
                    // println!("File: {file:?}");
                    prior_file.free = 0;
                    // println!("Prior File: {prior_file:?}");
                    let old_prior_file = new_files.get_mut(new_files_idx - 1).unwrap();
                    // println!("Old Prior File: {old_prior_file:?}");
                    old_prior_file.free += size;
                    // println!("Old Prior File: {old_prior_file:?}");
                    new_files.remove(new_files_idx);
                    new_files.insert(prior_idx + 1, file);
                    // println!("{new_files:?}");
                    break;
                }
                // println!("{file_idx} / {new_files_idx} : {file:?}");
                // println!("{prior_idx} : {prior_file:?}");
            }
            // println!("{}: {}", file.id, Self::print_files(new_files.clone()));
        }

        self.files = new_files.clone();
        self.disk = self.build_disk(new_files);
    }

    pub fn checksum(&self) -> u64 {
        self.disk
            .iter()
            .enumerate()
            .map(|(idx, v)| match v {
                Some(v) => idx as u64 * *v as u64,
                None => 0,
            })
            .sum()
    }

    pub fn formatted_disk(&self) -> String {
        Self::print_files(self.files.clone())
    }

    fn print_files(files: Vec<FileMap>) -> String {
        files
            .iter()
            .map(|f| {
                f.build_file()
                    .iter()
                    .map(|c| match c {
                        Some(v) => format!("{v}"),
                        None => ".".to_string(),
                    })
                    .collect::<String>()
            })
            .collect::<String>()
    }

    fn build_disk(&mut self, files: Vec<FileMap>) -> VecDeque<Option<usize>> {
        let mut disk: VecDeque<Option<usize>> = VecDeque::new();
        for file in files.iter() {
            // println!("{:?}", file);
            for b in file.build_file().iter() {
                disk.push_back(*b);
            }
        }
        disk
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("2333133121414131402", "00...111...2...333.44.5555.6666.777.888899")]
    #[case("12345", "0..111....22222")]
    fn test_formatted_disk(#[case] disk_map: &str, #[case] expected_disk: &str) {
        let mut map = DiskMap::from_map(disk_map);
        assert_eq!(map.formatted_disk(), expected_disk);
        map.defragment();
    }

    #[rstest]
    #[case("2333133121414131402", "00992111777.44.333....5555.6666.....8888..")]
    fn test_defragmented_file(#[case] disk_map: &str, #[case] expected_disk: &str) {
        let mut map = DiskMap::from_map(disk_map);
        map.defragment_files();
        assert_eq!(map.formatted_disk(), expected_disk);
    }
}
