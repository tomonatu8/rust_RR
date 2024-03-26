use rand::Rng;

fn envy_cycle_algorithm(utilities: &Vec<Vec<i32>>) -> Vec<Vec<usize>> {
    let m = utilities[0].len();
    let mut allocation: Vec<Vec<usize>> = vec![vec![]; 2];
    let mut unallocated_items: Vec<usize> = (0..m).collect();

    while !unallocated_items.is_empty() {
        println!("{:?}", allocation);
        let unenvied_agent = find_unenvied_agent(&mut allocation, utilities);
        let item = unallocated_items.remove(0);
        allocation[unenvied_agent].push(item);
    }

    allocation
}

fn find_unenvied_agent(allocation: &mut Vec<Vec<usize>>, utilities: &Vec<Vec<i32>>) -> usize {
    if !is_envious(0, 1, allocation, utilities) && is_envious(1, 0, allocation, utilities) {
        1
    } else if is_envious(0, 1, allocation, utilities) && !is_envious(1, 0, allocation, utilities) {
        0
    } else if is_envious(0, 1, allocation, utilities) && is_envious(1, 0, allocation, utilities)  {
        // サイクルが存在する場合、アイテムを交換
        allocation.swap(0, 1);
        0
    } else {
        0
    }
}

fn is_envious(agent1: usize, agent2: usize, allocation: &Vec<Vec<usize>>, utilities: &Vec<Vec<i32>>) -> bool {
    let util1 = calculate_utility(&allocation[agent1], &utilities[agent1]);
    let util2 = calculate_utility(&allocation[agent2], &utilities[agent1]);
    util1 < util2
}

fn calculate_utility(bundle: &Vec<usize>, utilities: &Vec<i32>) -> i32 {
    let mut util = 0;
    for &i in bundle {
        util += utilities[i];
    }
    util
}


fn main() {
    let utilities = vec![
        vec![2, 2, 3, 4],
        vec![1, 2, 3, 4],
    ];

    let allocation = envy_cycle_algorithm(&utilities);
    println!("Final allocation: {:?}", allocation);
}