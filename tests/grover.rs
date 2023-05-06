use bra_ket::*;
use log::{info, LevelFilter};

#[test]
fn grover_two_qubit() {
    env_logger::builder().filter_level(LevelFilter::Info).parse_default_env().init();

    let mut program = Program::new();

    program.h(0);
    program.h(1);

    program.cz(0, 1);

    program.h(0);
    program.h(1);

    program.z(0);
    program.z(1);
    program.cz(0, 1);
    program.h(0);
    program.h(1);

    program.measure_all();

    let mut state = StateVector::new(2);
    program.run(&mut state);
    program.draw();

    info!("{:?}", state)

}



