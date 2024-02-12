use std::{collections::{HashMap, HashSet}, marker::PhantomData, ops::Deref, any::TypeId};

pub use interface_macros_impl::py_gen;
pub use interface_macros_impl::py_type_gen;

pub use lazy_static::lazy_static;

use pyo3::{Py, Python, PyResult, PyRef, PyClass};

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


pub enum PyFuncType {
    Normal,
    Property,
}
pub struct PyFunc {
    pub name: &'static str,
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
        strs.push(format!("def {}({args}) -> {}: ...", name, self.ret.deref()));

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
    Class(TypeProperties) 
}

impl StoredPyTypes {
    pub const fn new_func(name: &'static str, args: &'static [(&'static str, &'static dyn StaticString, &'static dyn StaticTypeId)], ret: &'static dyn StaticString, slf: bool) -> Self {
        Self::Fn(
            PyFunc {
                name,
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

    pub const fn new_class(typ: TypeProperties) -> Self {
        Self::Class(typ)
    } 
}

enum PyTreeNode {
    Path(HashMap<String, PyTreeNode>),
    Class {
        extends: String,
        typ: TypeProperties,
        inner: HashMap<String, PyTreeNode>
    },
    Func(&'static PyFunc)
}

fn add_to_path(map: &mut HashMap<String, PyTreeNode>, node: &'static StoredPyTypes, path: &[&'static str]) {
    //println!("path: [{}]", path.join(", "));
    if !path.is_empty() {
        if let Some(path_node) = map.get_mut(path[0]) {
            match path_node {
                PyTreeNode::Class { extends:_, typ: _, inner }
                | PyTreeNode::Path(inner) => {
                    add_to_path(inner, node, &path[1..]);
                },
                PyTreeNode::Func(_) => todo!(),
            }
        } else {
            let mut inner = HashMap::new();
            add_to_path(&mut inner, node, &path[1..]);
            map.insert(path[0].to_string(), PyTreeNode::Path(inner));
        }
        return;
    }
    match node {
        StoredPyTypes::Fn(_) => todo!(),
        StoredPyTypes::Class(class) => {
            if let Some(exist) = map.remove(&class.name.deref()[..]) {
                match exist {
                    PyTreeNode::Path(inner) => {
                        map.insert(class.name.deref().clone(), PyTreeNode::Class {
                            extends: class.extend.deref().clone(),
                            typ: class.clone(),
                            inner
                        });
                    },
                    PyTreeNode::Class { extends:_, typ, inner:_ } => {
                        //std::any::Any::type_id(*class)
                        if typ.type_id.deref() != class.type_id.deref() {
                            panic!("`{}` already exists!", class.name.deref());
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
                inner: HashMap::new()
            });
        }
        StoredPyTypes::Impl(imp) => {
            let mut inner = if let Some(node) = map.remove(&imp.typ.name.deref()[..]) {
                match node {
                    PyTreeNode::Path(inner) => {
                        inner
                    },
                    PyTreeNode::Class { extends:_, typ, inner } => {
                        if typ.type_id.deref() == imp.typ.type_id.deref() {
                            inner
                        } else {
                            panic!("Two definitions of class!")
                        }
                    },
                    PyTreeNode::Func(_) => todo!(),
                }
            } else {
                HashMap::new()
            };

            for func in imp.funcs {
                inner.insert(func.name.to_string(), PyTreeNode::Func(*func));
            }

            map.insert(imp.typ.name.deref().clone(), PyTreeNode::Class {
                extends: imp.typ.extend.deref().clone(),
                typ: imp.typ.clone(),
                inner
            });
        },
    }
}
fn add_node(map: &mut HashMap<String, PyTreeNode>, node: &'static StoredPyTypes) {
    match node {
        StoredPyTypes::Fn(func) => {
            map.insert(func.name.to_string(), PyTreeNode::Func(func));
        },
        StoredPyTypes::Impl(imp) => {
            add_to_path(map, node, imp.typ.path)
        },
        StoredPyTypes::Class(class) => {
            add_to_path(map, node, class.path)
        }
    }
}

fn nodes_to_string(map: HashMap<String, PyTreeNode>) -> Vec<String> {
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
            PyTreeNode::Class { extends, typ: _, inner } => {
                if inner.is_empty() {
                    collected.push(format!("class {name}({extends}): ..."))
                } else {
                    collected.push(format!("class {name}({extends}):"));
                    for sub_str in nodes_to_string(inner) {
                        collected.push(format!("\t{sub_str}"));
                    }
                }
            },
            PyTreeNode::Func(func) => {
                collected.append(&mut func.to_strings());
            },
        }
    }

    collected
}
pub fn collect_stored_types() -> String {
    let mut map = HashMap::new();
    for py_type in inventory::iter::<StoredPyTypes> {
        add_node(&mut map, py_type);
    }

    nodes_to_string(map).join("\n")
}

inventory::collect!(StoredPyTypes);

