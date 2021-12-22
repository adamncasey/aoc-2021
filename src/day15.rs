use crate::day9::neighbours;

use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Debug, Clone, Eq, PartialEq)]
struct SearchNode {
    cost: u32,
    pos: (usize, usize),
}

impl Ord for SearchNode {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.pos.cmp(&other.pos))
    }
}
impl PartialOrd for SearchNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn day15(map: &Vec<Vec<u32>>, dest: (usize, usize)) -> u32 {
    let mut heap: BinaryHeap<SearchNode> = BinaryHeap::new();

    dbg!(dest);
    heap.push(SearchNode {
        cost: 0,
        pos: (0, 0),
    });

    let mut dists = vec![u32::MAX; map.len() * map[0].len()];

    while let Some(node) = heap.pop() {
        //dbg!(&node, heap.len());
        if node.pos == dest {
            return node.cost;
        }

        if node.cost > dists[node.pos.1 * map.len() + node.pos.0] {
            // heap contains a better route to this node already
            continue;
        }

        let (x, y) = node.pos;
        for neighbour in neighbours(x, y, dest.0 + 1, dest.1 + 1) {
            let next_cost = map[neighbour.1][neighbour.0] + node.cost;

            if next_cost >= dists[neighbour.1 * map.len() + neighbour.0] {
                // This isn't a better route to neighbour
                continue;
            }

            heap.push(SearchNode {
                cost: next_cost,
                pos: neighbour,
            });
            dists[neighbour.1 * map.len() + neighbour.0] = next_cost;
        }
    }

    panic!("No route");
}

fn expand_grid(map: &Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    let dimensions = map.len();

    let mut expanded = vec![vec![0u32; dimensions * 5]; dimensions * 5];

    for y in 0..dimensions * 5 {
        for x in 0..dimensions * 5 {
            let (xclamp, yclamp) = (x % dimensions, y % dimensions);
            let (xgrid, ygrid) = (x / dimensions, y / dimensions);
            //dbg!((xclamp, yclamp, xgrid, ygrid));

            let new_value = map[yclamp][xclamp] + (xgrid + ygrid) as u32;

            expanded[y][x] = ((new_value - 1) % 9) + 1;
        }
    }

    expanded
}

fn day15_2(map: &Vec<Vec<u32>>, dest: (usize, usize)) -> u32 {
    let expanded = expand_grid(map);

    day15(&expanded, dest)
}

fn read_input<'a>(input: String) -> Vec<Vec<u32>> {
    let input: Vec<Vec<u32>> = input
        .split("\n")
        .map(|x| {
            x.chars()
                .map(|ch| ch.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect();
    input
}

#[test]
fn day15_example() {
    let input = "1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581";

    let input = read_input(input.to_string());

    assert_eq!(day15(&input, (9, 9)), 40);
}

#[test]
fn day15_actual() {
    let input = std::fs::read_to_string("./input/day15.txt").unwrap();

    let input = read_input(input.to_string());

    assert_eq!(day15(&input, (99, 99)), 373);
}

#[test]
fn day15_2_example() {
    let input = "1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581";

    let input = read_input(input.to_string());

    assert_eq!(day15_2(&input, (49, 49)), 315);
}

#[test]
fn day15_2_actual() {
    let input = std::fs::read_to_string("./input/day15.txt").unwrap();

    let input = read_input(input.to_string());

    assert_eq!(day15_2(&input, (499, 499)), 2868);
}

#[test]
fn day15_2_expand_grid() {
    let input = "1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581";

    let input = read_input(input.to_string());

    let expanded = "11637517422274862853338597396444961841755517295286
13813736722492484783351359589446246169155735727126
21365113283247622439435873354154698446526571955763
36949315694715142671582625378269373648937148475914
74634171118574528222968563933317967414442817852555
13191281372421239248353234135946434524615754563572
13599124212461123532357223464346833457545794456865
31254216394236532741534764385264587549637569865174
12931385212314249632342535174345364628545647573965
23119445813422155692453326671356443778246755488935
22748628533385973964449618417555172952866628316397
24924847833513595894462461691557357271266846838237
32476224394358733541546984465265719557637682166874
47151426715826253782693736489371484759148259586125
85745282229685639333179674144428178525553928963666
24212392483532341359464345246157545635726865674683
24611235323572234643468334575457944568656815567976
42365327415347643852645875496375698651748671976285
23142496323425351743453646285456475739656758684176
34221556924533266713564437782467554889357866599146
33859739644496184175551729528666283163977739427418
35135958944624616915573572712668468382377957949348
43587335415469844652657195576376821668748793277985
58262537826937364893714847591482595861259361697236
96856393331796741444281785255539289636664139174777
35323413594643452461575456357268656746837976785794
35722346434683345754579445686568155679767926678187
53476438526458754963756986517486719762859782187396
34253517434536462854564757396567586841767869795287
45332667135644377824675548893578665991468977611257
44961841755517295286662831639777394274188841538529
46246169155735727126684683823779579493488168151459
54698446526571955763768216687487932779859814388196
69373648937148475914825958612593616972361472718347
17967414442817852555392896366641391747775241285888
46434524615754563572686567468379767857948187896815
46833457545794456865681556797679266781878137789298
64587549637569865174867197628597821873961893298417
45364628545647573965675868417678697952878971816398
56443778246755488935786659914689776112579188722368
55172952866628316397773942741888415385299952649631
57357271266846838237795794934881681514599279262561
65719557637682166874879327798598143881961925499217
71484759148259586125936169723614727183472583829458
28178525553928963666413917477752412858886352396999
57545635726865674683797678579481878968159298917926
57944568656815567976792667818781377892989248891319
75698651748671976285978218739618932984172914319528
56475739656758684176786979528789718163989182927419
67554889357866599146897761125791887223681299833479";

    let expanded = read_input(expanded.to_string());
    let calc_expanded = expand_grid(&input);

    assert_eq!(calc_expanded[0], expanded[0]);

    assert_eq!(expand_grid(&input), expanded);
}
