pub struct UnionFind {
    parents : Vec<usize>,
    size: Vec<usize>,
}

impl UnionFind {
    pub fn new(n:usize) -> UnionFind {
        UnionFind {
            parents: (0..n).map(|i| i).collect::<Vec<usize>>(),
            size: vec![1;n]
        }
    }

    pub fn unite( &mut self ,a :usize ,b : usize) {
        let pa = findParent(a);
        let pb = findParent(b);

        let (large,small) = if pa < pb {
            (pb,pa)
        } else {
            (pa,pb)
        }


        self.parents[large] = small;
        self.parents[a] = small;
        self.parents[b] = small;
        self.size[small] += self.size[large];
        self.size[large] = 0;

    }

    pub fn findParent(&mut self, a: usize) -> usize {
        if self.parents[a] == a{
            return a;
        }
        let root = self.findParent(self.parents[a]);
        self.parents[a] =  root;
        self.parents[a]
    }

    pub fn findSize(&mut self, a: usize) -> usize {
        let parent = self.findParent(a);
        self.size[parent]
    }
}

#[test]
fn union() {
    let uf = UnionFind::new(100);
    uf.unite(3,4);
    uf.unite(1,3);
    uf.unite(7,9);
    assert_eq!(1,uf.findParent(4));
    assert_eq!(1,uf.findParent(3));
    assert_eq!(2,uf.findSize(9));
    assert_eq!(3,uf.findSize(4));
    uf.unite(1,9);
    assert_eq!(6,uf.findSize(4));
    assert_eq!(6,uf.findSize(7));
    assert_eq!(1,uf.findParent(9));
    assert_eq!(1,uf.findParent(3));
    for i in 11..40 {
        uf.unite(i,i + 1);
    }
    uf.unite(30,3);
    assert_eq!(1,uf.findParent(9));
    assert_eq!(1,uf.findParent(3));
    assert_eq!(1,uf.findParent(32));
}