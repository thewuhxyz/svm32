// use sp1_build::BuildArgs;

// fn main() {
//     sp1_build::build_program_with_args(
//         "../program",
//         BuildArgs {
//             docker: true,
//             ..Default::default()
//         },
//     );
// }

fn main() {
    sp1_build::build_program("../program");
}
