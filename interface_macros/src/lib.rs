use std::{collections::{HashMap, HashSet}, marker::PhantomData, ops::Deref, any::TypeId, hash::Hash, borrow::Borrow};

pub use interface_macros_impl::py_gen;
pub use interface_macros_impl::py_type_gen;

pub use lazy_static::lazy_static;

use pyo3::{Py, Python, PyResult, PyRef, PyClass};


pub trait PyPath {
    const PATH: &'static [&'static str] = &[];
    const NAME: &'static str;
}
pub trait PyType {
    const PATH: &'static [&'static str] = &[];
    fn to_string() -> String;
    fn path_string() -> String {
        Self::PATH.into_iter()
            .map(|a| a.to_string())
            .chain(vec![Self::to_string()])
            .collect::<Vec<String>>()
            .join(".")
    }
    fn extend_string() -> String {
        "object".to_string()
    }
}

impl PyType for u8 { fn to_string() -> String {"int".to_string()} }
impl PyType for u16 { fn to_string() -> String {"int".to_string()} }
impl PyType for u32 { fn to_string() -> String {"int".to_string()} }
impl PyType for u64 { fn to_string() -> String {"int".to_string()} }
impl PyType for u128 { fn to_string() -> String {"int".to_string()} }
impl PyType for usize { fn to_string() -> String {"int".to_string()} }
impl PyType for i8 { fn to_string() -> String {"int".to_string()} }
impl PyType for i16 { fn to_string() -> String {"int".to_string()} }
impl PyType for i32 { fn to_string() -> String {"int".to_string()} }
impl PyType for i64 { fn to_string() -> String {"int".to_string()} }
impl PyType for i128 { fn to_string() -> String {"int".to_string()} }
impl PyType for isize { fn to_string() -> String {"int".to_string()} }


impl PyType for f32 { fn to_string() -> String {"float".to_string()} }
impl PyType for f64 { fn to_string() -> String {"float".to_string()} }

impl PyType for () { fn to_string() -> String {"None".to_string()} }

impl PyType for ::pyo3::PyAny {
    fn to_string() -> String {
        "object".to_string()
    }
}

impl<'a> PyType for ::pyo3::Python<'a> {
    fn to_string() -> String {
        "None".to_string()
    }
}

impl<T: PyType> PyType for PhantomData<T> {
    fn to_string() -> String {
        T::path_string()
    }
}

impl<'a, T: PyType> PyType for &'a T {
    fn to_string() -> String {
        T::path_string()
    }
}


impl PyType for String {
    fn to_string() -> String {
        "str".to_string()
    }
}
impl<'a> PyType for &'a str {
    fn to_string() -> String {
        "str".to_string()
    }
}

impl PyType for bool {
    fn to_string() -> String {
        "bool".to_string()
    }
}

impl<T1: PyType, T2: PyType> PyType for (T1, T2) {
    fn to_string() -> String {
        format!("tuple[{}, {}]", T1::path_string(), T2::path_string())
    }
}

impl<T1: PyType, T2: PyType, T3: PyType> PyType for (T1, T2, T3) {
    fn to_string() -> String {
        format!("tuple[{}, {}, {}]", T1::path_string(), T2::path_string(), T3::path_string())
    }
}

impl<T1: PyType, T2: PyType, T3: PyType, T4: PyType> PyType for (T1, T2, T3, T4) {
    fn to_string() -> String {
        format!("tuple[{}, {}, {}, {}]", T1::path_string(), T2::path_string(), T3::path_string(), T4::path_string())
    }
}

impl<T1: PyType, T2: PyType, T3: PyType, T4: PyType, T5: PyType> PyType for (T1, T2, T3, T4, T5) {
    fn to_string() -> String {
        format!("tuple[{}, {}, {}, {}, {}]", T1::path_string(), T2::path_string(), T3::path_string(), T4::path_string(), T5::path_string())
    }
}

impl<T: PyType> PyType for [T] {
    fn to_string() -> String {
        format!("list[{}]", T::path_string())
    }
}

impl<T: PyType> PyType for &[T] {
    fn to_string() -> String {
        format!("list[{}]", T::path_string())
    }
}

