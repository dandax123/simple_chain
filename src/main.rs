use crate::block::Block;
use crate::merkle::MerkleTree;
use std::fmt::Display;

mod block;
mod chain;
mod hash;
mod merkle;
mod node;

#[derive(Debug, Default)]
enum Gender {
    #[default]
    Male,
    Female,
}
#[derive(Debug, Default)]
struct Student {
    name: String,
    age: u8,
    gender: Gender,
    student_no: u32,
}

impl Display for Gender {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            &Gender::Female => write!(f, "Male"),
            Gender::Male => write!(f, "Female"),
        }
    }
}

impl Display for Student {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "{}-{}-{}-{}",
            self.student_no, self.age, self.gender, self.name
        )?;
        Ok(())
    }
}

fn main() {
    let mut student_merkletree = MerkleTree::new([
        Student {
            name: "121".to_string(),
            age: 4,
            gender: Gender::Male,
            student_no: 10129012,
        },
        Student {
            name: "121".to_string(),
            age: 4,
            gender: Gender::Male,
            student_no: 10129012,
        },
        Student {
            name: "121".to_string(),
            age: 4,
            gender: Gender::Male,
            student_no: 10129012,
        },
        Student {
            name: "121".to_string(),
            age: 4,
            gender: Gender::Female,
            student_no: 10129012,
        },
    ]);

    student_merkletree.build_tree();
    // println!("{}", struct_t);
    let mut student_block = Block::new("", student_merkletree);
    // block.mine_block();

    // println!("{}", )
}
