use crate::chain::Chain;

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
    let student_merkletree_1 = MerkleTree::new([
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
    let student_merkletree_2 = MerkleTree::new([
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
    let student_block_1 = Block::new(student_merkletree_1);
    let student_block_2 = Block::new(student_merkletree_2);
    let mut my_chain: Chain<Student> = Chain::new();
    my_chain.add_new_block(student_block_1);
    my_chain.add_new_block(student_block_2);
    my_chain.view_chain();
    // println!("{}", )
}
