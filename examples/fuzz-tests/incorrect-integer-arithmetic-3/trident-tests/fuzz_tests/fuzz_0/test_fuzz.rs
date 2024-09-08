use fuzz_instructions::incorrect_integer_arithmetic_3_fuzz_instructions::InitVesting;
use trident_client::fuzzing::*;

mod fuzz_instructions;

use incorrect_integer_arithmetic_3::entry as entry_incorrect_integer_arithmetic_3;
use incorrect_integer_arithmetic_3::ID as PROGRAM_ID_INCORRECT_INTEGER_ARITHMETIC_3;

const PROGRAM_NAME_INCORRECT_INTEGER_ARITHMETIC_3: &str = "incorrect_integer_arithmetic_3";

use fuzz_instructions::incorrect_integer_arithmetic_3_fuzz_instructions::FuzzInstruction as fuzz_instruction_incorrect_integer_arithmetic_3;

pub type FuzzInstruction = fuzz_instruction_incorrect_integer_arithmetic_3;

struct MyFuzzData;

impl FuzzDataBuilder<FuzzInstruction> for MyFuzzData {
    fn pre_ixs(u: &mut arbitrary::Unstructured) -> arbitrary::Result<Vec<FuzzInstruction>> {
        let init_ix = FuzzInstruction::InitVesting(InitVesting::arbitrary(u)?);

        Ok(vec![init_ix])
    }
}

fn fuzz_iteration<T: FuzzTestExecutor<U> + std::fmt::Display, U>(fuzz_data: FuzzData<T, U>) {
    let fuzzing_program_incorrect_integer_arithmetic_3 = FuzzingProgram::new(
        PROGRAM_NAME_INCORRECT_INTEGER_ARITHMETIC_3,
        &PROGRAM_ID_INCORRECT_INTEGER_ARITHMETIC_3,
        processor!(convert_entry!(entry_incorrect_integer_arithmetic_3)),
    );

    let mut client =
        ProgramTestClientBlocking::new(&[fuzzing_program_incorrect_integer_arithmetic_3], &[])
            .unwrap();

    let _ = fuzz_data.run_with_runtime(PROGRAM_ID_INCORRECT_INTEGER_ARITHMETIC_3, &mut client);
}

fn main() {
    loop {
        fuzz_trident ! (fuzz_ix : FuzzInstruction , | fuzz_data : MyFuzzData | { fuzz_iteration (fuzz_data) ; });
    }
}
