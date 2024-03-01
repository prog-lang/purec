use im::HashSet;

#[derive(Clone)]
pub struct Scheme {
    pub guards: HashSet<Binder>,
    pub body: Type,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Unit,
    Int,
    Var(Binder),
    Func(Box<Type>, Box<Type>),
}

pub type Binder = String;

impl From<Type> for Scheme {
    fn from(body: Type) -> Self {
        Self {
            guards: HashSet::default(),
            body,
        }
    }
}

impl Scheme {
    pub fn forall(guard: Binder, body: Type) -> Self {
        Self::new(vec![guard], body)
    }

    pub fn new(guards: Vec<Binder>, body: Type) -> Self {
        Self {
            guards: guards.into(),
            body,
        }
    }

    pub fn from(guards: HashSet<Binder>, body: Type) -> Self {
        Self { guards, body }
    }

    pub fn map_body<F>(&self, f: F) -> Self
    where
        F: Fn(Type) -> Type,
    {
        Self::from(self.guards.clone(), f(self.body.clone()))
    }

    pub fn concrete(body: Type) -> Self {
        Self::new(vec![], body)
    }
}

impl Type {
    pub fn free_type_vars(&self) -> HashSet<Binder> {
        match self {
            Self::Var(binder) => vec![binder.clone()].into(),
            Self::Func(arg, res) => arg.free_type_vars().union(res.free_type_vars()),
            _ => HashSet::default(),
        }
    }

    pub fn func(params: Vec<Self>, result: Self) -> Self {
        params
            .into_iter()
            .rev()
            .fold(result, |prev, next| Self::Func(next.into(), prev.into()))
    }
}
