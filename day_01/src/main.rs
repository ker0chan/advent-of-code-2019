fn main() {
    // INPUT
    let masses = [74364, 146203, 128470, 91616, 115655, 134147, 53470,
    126471, 70040, 88750, 142353, 143329, 86356, 118399, 97959, 148345,
    117705, 87624, 63862, 71962, 106974, 66255, 119735, 78726, 93698,
    148680, 144638, 83341, 149571, 147196, 54526, 91775, 63153, 143441,
    71134, 114131, 120931, 109833, 106073, 64547, 126938, 52877, 89945,
    59466, 79660, 147815, 55381, 100052, 78824, 121844, 104155, 117313,
    69305, 144645, 81350, 123512, 81467, 120836, 118612, 143999, 90792,
    71054, 138942, 56481, 71850, 85266, 77437, 86530, 147311, 133699,
    126684, 58708, 149482, 104101, 67985, 81648, 95290, 77155, 76578,
    116025, 83980, 59517, 62078, 89003, 126205, 122542, 116388, 144040,
    102560, 77098, 127534, 56415, 85703, 85580, 86787, 72029, 82533,
    132187, 70849, 98839];

    // PART 1
    //Folding is similar to reduce in JS
    // m is a borrow, needs dereferencing! (*m)
    let sum = masses.iter().fold(0, |acc, m| acc + compute_fuel(*m));

    println!("Part 1 answer: {}", sum);

    // PART 2
    let sum = masses.iter().fold(0, |acc, m| acc + compute_fuel_and_fuel_for_fuel(*m, 0));

    //Use the example to make sure we didn't mess up?
    assert_eq!(compute_fuel_and_fuel_for_fuel(1969, 0), 966);

    println!("Part 2 answer: {}", sum);
}

fn compute_fuel(mass: u32) -> u32
{
    /* Fuel required to launch a given module is based on its mass.
     * Specifically, to find the fuel required for a module, take
     * its mass, divide by three, round down, and subtract 2.
     */

    //Sweet, Rust rounds down on integer division
    mass/3 - 2
}

fn compute_fuel_and_fuel_for_fuel(mass: u32, mut total_fuel: u32) -> u32
{
    /* Fuel itself requires fuel just like a module - take its mass,
     * divide by three, round down, and subtract 2. However, that fuel
     * also requires fuel, and that fuel requires fuel, and so on.
     */
    
    //How much fuel does this mass require?
    let fuel_mass = compute_fuel(mass);

    //Keep track of the total amount of fuel
    total_fuel += fuel_mass;

    let mut result = total_fuel;
    //"Any mass that would require negative fuel should instead be treated as if it requires zero fuel"
    // (This is our recursion end condition :3)
    if fuel_mass >= 9 
    {
        //If the required fuel is heavy enough, it requires its own additional amount of fuel. 
        result = compute_fuel_and_fuel_for_fuel(fuel_mass, total_fuel);
    }

    result
}