use crate::spec::{
    cpu::CPU,
    function::{get_function, Function},
    sched_data::ReadyQueue,
    scheduler,
};
use spin::mutex::SpinMutex;

type NodeGroup = Vec<Node>;

// The root node
#[derive(Debug, PartialEq, Eq)]
pub struct OracleTree {
    root: Node,
}

impl OracleTree {
    const fn new() -> Self {
        OracleTree {
            root: Node {
                expected_state: scheduler::State {
                    cpu: CPU::const_new(),
                    ready_queue: ReadyQueue::new(),
                    terminated_tasks: vec![],
                },
                edges: vec![],
            },
        }
    }

    pub fn init(num_core: u32) {
        let spawn = get_function(Function::Spawn);
        let states = spawn.call(&scheduler::State::new(num_core), 0, &[]);
        let node_group = {
            let mut v = vec![];
            for state in states.into_iter() {
                v.push(Node {
                    expected_state: state,
                    edges: vec![],
                });
            }
            v
        };

        let mut tree = ORACLE_TREE.lock();
        let root: &mut Node = &mut tree.root;

        root.add_edge(Edge {
            fn_type: Function::Spawn,
            args: vec![],
            node_group,
        });
    }

    pub fn get_init_nodes(&self) -> Vec<&Node> {
        let mut v = vec![];
        for edge in self.root.get_edges().into_iter() {
            v.append(&mut edge.node_group.iter().collect());
        }
        v
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Node {
    expected_state: scheduler::State,
    edges: Vec<Edge>,
}

impl Node {
    pub fn new(state: scheduler::State) -> Node {
        Node {
            expected_state: state,
            edges: vec![],
        }
    }

    pub fn add_edge(&mut self, edge: Edge) {
        self.edges.push(edge);
    }

    pub fn get_edges(&self) -> Vec<&Edge> {
        self.edges.iter().collect()
    }

    pub fn get_state(&self) -> &scheduler::State {
        &self.expected_state
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Edge {
    pub fn_type: Function,
    pub args: Vec<u32>,
    pub node_group: NodeGroup,
}

pub static ORACLE_TREE: SpinMutex<OracleTree> = SpinMutex::new(OracleTree::new());

#[cfg(test)]
mod tests {
    use super::{Edge, Node, OracleTree, ORACLE_TREE};
    use crate::spec::{
        cpu::{Core, CPU},
        function::Function::Spawn,
        sched_data::{ReadyQueue, TaskControlBlock, TaskState::Running},
        scheduler::State,
    };

    #[test]
    fn test_oracle_tree_init() {
        OracleTree::init(2);
        let tree = ORACLE_TREE.lock();
        assert_eq!(
            *tree,
            OracleTree {
                root: Node {
                    expected_state: State {
                        cpu: CPU { cores: vec![] },
                        ready_queue: ReadyQueue::new(),
                        terminated_tasks: vec![],
                    },
                    edges: vec![Edge {
                        fn_type: Spawn,
                        args: vec![],
                        node_group: vec![
                            Node {
                                expected_state: State {
                                    cpu: CPU {
                                        cores: vec![
                                            Core {
                                                id: 0,
                                                task: Some(TaskControlBlock {
                                                    tid: 1,
                                                    prio: 1,
                                                    state: Running,
                                                },),
                                            },
                                            Core { id: 1, task: None },
                                        ],
                                    },
                                    ready_queue: ReadyQueue::new(),
                                    terminated_tasks: vec![],
                                },
                                edges: vec![],
                            },
                            Node {
                                expected_state: State {
                                    cpu: CPU {
                                        cores: vec![
                                            Core { id: 0, task: None },
                                            Core {
                                                id: 1,
                                                task: Some(TaskControlBlock {
                                                    tid: 1,
                                                    prio: 1,
                                                    state: Running,
                                                },),
                                            },
                                        ],
                                    },
                                    ready_queue: ReadyQueue::new(),
                                    terminated_tasks: vec![],
                                },
                                edges: vec![],
                            },
                        ],
                    },],
                },
            }
        );
    }
}
