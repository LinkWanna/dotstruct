use std::{ffi::{c_char, c_ulonglong, CString}, io::Write, process::{Command, Stdio}};



static mut TIMES: u64 = 0;
static mut GRAPH: Graph = Graph { nodes: vec![], edges: vec![] };

#[derive(Debug)]
struct Node {
    id: c_ulonglong,
    label: CString,
}

// impl Drop for Node {
//     fn drop(&mut self) {
//         println!("drop node {}", self.id)
//     }
// }

#[derive(Debug)]
struct Edge {
    // 记录的是节点的 id
    from: c_ulonglong,
    to: c_ulonglong,
}


#[derive(Debug)]
struct Graph {
    nodes: Vec<Node>,
    edges: Vec<Edge>,
}

impl Graph  {
    pub fn to_dot(&self) -> String {
        let mut dot = String::from("digraph G {\n");

        for node in &self.nodes {
            dot += format!("n{} [label={:?}]\n", node.id, node.label).as_str();
        }

        for edge in &self.edges {
            dot += format!("n{} -> n{}\n", edge.from, edge.to).as_str();
        }
        dot += "}\n";
        dot
    }

    pub fn clear(&mut self) {
        self.nodes.clear();
        self.edges.clear();
    }
}

#[allow(clippy::not_unsafe_ptr_arg_deref)]
#[no_mangle]
pub extern "C" fn node(id: c_ulonglong, label: *mut c_char) {
    unsafe {
        let node = Node {
            id,
            label: CString::from_raw(label),
        };
        GRAPH.nodes.push(node);
    }
}

#[no_mangle]
pub extern "C" fn edge(from: c_ulonglong, to: c_ulonglong) {
    unsafe {
        let edge = Edge {
            from,
            to,
        };
        GRAPH.edges.push(edge);
    }
}

#[no_mangle]
pub extern "C" fn recoder() {
    unsafe { 
        TIMES += 1; 
        
        let mut child = Command::new("dot")
        .arg("-Tpng")
        .arg("-o")
        .arg(format!("debug/gragh{:02}.png", TIMES))
        .stdin(Stdio::piped())
        .spawn()
        .expect("failed to execute process");
    
        let dot = GRAPH.to_dot();
        child.stdin.take().unwrap().write_all(dot.as_bytes()).unwrap();
        child.wait().expect("failed to wait on child");
        // 清空 GRAPH
        GRAPH.clear();
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn graph() {
        let s1 = CString::new("n1").unwrap();
        let s2 = CString::new("n2").unwrap();
        
        node(1, s1.into_raw());
        node(2, s2.into_raw());
        edge(1, 2);
        unsafe {
            let dot = GRAPH.to_dot();
            println!("{}", dot);
            recoder();

            let dot = GRAPH.to_dot();
            print!("{}", dot);
            recoder()
        }
    }

}
