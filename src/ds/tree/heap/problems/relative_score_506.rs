// use std::cmp::Ordering;
// use std::ops::Index;
//
// #[derive(Debug, Eq, PartialEq, Clone)]
// struct ScoreNode {
//     score: i32,
//     index: usize,
// }
//
// impl ScoreNode {
//     pub fn new(score: i32, index: usize) -> Self {
//         ScoreNode { score, index }
//     }
// }
//
// impl Ord for ScoreNode {
//     fn cmp(&self, other: &Self) -> Ordering {
//         other.score.cmp(&self.score)
//     }
// }
//
// impl PartialOrd for ScoreNode {
//     fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
//         Some(self.cmp(other))
//     }
// }
//
// #[derive(Clone)]
// struct RelativeRank {
//     data: Vec<ScoreNode>,
// }
//
//
// impl RelativeRank {
//     pub fn new() -> Self {
//         RelativeRank {
//             data: Vec::new()
//         }
//     }
//
//     fn parent_index(i: usize) -> usize {
//         (i - 1) / 2
//     }
//
//     fn left_child_index(i: usize) -> usize {
//         i * 2 + 1
//     }
//
//     fn right_child_index(i: usize) -> usize {
//         i * 2 + 2
//     }
//
//     pub fn add(&mut self, score: i32, index: usize) {
//         self.data.push(ScoreNode::new(score, index));
//         let mut current_rank_index = self.data.len() - 1;
//         while current_rank_index > 0 && self.data[current_rank_index].cmp(&self.data[Self::parent_index(current_rank_index)]) == Ordering::Less{
//             self.data.swap(current_rank_index, Self::parent_index(current_rank_index));
//             current_rank_index = Self::parent_index(current_rank_index);
//         }
//     }
//
//     pub fn print(&self) {
//         println!("{:?}", self.data);
//     }
//
//     pub fn remove_ele(&mut self) -> Option<ScoreNode> {
//
//         if self.data.is_empty() {
//             return None
//         }
//
//         let max_value = self.data[0].clone();
//
//         // pop the last ele and replace with the first one
//         if let Some(last) = self.data.pop() {
//             if !self.data.is_empty() {
//                 self.data[0] = last; // Move the last element to the root
//                 self.bubble_down(0); // Restore the max-heap property
//             }
//         }
//         Some(max_value)
//
//     }
//
//     fn bubble_down(&mut self, index: usize) {
//         let heap_len = self.data.len();
//         let mut current_index = index;
//
//         loop {
//             let mut max_index = current_index;
//             let left_child_index = Self::left_child_index(current_index);
//             let right_child_index = Self::right_child_index(current_index);
//
//             if left_child_index < heap_len && self.data[left_child_index] > self.data[max_index] {
//                 max_index = left_child_index
//             }
//
//             if right_child_index < heap_len && self.data[right_child_index] > self.data[max_index] {
//                 max_index = right_child_index
//             }
//
//             if current_index == max_index {
//                 break;
//             }
//
//             self.data.swap(current_index, max_index);
//             current_index = max_index
//         }
//     }
//
//
//     pub fn get_rank(&mut self) -> Vec<String> {
//         let mut rank_result: Vec<String> = vec!["0".to_string(); self.data.len()];
//         let mut rank = 1;
//
//         while let Some(score_node) = self.remove_ele() {
//             let rank_str = match rank {
//                 1 => "Gold Medal".to_string(),
//                 2 => "Silver Medal".to_string(),
//                 3 => "Bronze Medal".to_string(),
//                 _ => rank.to_string(),
//             };
//
//             rank_result[score_node.index] = rank_str;
//             rank += 1;
//         }
//
//         rank_result
//     }
// }

use std::collections::BinaryHeap;


pub fn find_relative_ranks(score: Vec<i32>) -> Vec<String> {
    let mut heap = BinaryHeap::from_iter(score.iter().cloned().enumerate().map(|(a, b)| (b, a)));
    let ranks = [String::from("Gold Medal"), String::from("Silver Medal"), String::from("Bronze Medal")];
    let mut res = vec![String::new(); score.len()];
    (0..score.len()).for_each(|i| res[heap.pop().unwrap().1] = ranks.get(i).unwrap_or(&(i + 1).to_string()).clone());
    res
}


pub fn relative_score_main() {
    println!("Relative score Leet - 506");

    // let mut relative_score: RelativeRank = RelativeRank::new();
    // let score = vec![10,3,8,9,4];
    // for (i, val) in score.iter().enumerate() {
    //     relative_score.add(*val, i);
    // }
    // println!("{:?}",relative_score.get_rank());
    let mut score = vec![5,4,3,2,1];
    println!("{:?}", find_relative_ranks(score));

}