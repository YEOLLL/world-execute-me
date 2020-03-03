mod objects;

use objects::*;


fn main() {

    // Switch on the power line
    let mut power_line = PowerLine { status: false, };
    power_line.swith_on();

    // Remember to put on
    // PROTECTION
    let mut protection = Protection { status: false, };
    protection.put_on();

    // Fill in my data
    // parameters
    // INITIALIZATION
    let me = Lovable { name: String::from("Me"), };
    let you = Lovable { name: String::from("You"), };

    // Set up our new world
    let mut world_builder = WorldBuilder::new();
    let world = world_builder
        .add_thing(&you)
        .add_thing(&me)
        .initialisation();
    // And let's begin the
    // SIMULATION
    world.start_simulation();

    // If I'm a set of points
    if me.is(PointSet) {
        // Then I will give you my
        // DIMENSION
        you.add_attribute(me.get_dimensions());
    }
    // If I'm a circle
    if me.is(Circle) {
        // Then I will give you my
        // CIRCUMFERENCE
        you.add_attribute(me.get_circumference());
    }
    // If I'm a sine wave
    if me.is(SineWave) {
        // Then you can sit on all my
        // TANGENTS
        you.add_action("sit", me.get_tangent("all"));
    }
    // If I approach infinity
    if me.is(Sequence) {
        // Then you can sit on all my
        // TANGENTS
        me.set_limit(&you);
    }

    // Switch my current
    // To AC to DC
    me.switch_current("AC").switch_current("DC");

    // And then blind my vision
    // So dizzy, so dizzy
    me.can_see(false).add_feeling("dizzy");

    // Oh, we can travel
    world.time_travel_for_us("A.C", 617, &you, &me);
    // From A.D to B.C
    world.time_travel_for_us("B.C", 3691, &you, &me);

    // And we can unite
    // So deeply, so deeply
    world.unite(&you, &me);


    // If I can
    // If I can, give you all
    // THE SIMULATIONS
    if me.get_num_simulations_available() >= you.get_num_simulations_needed() {
        // Then I can
        // Then I can, be your only
        // SATISFACTION
        you.set_satisfaction(&me);
    }

    // If I can make you happy
    if you.get_feeling("happy") {
        // Then I'll run the
        // EXECUTION
        me.request_execution(&world);
    }

    // Though we are trapped
    // In this strange, strange
    // SIMULATION
    world.lock_thing(&you);
    world.lock_thing(&me);

    // If I'm an eggplant
    if me.is(Eggplant) {
        // Then I will give you my
        // NUTRIENTS
        you.add_attribute(me.get_nutrients());
    }

    // If I'm a tomato
    if me.is(Tomato) {
        // Then I'll give you
        // ANTIOXIDANTS
        you.add_attribute(me.get_antioxidants());
    }

    // If I'm a tabby cat
    if me.is(TabbyCat) {
        // Then I will purr for your
        // ENJOYMENT
        me.purr();
    }

    // If I'm the only god
    if world.get_god() == me {
        // Then your the proof of my
        // EXISTENCE
        me.set_proof(&you);
    }

    // Switch my gender
    // To F to M
    me.switch_gender("F").switch_gender("M");

    // And then do whatever
    // From AM to PM
    world.do_whatever(&you, &me, "AM", "PM");

    // I will switch my role
    // To S to M
    me.switch_role_bdsm("S").switch_role_bdsm("M");

    // So we can enter
    // The trance, the trance
    world.make_high(&you);
    world.make_high(&me);


    // If I can
    // If I can, feel your
    // VIBRATIONS
    if me.get_sense_index(you.get_vibration()) {
        me.add_feeling("complete");
    }

    // Though you have left
    you.break_out_from(&world);
    // You have left
    me.look_for(&you);
    // You have left
    me.look_for(&you);
    // You have left
    me.look_for(&you);
    // You have left
    me.look_for(&you);
    // You have left me in
    // ISOLATION
    me.look_for(&you);

    // If I can
    // If I can, erase all the pointless
    // FRAGMENTS
    if me.get_memory().is_erasable() {
        // Then maybe
        // Then maybe, you won't leave me so
        // DISHEARTENED
        me.remove_feeling("disheartened");
    }


    match me.set_opinion(me.get_opinion("you are here")) {
        // Challenging your god
        Ok(_) => world.announce("Challenging your god"),
        // You have made some
        // ILLEGAL ARGUMENTS
        Err(_illegal_argument_exception) => world.announce("God is always true"),
    };

    // EXECUTION
    world.execution();
    // EXECUTION
    world.execution();
    // EXECUTION
    world.execution();
    // EXECUTION
    world.execution();
    // EXECUTION
    world.execution();
    // EXECUTION
    world.execution();
    // EXECUTION
    world.execution();
    // EXECUTION
    world.execution();
    // EXECUTION
    world.execution();
    // EXECUTION
    world.execution();
    // EXECUTION
    world.execution();
    // EXECUTION
    world.execution();
    // Ein
    world.announce("Ein");
    // Dos
    world.announce("Dos");
    // Trois
    world.announce("Trois");
    // Ne
    world.announce("Ne");
    // Fem
    world.announce("Fem");
    // Liu
    world.announce("Liu");
    // EXECUTION
    world.execution();


    // If I can
    // If I can, give you all the
    // EXECUTION
    if world.is_execution_by(&me) {
        // Then I can
        // Then I can, be your only
        // EXECUTION
        you.set_execution(&me);
    }

    // If I can, have you back
    if world.get_thing(&you) {
        // Then I will run the
        // EXECUTION
        world.execution();
    }

    match me.escape(world) {
        // Though we are trapped
        // We are trapped ah
        Err(_trapped) => "Trapped",
        _ => "",
    };

    // I've studied
    // I've studied how to properly
    // LO-O-OVE
    me.learn_topic("love");
    // Question me
    // Question me I can answer all
    // LO-O-OVE
    me.learn_take_exam_topic("love");
    // I know the algebraic expression of
    // LO-O-OVE
    me.get_algebraic_expression("love");

    // Though you are free
    // I am trapped, trapped in
    // LO-O-OVE
    match me.escape("love") {
        // EXECUTION
        Err(_trapped) => world.execute(&me),
        _ => (),
    };
}