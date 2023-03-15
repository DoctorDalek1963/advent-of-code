pub mod bin;
mod parse;

pub use self::parse::parse_blueprint_list;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ResourceType {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

pub type Cost = Vec<(u16, ResourceType)>;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct ResourceCollection {
    ore_count: u16,
    clay_count: u16,
    obsidian_count: u16,
    geode_count: u16,

    ore_robot_count: u16,
    clay_robot_count: u16,
    obsidian_robot_count: u16,
    geode_robot_count: u16,
}

impl Default for ResourceCollection {
    fn default() -> Self {
        Self {
            ore_count: 0,
            clay_count: 0,
            obsidian_count: 0,
            geode_count: 0,

            ore_robot_count: 1,
            clay_robot_count: 0,
            obsidian_robot_count: 0,
            geode_robot_count: 0,
        }
    }
}

impl ResourceCollection {
    fn increment_resource_counts(&mut self) {
        self.ore_count += self.ore_robot_count;
        self.clay_count += self.clay_robot_count;
        self.obsidian_count += self.obsidian_robot_count;
        self.geode_count += self.geode_robot_count;
    }

    fn cost_satisfied(&self, cost: &Cost) -> bool {
        use ResourceType::*;

        cost.iter().all(|&(count, resource_type)| match resource_type {
            Ore => self.ore_count,
            Clay => self.clay_count,
            Obsidian => self.obsidian_count,
            Geode => self.geode_count,
        } >= count)
    }

    fn can_build_robot(&self, blueprint: &Blueprint, robot_type: ResourceType) -> bool {
        use ResourceType::*;

        self.cost_satisfied(match robot_type {
            Ore => &blueprint.ore_robot_cost,
            Clay => &blueprint.clay_robot_cost,
            Obsidian => &blueprint.obsidian_robot_cost,
            Geode => &blueprint.geode_robot_cost,
        })
    }

    fn after_cost(&self, cost: &Cost) -> Self {
        use ResourceType::*;

        let mut new = self.clone();
        for &(count, resource_type) in cost {
            match resource_type {
                Ore => new.ore_count -= count,
                Clay => new.clay_count -= count,
                Obsidian => new.obsidian_count -= count,
                Geode => new.geode_count -= count,
            };
        }

        new
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Blueprint {
    id_number: u32,
    ore_robot_cost: Cost,
    clay_robot_cost: Cost,
    obsidian_robot_cost: Cost,
    geode_robot_cost: Cost,
}

impl Blueprint {
    pub fn quality_level(&self) -> u32 {
        self.id_number * self.max_geodes_opened_in_24_minutes() as u32
    }

    fn max_geodes_opened_in_24_minutes(&self) -> u16 {
        use ResourceType::*;

        let costs: Vec<(u16, ResourceType)> = [
            &self.ore_robot_cost,
            &self.clay_robot_cost,
            &self.obsidian_robot_cost,
            &self.geode_robot_cost,
        ]
        .iter()
        .copied()
        .flatten()
        .copied()
        .collect();

        // (ore, clay, obsidian)
        let max_resource_requirements = (
            costs
                .iter()
                .filter_map(|&(n, t)| if t == Ore { Some(n) } else { None })
                .max()
                .unwrap(),
            costs
                .iter()
                .filter_map(|&(n, t)| if t == Clay { Some(n) } else { None })
                .max()
                .unwrap(),
            costs
                .iter()
                .filter_map(|&(n, t)| if t == Obsidian { Some(n) } else { None })
                .max()
                .unwrap(),
        );

        let v = step_blueprint_one_minute(
            self,
            ResourceCollection::default(),
            max_resource_requirements,
            24,
        );

        dbg!(v.len());
        dbg!(self, v.iter().max_by_key(|coll| coll.geode_count).unwrap());
        v.iter().map(|coll| coll.geode_count).max().unwrap()
    }
}

fn step_blueprint_one_minute(
    blueprint: &Blueprint,
    resources: ResourceCollection,
    max_resource_requirements: (u16, u16, u16), // (ore, clay, obsidian)
    time_left: u8,
) -> Vec<ResourceCollection> {
    use ResourceType::*;

    if time_left == 0 {
        return vec![resources];
    }

    let mut branches: Vec<ResourceCollection> = vec![{
        let mut new_resources = resources.clone();
        new_resources.increment_resource_counts();
        new_resources
    }];

    // Choose robots to build. We have to check to make sure we don't build more robots than we
    // would ever need on a single turn, using `max_resource_requirements`
    if resources.can_build_robot(blueprint, Geode) {
        let mut new_resources = resources.after_cost(&blueprint.geode_robot_cost);
        new_resources.increment_resource_counts();
        new_resources.geode_robot_count += 1;
        branches.push(new_resources);
    }

    if resources.can_build_robot(blueprint, Obsidian)
        && resources.obsidian_robot_count < max_resource_requirements.2
        && resources.obsidian_robot_count * time_left as u16 + resources.obsidian_count
            < time_left as u16 * max_resource_requirements.2
    {
        let mut new_resources = resources.after_cost(&blueprint.obsidian_robot_cost);
        new_resources.increment_resource_counts();
        new_resources.obsidian_robot_count += 1;
        branches.push(new_resources);
    }

    if resources.can_build_robot(blueprint, Clay)
        && resources.clay_robot_count < max_resource_requirements.1
        && resources.clay_robot_count * time_left as u16 + resources.clay_count
            < time_left as u16 * max_resource_requirements.1
    {
        let mut new_resources = resources.after_cost(&blueprint.clay_robot_cost);
        new_resources.increment_resource_counts();
        new_resources.clay_robot_count += 1;
        branches.push(new_resources);
    }

    if resources.can_build_robot(blueprint, Ore)
        && resources.ore_robot_count < max_resource_requirements.0
        && resources.ore_robot_count * time_left as u16 + resources.ore_count
            < time_left as u16 * max_resource_requirements.0
    {
        let mut new_resources = resources.after_cost(&blueprint.ore_robot_cost);
        new_resources.increment_resource_counts();
        new_resources.ore_robot_count += 1;
        branches.push(new_resources);
    }

    //{
    //let (robot_type, mut resources_after_robot): (Option<ResourceType>, ResourceCollection) =
    //if resources.can_build_robot(blueprint, Obsidian)
    //&& resources.obsidian_robot_count < max_resource_requirements.2
    //{
    //(
    //Some(Obsidian),
    //resources.after_cost(&blueprint.obsidian_robot_cost),
    //)
    //} else if resources.can_build_robot(blueprint, Clay)
    //&& resources.clay_robot_count < max_resource_requirements.1
    //{
    //(Some(Clay), resources.after_cost(&blueprint.clay_robot_cost))
    //} else if resources.can_build_robot(blueprint, Ore)
    //&& resources.ore_robot_count < max_resource_requirements.0
    //{
    //(Some(Ore), resources.after_cost(&blueprint.ore_robot_cost))
    //} else {
    //(None, resources)
    //};
    //}

    //// Collect resources
    //resources_after_robot.increment_resource_counts();

    //// Add newly built robot back
    //if let Some(robot) = robot_type {
    //match robot {
    //Ore => resources_after_robot.ore_robot_count += 1,
    //Clay => resources_after_robot.clay_robot_count += 1,
    //Obsidian => resources_after_robot.obsidian_robot_count += 1,
    //Geode => resources_after_robot.geode_robot_count += 1,
    //}
    //}

    //[
    //// Case where we build a robot
    //Some(step_blueprint_one_minute(
    //blueprint,
    //resources_after_robot,
    //max_resource_requirements,
    //time_left - 1,
    //)),
    //// Case where we don't buy a robot
    //if robot_type.is_none() {
    //// If robot_type is None, then we couldn't buy a robot anyway, so there's no need to
    //// repeat that case
    //None
    //} else {
    //let mut new_resources = resources;
    //new_resources.increment_resource_counts();
    //Some(step_blueprint_one_minute(
    //blueprint,
    //new_resources,
    //max_resource_requirements,
    //time_left - 1,
    //))
    //},
    //]
    //.into_iter()
    //.filter_map(|v| v)
    //.flatten()
    //.collect();

    branches
        .into_iter()
        .map(|coll| {
            step_blueprint_one_minute(blueprint, coll, max_resource_requirements, time_left - 1)
        })
        .flatten()
        .collect()
}

#[cfg(test)]
pub const TEST_INPUT: &str = "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.
";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore = "I never finished this day"]
    fn quality_level_test() {
        let blueprints = parse_blueprint_list(TEST_INPUT).unwrap().1;

        assert_eq!(blueprints[0].quality_level(), 9);
        assert_eq!(blueprints[1].quality_level(), 24);
    }

    #[test]
    fn resource_collection_can_build_test() {
        use ResourceType::*;

        let blueprint = Blueprint {
            id_number: 1,
            ore_robot_cost: vec![(4, Ore)],
            clay_robot_cost: vec![(2, Ore)],
            obsidian_robot_cost: vec![(3, Ore), (14, Clay)],
            geode_robot_cost: vec![(2, Ore), (7, Obsidian)],
        };

        let collection = ResourceCollection {
            ore_count: 12,
            clay_count: 6,
            obsidian_count: 8,
            geode_count: 1,
            ..Default::default()
        };

        assert!(collection.can_build_robot(&blueprint, Ore));
        assert!(collection.can_build_robot(&blueprint, Clay));
        assert!(!collection.can_build_robot(&blueprint, Obsidian));
        assert!(collection.can_build_robot(&blueprint, Geode));
    }
}