impl<T: PyType, const N: usize> PyType for [T; N] {
    fn to_string() -> String {
        format!("list[{}]", T::path_string())
    }
}

impl<T: PyType> PyType for Vec<T> {
    fn to_string() -> String {
        format!("list[{}]", T::path_string())
    }
}

impl<T: PyType, G: PyType> PyType for HashMap<T, G> {
    fn to_string() -> String {
        format!("dict[{}, {}]", T::path_string(), G::path_string())
    }
}

impl<T: PyType> PyType for HashSet<T> {
    fn to_string() -> String {
        format!("set[{}]", T::path_string())
    }
}

impl<T: PyType> PyType for Option<T> {
    fn to_string() -> String {
        format!("None | {}", T::path_string())
    }
}

impl<'a, T: PyType + PyClass> PyType for PyRef<'a, T> {
    fn to_string() -> String {
        T::path_string()
    }
}

/*pub trait PyUnion {
    const PARTS: &'static [&'static dyn TypeWrapper];
}
impl<T: PyUnion> PyType for T {
    fn to_string() -> String {
        //format!("{} | {}", T::A::path_string(), T::B::path_string())
        T::PARTS.iter()
            .map(|a| a.path_string())
            .collect::<Vec<String>>()
            .join(" | ")
    }
}*/


impl<T: PyType> PyType for Py<T> {
    fn to_string() -> String {
        T::path_string()
    }
}

impl<T: PyType, G> PyType for Result<T, G> {
    fn to_string() -> String {
        T::path_string()
    }
}





pub trait PyBridge<T>: Sized {
    type PyOut;

    fn into_py(self, py: Python) -> PyResult<Self::PyOut>;

    fn from_py(val: T, py: Python) -> PyResult<Self>;
}

impl<T, U: PyBridge<T>> PyBridge<Vec<T>> for Vec<U> {
    type PyOut = Vec<U::PyOut>;

    fn into_py(self, py: Python) -> PyResult<Self::PyOut> {
        self.into_iter().map(|a| a.into_py(py)).collect()
    }

    fn from_py(val: Vec<T>, py: Python) -> PyResult<Self> {
        val.into_iter().map(|a| U::from_py(a, py)).collect()
    }
}

impl<T1, U1, T2, U2> PyBridge<(T1, T2)> for (U1, U2) 
where
    U1: PyBridge<T1>,
    U2: PyBridge<T2>
{
    type PyOut = (U1::PyOut, U2::PyOut);

    fn into_py(self, py: Python) -> PyResult<Self::PyOut> {
        Ok((
            self.0.into_py(py)?,
            self.1.into_py(py)?
        ))
    }

    fn from_py(val: (T1, T2), py: Python) -> PyResult<Self> {
        Ok((
            U1::from_py(val.0, py)?,
            U2::from_py(val.1, py)?
        ))
    }
}


/*
impl<U, K, T: PyBridge<U>, G:PyBridge<K>> PyBridge<HashMap<U, K>> for HashMap<T, G> 
where
    T::PyOut: Hash + Eq,
    T: Hash + Eq,
{
    type PyOut = HashMap<T::Bridge, G::Bridge>;

    fn into_py(self, py: Python) -> Self::Bridge {
        HashMap::from_iter(
            self.into_iter().map(|(key, val)| {
                (
                    key.into_py(py),
                    val.into_py(py)
                )
            })
        )
    }

    fn from_py(val: Self::Bridge, py: Python) -> Self {
        HashMap::from_iter(
            val.into_iter().map(|(key, val)| {
                (
                    T::from_py(key, py),
                    G::from_py(val, py)
                )
            })
        )
    }
    
}*/


pub use inventory;

pub const fn check_pytype<T: PyType>() {}


pub trait StaticString: Deref<Target = String> + Send + Sync {}
impl<T> StaticString for T 
where
    T: Deref<Target = String>,
    T: Send + Sync
{
    
}
pub trait StaticTypeId: Deref<Target = TypeId> + Send + Sync {}
impl<T> StaticTypeId for T 
where
    T: Deref<Target = TypeId>,
    T: Send + Sync
{

}


