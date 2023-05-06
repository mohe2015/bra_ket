use bra_ket::*;
use log::info;
fn main() {
    let number_of_qubits = 28;

    let mut program = Program::new();

    for i in 0..number_of_qubits {
        program.h(i);
    }

    program.draw();

    let mut state_vector = StateVector::new(number_of_qubits);

    program.run(&mut state_vector);

    info!("{}", state_vector);
}