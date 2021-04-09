#[derive(Debug, Clone)]
pub struct Token{
	pub value: String,
	pub synonyms: Vec<String>
}

pub struct Node {
    pub chars: [u32; 37], // array to hold transitions for all alphanumeric characters
    val: Option<u32>, // value
}

impl Node{
	pub fn new() -> Node {
		Node{chars: [u32::MAX;37], val: None}// if an element in the transition array equals max u32 value, there is no transition for the character with the equivalent index
	}
}

pub struct Trie {
    pub nodes: Vec<Node>,
	pub synonym_dict: Vec<Vec<String>>
}
fn char_2_index(c: u8) -> usize{ // map characters of input string to an usize in range 0..37
	let index: usize = match c{
		0..=31 => panic!("{} is a non lowercase alphanumeric characters", c as char),
		32 => 0,
		33..=47 => panic!("{} is a non lowercase alphanumeric characters", c as char),
		48..=57 => c as usize - 47,
		58..=96 => panic!("{} is a non lowercase alphanumeric characters", c as char),
		97..=122 => c as usize - 86,
		123..=255 => panic!("{} is a non lowercase alphanumeric characters", c as char),
	};
	return index;
}



impl Trie {
    pub fn new() -> Trie {
        Trie { nodes: vec![Node::new()], synonym_dict: Vec::new() }
    }

	fn transition(&self, node_pos: usize, c: &u8) -> usize{// returns index for the node to walk to according to input character

		return self.nodes[node_pos].chars[char_2_index(*c)] as usize;
	}
    pub fn insert(&mut self, string: &str, val: Option<u32>) { //insert 
        let mut node_pos: usize = 0;
		let mut temp_node_pos: usize;
		for c in string.as_bytes(){
			temp_node_pos = self.transition(node_pos, c);
			if temp_node_pos == 4294967295{
				self.nodes.push(Node::new());
				self.nodes[node_pos].chars[char_2_index(*c)] = (self.nodes.len() - 1) as u32;
				node_pos = self.nodes.len() - 1;
			}
			else {node_pos = temp_node_pos;}
		}
		if val == Some(4294967295){
			// println!("{:?}", string);
			self.nodes[node_pos].val = Some(1);
		}
		else{
			self.nodes[node_pos].val = val;
		}
    }
    pub fn insert_synonym(&mut self, string: &str, synonyms: Vec<String>) { //insert a string with a vector of its synonyms as strings
        let mut node_pos: usize = 0;
		let mut temp_node_pos: usize;
		for c in string.as_bytes(){
			temp_node_pos = self.transition(node_pos, c);
			if temp_node_pos == 4294967295{
				self.nodes.push(Node::new());
				self.nodes[node_pos].chars[char_2_index(*c)] = (self.nodes.len() - 1) as u32;
				node_pos = self.nodes.len() - 1;
			}
			else {node_pos = temp_node_pos;}
		}
		self.synonym_dict.push(synonyms);
		self.nodes[node_pos].val = Some(4000000000+self.synonym_dict.len() as u32 - 1);
    }
	pub fn search(&self, string: &str) -> Option<u32> {
		let mut node_pos: usize = 0;
		for c in string.as_bytes(){
			node_pos = self.transition(node_pos, c);
			if node_pos == 4294967295 {return None;}
		}
		return self.nodes[node_pos].val;
	}
	
