#[allow(unused)]
use crate::{error::ErrorType, log_debug, log_error, log_info, log_warn};

/// A real query in the ECS system
pub type Query = super::generational::GenerationalKey;

/// A static query generator
pub(crate) struct QueryGenerator {
    /// The total number of created queries
    pub nb_queries_total: usize,
    /// The current generatrion of queries
    pub generation: super::generational::GenerationalGeneration,
}

impl QueryGenerator {
    /// Creates a new generator
    pub fn init() -> Self {
        Self {
            nb_queries_total: 0,
            generation: 0,
        }
    }

    /// Creates new queries
    pub fn generate_queries(&mut self, nb_queries: usize) -> Vec<Query> {
        let mut new_queries = vec![];
        for _ in 0..nb_queries {
            match self.nb_queries_total.checked_add(1) {
                Some(res) => self.nb_queries_total = res,
                None => {
                    self.nb_queries_total = 0;
                    self.generation += 1;
                }
            }
            let new_query = Query {
                index: self.nb_queries_total,
                generation: self.generation,
            };
            new_queries.push(new_query);
        }
        new_queries
    }
}

use crate::platform_layer::PlatformLayerRwLock;
use once_cell::sync::Lazy;

/// The global query generator to interface between user request and real queries
pub(crate) static GLOBAL_QUERY_GENERATOR: Lazy<PlatformLayerRwLock<QueryGenerator>> =
    Lazy::new(|| PlatformLayerRwLock::new(QueryGenerator::init()));

pub type OnGetComponentValueQueryCallback = Box<
    dyn Fn(&mut dyn crate::Game, Box<dyn super::component::RealComponent>) -> Result<(), ErrorType>
        + Send
        + Sync,
>;
pub type UserOnGetComponentValueQueryCallback<G, T> = fn(&mut G, T) -> Result<(), ErrorType>;

pub struct UserQueryCallbackBuilder;
impl UserQueryCallbackBuilder {
    pub fn on_get_component_value<G, T>(
        callback: UserOnGetComponentValueQueryCallback<G, T>,
    ) -> OnGetComponentValueQueryCallback
    where
        G: crate::Game + 'static,
        T: crate::Component + 'static,
    {
        Box::new(move |game, value| {
            let value = match value.into_any().downcast::<T>() {
                Ok(v) => *v,
                Err(err) => {
                    log_error!(
                        "Failed to downcast component `{}`: {:?}",
                        std::any::type_name::<T>(),
                        err
                    );
                    return Err(ErrorType::Unknown);
                }
            };

            let game = match (game as &mut dyn std::any::Any).downcast_mut::<G>() {
                Some(g) => g,
                None => {
                    log_error!("Failed to downcast game `{}`", std::any::type_name::<G>());
                    return Err(ErrorType::Unknown);
                }
            };

            callback(game, value)
        })
    }
}
