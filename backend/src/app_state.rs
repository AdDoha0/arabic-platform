use sqlx::PgPool;
use std::sync::Arc;
use crate::modules::lesson_video::{
    repository::postgres::PostgresLessonVideoRepository, 
    service::{
        service_impl::LessonVideoServiceImpl, 
        service_trait::LessonVideoService
    }
};

/// Контейнер сервисов
#[derive(Clone)]
pub struct Services {
    pub lesson_video: Arc<dyn LessonVideoService + Send + Sync>,
}

impl Services {
    pub fn lesson_video(&self) -> &(dyn LessonVideoService + Send + Sync) {
        &*self.lesson_video
    }
}

/// Строитель сервисов
pub struct ServiceBuilder {
    db_pool: PgPool,
    lesson_video: Option<Arc<dyn LessonVideoService + Send + Sync>>,
}

impl ServiceBuilder {
    pub fn new(db_pool: PgPool) -> Self {
        Self {
            db_pool,
            lesson_video: None,
        }
    }

    /// Подменить сервис видеоуроков
    pub fn with_lesson_video_service(
        mut self,
        service: Arc<dyn LessonVideoService + Send + Sync>,
    ) -> Self {
        self.lesson_video = Some(service);
        self
    }

    /// Создать сервисы с настройками по умолчанию
    pub fn build(self) -> Services {
        // Создаем lesson_video сервис
        let lesson_video = self.lesson_video.unwrap_or_else(|| {
            let repo = PostgresLessonVideoRepository::new(self.db_pool.clone());
            Arc::new(LessonVideoServiceImpl::new(repo))
        });

        Services {
            lesson_video,
        }
    }
}

/// Глобальное состояние приложения
#[derive(Clone)]
pub struct AppState {
    pub db_pool: PgPool,
    pub services: Arc<Services>,
}

impl AppState {
    pub fn new(db_pool: PgPool) -> Self {
        let services = ServiceBuilder::new(db_pool.clone()).build();
        Self {
            db_pool,
            services: Arc::new(services),
        }
    }

    pub fn services(&self) -> &Services {
        &self.services
    }

    pub fn db_pool(&self) -> &PgPool {
        &self.db_pool
    }
}