	pub fn longest_common_prefix_search(&self, string: &str) -> Option<(u32, usize)>{ //returns longest key from the beginning of the input string, returns a prefix for a key in the trie if no complete key can be found
		let mut node_pos: usize = 0;
		let mut value: Option<u32> = None;
		let mut offset: usize = 0;
		for i in 0..string.len(){
			node_pos = self.transition(node_pos, &string.as_bytes()[i]);
			// println!("{}, {:?}", node_pos, self.nodes[node_pos].val);
			if node_pos != 4294967295 && i < string.len()-1{
				// if self.nodes[node_pos].val != None{
					// value = self.nodes[node_pos].val;
				// }				

				
					if string.as_bytes()[i+1] == ' ' as u8{
						value = self.nodes[node_pos].val;
						offset = i;
					}
				
			}
			if i == string.len()-1 && node_pos != 4294967295{
				return Some((self.nodes[node_pos].val.unwrap_or(1), i));
			}
			if node_pos == 4294967295 {
				match value {
					None => {
						return None;
					},
					Some(val) => {
						return Some((val, offset));
					}
				}
			}
		}
		return Some((value.unwrap_or(0), offset));
	}
	//return set of common prefixes for input string
	pub fn common_prefix_search(&self, string: &str) -> Vec<(u32, usize)> {
		let mut node_pos: usize = 0;
		// let mut value: Option<u32> = None;
		// let mut offset: usize = 0;	
		let mut results: Vec<(u32, usize)> = Vec::new();
		for i in 0..string.len(){
			node_pos = self.transition(node_pos, &string.as_bytes()[i]);
			if node_pos != 4294967295 && self.nodes[node_pos].val != None{
				if i == string.len()-1{
					results.push((self.nodes[node_pos].val.unwrap(), i));
				}
				else {
					if string.as_bytes()[i+1] == ' ' as u8{
						results.push((self.nodes[node_pos].val.unwrap(), i));
					}
				}
			}
			if node_pos == 4294967295 {
				return results;
			}
		}
		return results;
	}
	/// function that gets the highest scoring (sum of values) combination of tokens
	pub fn get_all_tokens(&self, input: &str)-> Vec<Token>{
		if !input.contains(' '){return vec![Token {value: input.to_string(), synonyms: match self.search(input){
			None => Vec::new(),
			Some(val) => {
				if val >= 4000000000{
					self.synonym_dict[val as usize - 4000000000].clone()
				}
				else{Vec::new()}
			}
		}
		}
		]}
		let mut candidates: Vec<(String, usize, usize, u32)> = Vec::new();

		let mut offsets: Vec<usize> = input.match_indices(' ').map(|s| s.0+1).collect();
		offsets.push(0);
		offsets.rotate_right(1);
		for offset in offsets{
			let word  = self.longest_common_prefix_search(&input[offset..]).unwrap_or((0, 0));
			let candidate = (input[offset..=offset+word.1].to_string(), offset, offset+word.1,word.0);
			if candidate.2 > candidates.last().unwrap_or(&("".to_string(), 0, 0, 0)).2 && word.1!=0{
				candidates.push(candidate);
			}
		}
		// println!("{:?}", candidates);
		let mut offset: usize = 0;
		let mut to_add: Vec<(String, usize, usize, u32)> = Vec::new();
		if candidates.len() == 0 {
			return self.split_candidate((input.to_string(), 0, 0 ,0));
		}
		if candidates.len() == 1{
			for candidate in &candidates{
				if &input[offset..candidate.1] != "" && &input[offset..candidate.1] != " "{
					if offset == 0{
						to_add.push((input[offset..candidate.1-1].to_string(), offset, candidate.1-2, 0))
					}
					else{
						to_add.push((input[offset+1..candidate.1-1].to_string(), offset+1, candidate.1-2, 0))
					}
				}
				offset = candidate.2+1;
			}
			if &input[offset..input.len()] != "" && &input[offset..input.len()] != " "{
				to_add.push((input[offset+1..input.len()].to_string(), offset+1, input.len(), 0))
			}
			// println!("{:?}", to_add);
			candidates.append(&mut to_add);
			candidates.sort_by_key(|a| a.1);
			let mut result: Vec<Token> = Vec::new();
			for candidate in candidates{
				if candidate.3 == 0{
					let mut splits = self.split_candidate(candidate);
					result.append(&mut splits);
				}
				else{
					if candidate.3 >= 4000000000{
						
						result.push(Token{value: candidate.0, synonyms: self.synonym_dict[candidate.3 as usize -4000000000].clone()})
					}
					else{
						result.push(Token{value: candidate.0, synonyms: Vec::new()});
					}
				}
			}
			return result;
			
		}
		if candidates.len() == 2{
			if candidates[0].3 >= candidates[1].3 && candidates[0].2 >= candidates[1].1{
				candidates.remove(1);
			}
			else{
				if candidates[0].3 < candidates[1].3 && candidates[0].2 >= candidates[1].1{
					candidates.remove(0);
				}
			}
		}
		if candidates.len()>=3{
			// println!("candidates: {:?}", candidates);
			let mut tokens_to_remove: Vec<usize> = Vec::new();
			let windows_iter = candidates.windows(3);
			
			for (i, window) in windows_iter.enumerate(){
				if window[1].3 < window[0].3 + window[2].3 && window[0].2 >= window[1].1 && window[1].2 >= window[2].1 && !tokens_to_remove.contains(&i){
					// println!("{} {}", i, tokens_to_remove.contains(&i));
					tokens_to_remove.push(i+1);
				}
				else{
					if window[0].3 >= window[1].3 && window[0].2 >= window[1].1 && !tokens_to_remove.contains(&i){
						tokens_to_remove.push(i+1);
					}
					if window[1].3 > window[0].3 && window[0].2 >= window[1].1{
						tokens_to_remove.push(i);
					}
					if window[1].3 >= window[2].3 && window[1].2 >= window[2].1 && !tokens_to_remove.contains(&(i+1)){
						tokens_to_remove.push(i+2);
					}
				}
			}
			tokens_to_remove.dedup();
			// println!("{:?}", tokens_to_remove);
			for index in tokens_to_remove.iter().rev(){
				candidates.remove(*index);
			}
			if candidates.len()>=2{
				if candidates[candidates.len()-1].1 < candidates[candidates.len()-2].2{
					if candidates[candidates.len()-2].3 >= candidates[candidates.len()-1].3{
						candidates.remove(candidates.len()-1);
					}
					else{
						candidates.remove(candidates.len()-2);
					}
				}
			}
		}
		// println!("candidates: {:?}", candidates);
		for candidate in &candidates{
			// println!("{}, {}", offset, candidate.1);
			if &input[offset..candidate.1] != "" && &input[offset..candidate.1] != " "{
				if offset == 0{
					to_add.push((input[offset..candidate.1].to_string(), offset, candidate.1-2, 0))
				}
				else{
					to_add.push((input[offset..candidate.1].to_string(), offset+1, candidate.1-2, 0))
				}
			}
			offset = candidate.2+1;
		}
		if &input[offset..input.len()] != "" && &input[offset..input.len()] != " "{
			to_add.push((input[offset+1..input.len()].to_string(), offset+1, input.len(), 0))
		}
		// println!("{:?}", to_add);
		candidates.append(&mut to_add);
		candidates.sort_by_key(|a| a.1);
		let mut result: Vec<Token> = Vec::new();

		for candidate in candidates{
			if candidate.3 == 0{
				let mut splits = self.split_candidate(candidate);
				result.append(&mut splits);
			}
			else{
				if candidate.3 >= 4000000000{
					
					result.push(Token{value: candidate.0, synonyms: self.synonym_dict[candidate.3 as usize -4000000000].clone()})
				}
				else{
					result.push(Token{value: candidate.0, synonyms: Vec::new()});
				}
			}
		}
		return result;
	}
	
	fn split_candidate(&self, candidate: (String, usize, usize, u32)) -> Vec<Token>{//split get_all_token candidates into valid tokens
		
		// let ngo_hem: Vec<String> = vec!["hem".to_string(), "ngo".to_string(), "ngach".to_string()];
						// println!("{}", candidate.0);
		// for i in 0..3{
			// if candidate.0.starts_with(&ngo_hem[i]) && (48..=57).contains(&(candidate.0.as_bytes()[candidate.0.find(' ').unwrap_or(0)+1] as u8)){

				// return vec![Token{value: candidate.0.clone(), synonyms: vec![candidate.0.split_whitespace().last().unwrap().to_string()]}];
			// }
			// else {
				// continue;
			// }
		// }
		match self.search(&candidate.0){
			Some(val)=> {
					if val >= 4000000000 {
						return vec![Token{value: candidate.0, synonyms: self.synonym_dict[val as usize -4000000000].clone()}];
					}
					else{
						return vec![Token{value: candidate.0, synonyms: Vec::new()}];
					}
				}
			None => {
				let splits: Vec<Token> = candidate.0.split_whitespace().map(|s| Token{value: s.to_owned(), synonyms: Vec::new()}).collect();
				return splits;
			}
		}
	}
}