#[derive(Clone, Copy)]
pub struct TypeProperties {
    pub name: &'static dyn StaticString,
    pub name_path: &'static dyn StaticString,
    pub extend: &'static dyn StaticString,
    pub path: &'static [&'static str],
    pub type_id: &'static dyn StaticTypeId,
}



#[derive(Debug)]
pub enum PyFuncType {
    Normal,
    Property,
}
pub struct PyFunc {
    pub name: &'static str,
    pub comments: &'static [&'static str],
    pub args: &'static [(&'static str, &'static dyn StaticString, &'static dyn StaticTypeId)],
    pub ret: &'static dyn StaticString,
    pub slf: bool,
    pub typ: PyFuncType
}
impl PyFunc {
    pub fn to_strings(&self) -> Vec<String> {

        let args = if self.slf {
            vec!["self".to_string()]
        } else {
            vec![]
        };

        let args = args.into_iter()
            .chain(
                self.args.iter()
                    .filter_map(|(a,b, c)| {
                        let b = (*b).deref();
                        if (*c).deref() == &TypeId::of::<pyo3::Python>() {
                            None
                        } else if b.len() < 6 || &b[0..6] != "None |" {
                            Some(format!("{a}: {}", b))
                        } else {
                            Some(format!("{a}: {} = None", b))
                        }
                    })
            ).collect::<Vec<String>>()
            .join(", ");

        let mut strs = Vec::new();

        let name = match &self.typ {
            PyFuncType::Normal => self.name.to_string(),
            PyFuncType::Property => {
                strs.push(format!("@property"));
                if self.name.len() < 5 {
                    self.name.to_string()
                } else if &self.name[0..4] == "get_" {
                    self.name[4..].to_string()
                } else {
                    self.name.to_string()
                }
            },
        };

        strs.push(format!("def {}({args}) -> {}:", name, self.ret.deref()));

        if !self.comments.is_empty() {
            strs.push("\t\"\"\"".to_string());
            for comment in self.comments {
                strs.push(format!("\t{comment}"))
            }
            strs.push("\t\"\"\"".to_string());
        }
        strs.push("\t...".to_string());

        strs
    }
}


pub struct PyImpl {
    typ: TypeProperties,
    funcs: &'static [&'static PyFunc],
}

pub enum StoredPyTypes {
    Fn(PyFunc),
    Impl(PyImpl),
    Class(TypeProperties, &'static [&'static str], &'static [&'static dyn StaticString]) ,
}

impl StoredPyTypes {
    pub const fn new_func(name: &'static str, comments: &'static [&'static str], args: &'static [(&'static str, &'static dyn StaticString, &'static dyn StaticTypeId)], ret: &'static dyn StaticString, slf: bool) -> Self {
        Self::Fn(
            PyFunc {
                name,
                comments,
                args,
                ret,
                slf,
                typ: PyFuncType::Normal
            }
        )
    }

    pub const fn new_impl(typ: TypeProperties, funcs: &'static [&'static PyFunc]) -> Self {
        Self::Impl(
            PyImpl {
                typ,
                funcs,
            }
        )
    }

    pub const fn new_class(typ: TypeProperties, comments: &'static [&'static str], declarations: &'static [&'static dyn StaticString]) -> Self {
        Self::Class(typ, comments, declarations)
    } 
}

#[derive(Clone)]
struct OrderedHashMap<T: Clone + Hash + Eq, U: Clone> {
    key_order: Vec<T>,
    hash_map: HashMap<T, U>
}
impl<T: Clone + Hash + Eq, U: Clone> OrderedHashMap<T, U> {
    fn new() -> Self {
        Self {
            key_order: Vec::new(),
            hash_map: HashMap::new()
        }
    }
    fn insert(&mut self, t: T, u: U) -> Option<U> {
        if self.hash_map.contains_key(&t) {
            self.key_order.retain(|a| *a != t);
        }

        self.key_order.push(t.clone());
        self.hash_map.insert(t, u)
    }

