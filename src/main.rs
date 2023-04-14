use std::fmt::Display;

use crate::merkle::MerkleTree;
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
    // let mut string_t = MerkleTree::new(["1", "2", "3", "4"]);
    // string_t.build_tree();
    // println!("{}", string_t);

    let mut struct_t = MerkleTree::new([
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

    struct_t.build_tree();
    println!("{}", struct_t);
}
