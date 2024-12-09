use std::collections::{HashSet, VecDeque};

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
            // println!("{}", self.formatted_disk());
        }
    }

    pub fn defragment_files(&mut self) {
        let mut moved_ids: HashSet<usize> = HashSet::new();
        let mut new_files: Vec<FileMap> = self.files.clone();
        // println!("{new_files:?}");
        for file in self.files.iter().rev() {
            let file_idx = new_files.iter().position(|f| f.id == file.id).unwrap();
            // println!("File: {file:?} : {file_idx:} : {} : {}", file.free, file.size());
            if !moved_ids.contains(&file.id) {
                moved_ids.insert(file.id);
                for idx in 0..file_idx {
                    let check_file = new_files.get_mut(idx).unwrap();
                    // println!("Check File: {check_file:?} : {idx:} : {} : {}", check_file.free, check_file.size());
                    if file.blocks <= check_file.free {
                        let new_free = check_file.free - file.blocks;
                        check_file.free = 0;
                        let mut next = new_files.get_mut(file_idx).unwrap().clone();
                        let mut prev = new_files.get_mut(file_idx - 1).unwrap();
                        // println!("Next: {next:?} : {idx:} : {} : {}", next.free, next.size());

                        prev.free += next.size();
                        next.free = new_free;
                        new_files.remove(file_idx);
                        new_files.insert(idx + 1, next);
                        break;
                    }
                }
            }
            // println!("{new_files:?}");
            let disk = Self::create_disk(new_files.clone());
            // println!("{}", Self::display_disk(disk));
        }
        // println!("{new_files:?}");
        self.disk = self.build_disk(new_files);
        println!("{}\n", self.formatted_disk());
    }

    pub fn checksum(&self) -> usize {
        let mut sum: usize = 0;
        for (idx, bit) in self.disk.iter().enumerate() {
            if let Some(v) = bit {
                sum += idx * v;
            }
        }
        return sum;
        // self.disk
        //     .iter()
        //     .filter(|v| v.is_some())
        //     .enumerate()
        //     .map(|(idx, v)| idx * v.unwrap())
        //     .sum()
    }

    pub fn formatted_disk(&self) -> String {
        self.disk
            .iter()
            .map(|v| match v {
                Some(v) => char::from_digit(*v as u32, 10).unwrap(),
                None => '.',
            })
            .collect::<String>()
    }

    fn build_disk(&mut self, files: Vec<FileMap>) -> VecDeque<Option<usize>> {
        let mut disk: VecDeque<Option<usize>> = VecDeque::new();
        for file in files.iter() {
            for b in file.build_file().iter() {
                disk.push_back(*b);
            }
        }
        return disk;
    }

    fn display_disk(disk: VecDeque<Option<usize>>) -> String {
        disk
            .iter()
            .map(|v| match v {
                Some(v) => char::from_digit(*v as u32, 10).unwrap(),
                None => '.',
            })
            .collect::<String>()
    }

    fn create_disk(files: Vec<FileMap>) -> VecDeque<Option<usize>> {
        let mut disk: VecDeque<Option<usize>> = VecDeque::new();
        for file in files.iter() {
            for b in file.build_file().iter() {
                disk.push_back(*b);
            }
        }
        return disk;
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