    fn get_mut<G>(&mut self, g: &G) -> Option<&mut U> 
    where
        G: ?Sized,
        T: Borrow<G>,
        G: Hash + Eq,
    {
        self.hash_map.get_mut(g)
    }

    fn remove<G>(&mut self, g: &G) -> Option<U> 
    where
        G: ?Sized,
        T: Borrow<G>,
        G: Hash + Eq,
        T: PartialEq<G>
    {
        if self.hash_map.contains_key(&g) {
            self.key_order.retain(|a| a != g);
        }
        self.hash_map.remove(g)
    }

    fn is_empty(&self) -> bool {
        self.hash_map.is_empty()
    }


}

impl<T: Clone + Hash + Eq, U: Clone> IntoIterator for OrderedHashMap<T, U> {
    type Item = (T, U);

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(mut self) -> Self::IntoIter {
        self.key_order.into_iter().map(|a| {
            let b = self.hash_map.remove(&a).unwrap();
            (a, b)
        }).collect::<Vec<(T, U)>>()
            .into_iter()
    }
}


#[derive(Clone)]
enum PyTreeNode {
    Path(OrderedHashMap<String, PyTreeNode>),
    Class {
        extends: String,
        typ: TypeProperties,
        inner: OrderedHashMap<String, PyTreeNode>,
        declarations: &'static [&'static dyn StaticString],
        comments: &'static [&'static str],
    },
    Func(&'static PyFunc)
}

fn add_to_path(map: &mut OrderedHashMap<String, PyTreeNode>, node: &'static StoredPyTypes, path: &[&'static str]) {
    //println!("path: [{}]", path.join(", "));
    if !path.is_empty() {
        if let Some(path_node) = map.get_mut(path[0]) {
            match path_node {
                PyTreeNode::Class { extends:_, typ: _, inner, declarations: _, comments: _ }
                | PyTreeNode::Path(inner) => {
                    add_to_path(inner, node, &path[1..]);
                },
                PyTreeNode::Func(_) => todo!(),
            }
        } else {
            let mut inner = OrderedHashMap::new();
            add_to_path(&mut inner, node, &path[1..]);
            map.insert(path[0].to_string(), PyTreeNode::Path(inner));
        }
        return;
    }
    match node {
        StoredPyTypes::Fn(_) => todo!(),
        StoredPyTypes::Class(class, comments, declarations) => {
            if let Some(mut exist) = map.remove(&class.name.deref()[..]) {
                match exist {
                    PyTreeNode::Path(inner) => {
                        map.insert(class.name.deref().clone(), PyTreeNode::Class {
                            extends: class.extend.deref().clone(),
                            typ: class.clone(),
                            inner,
                            declarations,
                            comments
                        });
                    },
                    PyTreeNode::Class { extends:_, typ, inner:_, comments: _, declarations: _ } => {
                        //std::any::Any::type_id(*class)
                        if typ.type_id.deref() != class.type_id.deref() {
                            panic!("`{}` already exists!", class.name.deref());
                        }
                        if let PyTreeNode::Class {extends: _, typ: _, inner: _, comments: old_comments, declarations: old_declarations} = &mut exist {
                            *old_comments = comments;
                            *old_declarations = declarations;
                        }
                        map.insert(class.name.deref().clone(), exist);
                    },
                    PyTreeNode::Func(_) => panic!("`{}` already exists!", class.name.deref()),
                }
                return;
            }
            map.insert(class.name.deref().clone(), PyTreeNode::Class {
                extends: class.extend.deref().clone(),
                typ: *class,
                inner: OrderedHashMap::new(),
                declarations,
                comments
            });
        }
        StoredPyTypes::Impl(imp) => {
            let mut new_comments: &[&str] = &[];
            let mut declarations: &'static [&'static dyn StaticString] = &[];
            let mut inner = if let Some(node) = map.remove(&imp.typ.name.deref()[..]) {
                match node {
                    PyTreeNode::Path(inner) => {
                        //declarations = &[];
                        inner
                    },
                    PyTreeNode::Class { extends:_, typ, inner, comments, declarations: old_declarations } => {
                        if typ.type_id.deref() == imp.typ.type_id.deref() {
                            new_comments = comments;
                            declarations = old_declarations;
                            inner
                        } else {
                            panic!("Two definitions of class!")
                        }
                    },
                    PyTreeNode::Func(_) => todo!(),
                }
            } else {
                OrderedHashMap::new()
            };


            for func in imp.funcs {
                //eprintln!("func: {}", func.name.to_string());
                inner.insert(func.name.to_string(), PyTreeNode::Func(*func));
            }
            map.insert(imp.typ.name.deref().clone(), PyTreeNode::Class {
                extends: imp.typ.extend.deref().clone(),
                typ: imp.typ.clone(),
                inner,
                declarations,
                comments: new_comments
            });
        },
    }
}
fn add_node(map: &mut OrderedHashMap<String, PyTreeNode>, node: &'static StoredPyTypes) {
    match node {
        StoredPyTypes::Fn(func) => {
            map.insert(func.name.to_string(), PyTreeNode::Func(func));
        },
        StoredPyTypes::Impl(imp) => {
            add_to_path(map, node, imp.typ.path)
        },
        StoredPyTypes::Class(class, _comments, _declarations) => {
            add_to_path(map, node, class.path)
        }
    }
}

