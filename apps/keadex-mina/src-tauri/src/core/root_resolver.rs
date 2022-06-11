use crate::resolver::Resolver;
use crate::controller::diagram_controller::DiagramController;

/**
Resolver for the core.
The goal of the resolver is to let consumers construct components without having to know what their dependencies are.
The `Resolver` type wraps resolvers from other modules.
Private implementation details live on the wrapped resolvers.
Controllers and repositories are resolved from this `Resolver`.
*/
pub struct RootResolver {
  pub diagram_controller_resolver: Resolver<DiagramController>
}

impl Default for RootResolver {
  fn default() -> Self {
    RootResolver {
      diagram_controller_resolver: Default::default()
    }
  }
}

impl RootResolver {
  // pub(in crate::core) fn resolve<T>(&self, register: &Register<T>) -> T
  // where
  //   T: Clone,
  // {
  //   (register.0)(self)
  // }
}















// /**
// A registration in the resolver.
// Registers produce values of a specific type when a command or query asks for them.
// The values may be singletons that share a single value, or they may be created on-demand.
// */
// #[derive(Clone)]
// pub struct Register<T>(Arc<dyn Fn(&Resolver) -> T + Send + Sync>);

// impl<T> Register<T> {
//   /**
//   Create a register that returns the same instance of a value.
//   */
//   pub fn once(f: impl Fn(&Resolver) -> T + Send + Sync + 'static) -> Self
//   where
//     T: Send + Sync + Clone + 'static,
//   {
//     let cell = OnceCell::new();
//     Register(Arc::new(move |resolver| {
//       cell.get_or_init(|| f(resolver)).clone()
//     }))
//   }

//   /**
//   Create a register that returns a new instance of a value each time.
//   */
//   pub fn factory(f: impl Fn(&Resolver) -> T + Send + Sync + 'static) -> Self {
//     Register(Arc::new(f))
//   }
// }