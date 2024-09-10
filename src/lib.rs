use std::{
    collections::HashMap, 
    ffi::{c_char, c_longlong, c_ulonglong, CString}, 
    io::Write, 
    process::{Command, Stdio},
    sync::Mutex
};
use lazy_static::lazy_static;

lazy_static! {
    static ref TIMES: Mutex<HashMap<String, u64>> = Mutex::new(HashMap::new());
}

static mut GRAPH: Graph = Graph { 
    directed: 1,
    root: -1, 
    nodes: vec![],
    edges: vec![],
};

#[derive(Debug)]
struct Node {
    id: c_ulonglong,
    label: CString,
}

#[derive(Debug)]
struct Edge {
    // 记录的是节点的 id
    from: c_ulonglong,
    to: c_ulonglong,
}



/// 默认值
/// directed: 1（有向图）
/// root: -1（代表不指定根节点）
#[derive(Debug)]
struct Graph {
    directed: c_char,
    root: c_longlong,
    nodes: Vec<Node>,
    edges: Vec<Edge>,
}

impl Graph  {

    pub fn to_dot(&self) -> String {
        // 判断是否为有向图
        let( mut dot, arrow) = if self.directed != 0 {
            (String::from("digraph G {\n"), "->")
        } else {
            (String::from("graph G {\n"), "--")
        };

        // 判断是否有根节点
        if self.root != -1 {
            dot += "root [shape=point]\n";
            dot += format!("root {} n{}\n", arrow, self.root).as_str();
        }

        for node in &self.nodes {
            dot += format!("n{} [label={:?}]\n", node.id, node.label).as_str();
        }

        for edge in &self.edges {
            dot += format!("n{} {} n{}\n", edge.from, arrow,edge.to).as_str();
        }
        dot += "}\n";
        dot
    }

    pub fn clear(&mut self) {
        self.nodes.clear();
        self.edges.clear();
    }
}

#[no_mangle]
pub extern "C" fn init(directed: c_char, root: c_longlong) {
    // 1. 有向图 / 无向图
    // 2. 根节点定义
    unsafe {
        GRAPH.directed = directed;
        GRAPH.root = root;
    }
}

/// # Safety
#[no_mangle]
pub unsafe extern "C" fn node(id: c_ulonglong, label: *mut c_char) {
    let node = Node {
        id,
        label: CString::from_raw(label),
    };
    GRAPH.nodes.push(node);
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

/// # Safety
#[no_mangle]
pub unsafe  extern "C" fn recoder(name: *mut c_char) {
    let name = CString::from_raw(name).to_string_lossy().to_string();
    let mut times = TIMES.lock().unwrap();
    
    // 如果不存在
    if !times.contains_key(&name) {
        times.insert(name.clone(), 1);
    } else {
        *times.get_mut(&name).unwrap() += 1;
    }
        
    let mut child = Command::new("dot")
    .arg("-Tpng")
    .arg("-o")
    .arg(format!("debug/{}_gragh{:02}.png", name, times[&name]))
    .stdin(Stdio::piped())
    .spawn()
    .expect("failed to execute process");

    let dot = GRAPH.to_dot();
    child.stdin.take().unwrap().write_all(dot.as_bytes()).unwrap();
    child.wait().expect("failed to wait on child");
    // 清空 GRAPH
    GRAPH.clear();
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn graph() {
        let s1 = CString::new("n1").unwrap();
        let s2 = CString::new("n2").unwrap();
        
        unsafe {
            node(1, s1.into_raw());
            node(2, s2.into_raw());
            edge(1, 2);
            let dot = GRAPH.to_dot();
            println!("{}", dot);
            recoder(CString::new("LL").unwrap().into_raw());

            let dot = GRAPH.to_dot();
            print!("{}", dot);
            recoder(CString::new("taget").unwrap().into_raw())
        }
    }

}
