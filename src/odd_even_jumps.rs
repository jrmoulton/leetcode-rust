#[derive(Clone, Copy)]
enum IndexType {
    Even,
    Odd,
    Final,
}

// Have noes. Noes in set. Nodes can be joined
// We can have special nodes
// Track final index and see if in same group as

#[derive(Clone, Copy)]
enum HeadStatus {
    Head(usize),
    NotHead(usize),
}
impl HeadStatus {
    fn set_next(&mut self, node: usize) {
        *self = Self::NotHead(node);
    }
    fn incrase_size(&mut self, other: HeadStatus) {
        use HeadStatus::*;
        let Head(self_size) = self else {
            unreachable!();
        };
        let Head(other_size) = other else {
            unreachable!();
        };
        *self = Head(*self_size + other_size);
    }
}

#[derive(Clone, Copy)]
struct DisNode {
    head_status: HeadStatus,
    index: usize,
    index_type: IndexType,
    val: i32,
}
impl PartialEq for DisNode {
    fn eq(&self, other: &Self) -> bool {
        match self.head_status {
            HeadStatus::Head(_) => match other.head_status {
                HeadStatus::Head(_) => false,
                HeadStatus::NotHead(next) => self.index == next,
            },
            HeadStatus::NotHead(next) => match other.head_status {
                HeadStatus::Head(_) => next == other.index,
                HeadStatus::NotHead(_) => false,
            },
        }
    }
}
impl DisNode {
    fn new(index: usize, val: i32) -> Self {
        let index_type = match index % 2 {
            0 => IndexType::Even,
            1 => IndexType::Odd,
            _ => unreachable!(),
        };
        Self {
            head_status: HeadStatus::Head(1),
            index,
            index_type,
            val,
        }
    }
}

trait DisjointSet {
    type Node;
    fn find(&mut self, node: usize) -> Self::Node;
    fn union(&mut self, root1: usize, root2: usize);
}

struct EvenOddSet {
    arr: Vec<DisNode>,
}
impl DisjointSet for EvenOddSet {
    type Node = DisNode;
    fn find(&mut self, node: usize) -> Self::Node {
        use HeadStatus::*;
        match self.arr.get(node) {
            Some(dis_node) => match dis_node.head_status {
                Head(_) => *dis_node,
                NotHead(_) => {
                    let root = self.find(node);
                    self.arr[node].head_status.set_next(root.index);
                    root
                }
            },
            None => unimplemented!(),
        }
    }

    fn union(&mut self, root1: usize, root2: usize) {
        if !self.in_same_set(root1, root2) {
            let root1_root = self.find(root1);
            let root2_root = self.find(root2);
            if let IndexType::Final = self.arr[root1_root.index].index_type {
                self.arr[root1_root.index]
                    .head_status
                    .incrase_size(root2_root.head_status);
                self.arr[root2_root.index]
                    .head_status
                    .set_next(root1_root.index);
            } else {
                self.arr[root2_root.index]
                    .head_status
                    .incrase_size(root1_root.head_status);
                self.arr[root1_root.index]
                    .head_status
                    .set_next(root2_root.index);
            }
        }
    }
}
impl EvenOddSet {
    fn in_same_set(&mut self, root1: usize, root2: usize) -> bool {
        self.find(root1) == self.find(root2)
    }
}

pub fn odd_even_jumps(arr: Vec<i32>) -> i32 {
    let mut nodes: Vec<DisNode> = arr
        .iter()
        .enumerate()
        .map(|(idx, val)| DisNode::new(idx, *val))
        .collect();
    let nodes_len = nodes.len();
    nodes[nodes_len - 1].index_type = IndexType::Final;
    nodes.sort_by(|a, b| a.val.cmp(&b.val));
    for (idx, mut node) in nodes.iter_mut().enumerate() {
        node.index = idx;
    }
    let mut set = EvenOddSet { arr: nodes };
    let mut total_good = 0;
    for start_idx in 0..set.arr.len() {
        let found = play(start_idx, &mut set);
        if let IndexType::Final = set.arr[found].index_type {
            total_good += 1;
        }
    }
    total_good
}

fn play(idx: usize, set: &mut EvenOddSet) -> usize {
    use IndexType::*;
    let play_result = match set.arr[idx].index_type {
        Even => {
            if idx >= set.arr.len() {
                return idx;
            }
            play(idx + 1, set)
        }
        Odd => {
            if idx == 0 {
                return idx;
            }
            play(idx - 1, set)
        }
        Final => return idx,
    };
    set.union(idx, play_result);
    idx
}
// assert_eq!(3, odd_even_jumps::odd_even_jumps(vec![2, 3, 1, 1, 4]));
// (1, Odd), (1, Even), (2, Even), (3, Odd), (4, Even)
