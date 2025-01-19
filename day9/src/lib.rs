use std::{cmp};

#[derive(Copy, Clone, Debug)]
pub enum MemoryBlock {
    Empty {size: u32},
    File {size: u32, file_idx: u32}
}

impl MemoryBlock {

    fn new(size: u32, file_idx: Option<u32>) -> Self {
        match file_idx  {
            Some(file_idx) => Self::File  {size, file_idx},
            None                => Self::Empty {size}
        }
    }

    fn is_empty(&self) -> bool
    {
        match self  {
            Self::Empty{..} => true,
            Self::File{..}  => false
        }
    }

    fn is_file(&self) -> bool
    {
        match self  {
            Self::Empty{..} => false,
            Self::File{..}  => true
        }
    }

    fn size(&self) -> u32 {
        match self  {
            Self::Empty{size} => *size,
            Self::File{size, ..}  => *size
        }
    }

    fn file_idx(&self) -> Option<u32> {
        match self  {
            Self::Empty{..} => None,
            Self::File{file_idx, ..}  => Some(*file_idx)
        }
    }

    fn resize(&mut self, new_size: u32) {
        match self  {
            Self::Empty{size}     => *size = new_size,
            Self::File{size, ..}  => *size = new_size
        }
    }
}

pub fn move_fragments(dest: &mut MemoryBlock, source: &mut MemoryBlock) -> MemoryBlock
{
    match (dest, source)  {
        (MemoryBlock::Empty{size: dest_size},
         MemoryBlock::File {size: source_size, file_idx}) =>
        {
            let moved_size: u32 = cmp::min(*dest_size, *source_size);
            *dest_size -= moved_size;
            *source_size -= moved_size;
            MemoryBlock::File { size: moved_size, file_idx: *file_idx }
        }
        _ => panic!{"Invalid branch"}
    }
}

pub fn print_disk(disk: &[MemoryBlock])
{
    disk.iter()
        .for_each(|m|
        {
            let (size, character) = match m
            {
                MemoryBlock::Empty{size}             => (size, '.'),
                MemoryBlock::File { size, file_idx } => (size,  char::from_digit(*file_idx, 10).unwrap())
            };
            for _ in 0..*size {
                print!("{character}");
            }
        });
    println!();
}

fn parse_input(input: &str) -> Vec<MemoryBlock>
{
    let mut empty_space = true;
    let mut file_idx = 0;

    input.chars()
        .enumerate()
        .map(|(idx, size)|
        {
            empty_space = !empty_space;
            file_idx += if empty_space { 1 } else { 0 };
            MemoryBlock::new
            (
                size.to_digit(10).unwrap_or_else( || panic!("Invalid digit `{size}` at {idx}")),
                if !empty_space { Some(file_idx) } else { None }
            )
        })
        .collect()
}

fn calculate_checksum(disk: &[MemoryBlock]) -> u64
{
    // Use a counter to keep track of the block_idx
    let mut counter = 0;
    disk.iter()
        .map( |mb|
        {
            match mb {
                MemoryBlock::Empty { size } =>
                {
                    counter += size;
                    0
                }
                MemoryBlock::File { size, file_idx } =>
                {
                    let start = counter;
                    let end = counter + size - 1;
                    let sum_of_counter_values: u32 = (start..(end + 1)).sum();
                    //let sum_of_counter_values = size * (2*counter + (size-1)) / 2; // Better but less readable

                    //println!("Checksum: starting at {counter}, with size {size} to {end}, with sum {sum_of_counter_values} or {option_2}");

                    counter += size;
                    (sum_of_counter_values * file_idx) as u64
                }
            }
        })
        .sum()
}

pub fn part1(input: &str) -> u64
{
    let disk_map = parse_input(input);
    //print_disk(&disk_map);

    // Use 2 indices to move from the edges of the vector until they converge
    let mut left_idx  = 0;
    let mut right_idx = disk_map.len() - 1;

    let mut left  = disk_map[left_idx];
    let mut right = disk_map[right_idx];

    let mut defrag_disk = vec!();
    loop {
        if right_idx == left_idx
        {
            break;
        }
        else if left.is_file() // This is a file on the left, put it in the vector
        {
            defrag_disk.push(left);
            left_idx += 1;
            left = disk_map[left_idx];
        }
        else if left.is_empty() && (left.size() == 0) // This is a zero-sized empty space on the left, skip it
        {
            left_idx += 1;
            left = disk_map[left_idx];
        }
        else if right.is_empty() || (right.is_file() && (right.size() == 0)) // This is an empty space or a zero-sized file on the right, skip it
        {
            right_idx -= 1;
            right = disk_map[right_idx];
        }
        else if left.is_empty() && right.is_file() // Left is a non-zero-sized empty space and right is a non-zero-sized file, MOVE IT!
        {
            // Subtract the size from left and right, and create a new memory block to be inserted
            let moved_block = move_fragments(/* dest */ &mut left, /* source */ &mut right);
            defrag_disk.push(moved_block);
            //print_disk(&defrag_disk);
        }
        else
        {
            panic!("Invalid state")
        }
    }

    // There might be some leftover data, don't forget to add it
    if right.is_file() && !right.is_empty() {
        defrag_disk.push(right);
    }

    //print_disk(&defrag_disk);

    // Everything left in the disk should be files
    assert!( defrag_disk.iter().all(|m| m.is_file() ) );

    // Calculate checksum
    calculate_checksum(&defrag_disk)
}

pub fn part2(input: &str) -> u64
{
    let mut disk_map = parse_input(input);
    //print_disk(&disk_map);

    // Get a copy of all the files to be iterated in reverse order
    let files = disk_map.iter()
        .rev()
        .filter(|m| m.is_file() )
        .cloned()
        .collect::<Vec<MemoryBlock>>();

    // For each of the files (right to left)
    for file in files
    {
        // Locate the current file position in the array and size
        let (file_pos, file_size) = disk_map.iter()
            .enumerate()
            .find(|(_, m)| m.file_idx() == file.file_idx())
            .map( |(idx, f)| (idx, f.size()) )
            .unwrap();

        // Find the leftmost space that is big enough
        let leftmost_sufficiently_big_empty_space = disk_map.iter()
            .enumerate()
            .filter(|(block_idx, _)| *block_idx < file_pos )
            .filter(|(_, m)| m.is_empty() )
            .find(|(_, e)| e.size() >= file_size )
            .map( |(idx, e)| (idx, e.size()) );

        // If there are not spaces available, skip this file, otherwise extract position in the array and size
        let Some((empty_pos, empty_size)) = leftmost_sufficiently_big_empty_space else
        {
            continue;
        };

        // If they space and the file are the same size, do a clean swap
        if empty_size == file_size
        {
            disk_map.swap(empty_pos, file_pos);
            // print_disk(&disk_map);
        }
        else
        {
            // If the empty space is bigger than the file, swap the empty space and the file
            // The empty space left behind by the file should have the same size as the file
            disk_map[empty_pos].resize(file_size);
            disk_map.swap(empty_pos, file_pos);

            // Because not all the empty space was occupied, make sure the leftover space is still
            // represented by inserting a new empty space block with the difference in size
            let remaining_space_after_swap = MemoryBlock::Empty { size: empty_size - file_size};
            disk_map.insert(empty_pos + 1, remaining_space_after_swap);
            // print_disk(&disk_map);
        }
    }

    // Calculate checksum
    calculate_checksum(&disk_map)
}