fn nodes_to_string(map: OrderedHashMap<String, PyTreeNode>) -> Vec<String> {
    let mut collected = Vec::new();

    for (name, val) in map {
        match val {
            PyTreeNode::Path(path) => {
                if path.is_empty() {
                    collected.push(format!("class {name}: ..."));
                } else {
                    collected.push(format!("class {name}:"));
                    for sub_str in nodes_to_string(path) {
                        collected.push(format!("\t{sub_str}"));
                    }
                }
            },
            PyTreeNode::Class { extends, typ: _, inner, comments, declarations } => {
                collected.push(format!("class {name}({extends}):"));

                if comments.len() != 0 {
                    collected.push("\t\"\"\"".to_string());
                    for comment in comments {
                        collected.push(format!("\t{comment}"));
                    }
                    collected.push("\t\"\"\"".to_string());
                }
                for decl in declarations {
                    collected.push(format!("\t{}", decl.to_string()));
                }

                for sub_str in nodes_to_string(inner) {
                    collected.push(format!("\t{sub_str}"));
                }

                collected.push("\t...".to_string());
                
            },
            PyTreeNode::Func(func) => {
                collected.append(&mut func.to_strings());
            },
        }
    }

    collected
}


fn declare_declerations(map: OrderedHashMap<String, PyTreeNode>, path: Vec<String>, out: &mut Vec<String>) {
    //println!("{}{}", "\t".repeat(path.len()) ,path.join("."));
    for (key, val) in map {
        match val {
            PyTreeNode::Path(inner) => {
                let mut next_path = path.clone();
                next_path.push(key.clone());
                declare_declerations(inner, next_path, out);
            },
            PyTreeNode::Class { extends: _, typ: _, inner, declarations, comments: _ } => {
                //println!("outer path {key}: {}", path.join("."));
                    //println!("outer path {key}: {}", path.join("."));
                
                for decl in declarations {
                    //println!("{}inner path {key}: {}", "\t".repeat(path.len()+1), path.join("."));
                    if path.is_empty() {
                        out.push(format!("{key}.{}", decl.to_string()));
                    } else {
                        out.push(format!("{key}.{}.{}", path.join("."), decl.to_string()));
                    }
                    
                }
                let mut next_path = path.clone();
                next_path.push(key.clone());
                declare_declerations(inner, next_path, out);
            },
            PyTreeNode::Func(_) => (),
        }
    }

}
pub fn collect_stored_types() -> (String, String) {
    let mut map = OrderedHashMap::new();
    for py_type in inventory::iter::<StoredPyTypes> {
        add_node(&mut map, py_type);
    }

    let mut declarations = Vec::new();
    declare_declerations(map.clone(), Vec::new(), &mut declarations);

    let declarations = declarations.join("\n");

    (
        nodes_to_string(map).join("\n"),
        declarations
    )
}

inventory::collect!(StoredPyTypes);

