use sqlx::PgPool;
use std::sync::Arc;
use crate::modules::lesson_video::{
    repository::postgres::PostgresLessonVideoRepository, 
    service::{
        service_impl::LessonVideoServiceImpl, 
        service_trait::LessonVideoService
    }
};

use crate::modules::lessons::{
    repository::postgres::PostgresLessonRepository, 
    service::{
        service_impl::LessonServiceImpl, 
        service_trait::LessonService
    }
};

use crate::modules::textbooks::{
    repository::postgres::PostgresTextbookRepository, 
    service::{
        service_impl::TextbookServiceImpl, 
        service_trait::TextbookService
    }
};


/// Контейнер сервисов
#[derive(Clone)]
pub struct Services {
    pub lesson_video: Arc<dyn LessonVideoService + Send + Sync>,
    pub lesson: Arc<dyn LessonService + Send + Sync>,
    pub textbook: Arc<dyn TextbookService + Send + Sync>,

}

impl Services {
    pub fn lesson_video(&self) -> &(dyn LessonVideoService + Send + Sync) {
        &*self.lesson_video
    }

    pub fn lesson(&self) -> &(dyn LessonService + Send + Sync) {
        &*self.lesson
    }

    pub fn textbook(&self) -> &(dyn TextbookService + Send + Sync) {
        &*self.textbook
    }
}

/// Строитель сервисов
pub struct ServiceBuilder {
    db_pool: PgPool,
    lesson_video: Option<Arc<dyn LessonVideoService + Send + Sync>>,
    lesson: Option<Arc<dyn LessonService + Send + Sync>>,
    textbook: Option<Arc<dyn TextbookService + Send + Sync>>,

}

impl ServiceBuilder {
    pub fn new(db_pool: PgPool) -> Self {
        Self {
            db_pool,
            lesson_video: None,
            lesson: None,
            textbook: None
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

    pub fn with_lesson_service(
        mut self,
        service: Arc<dyn LessonService + Send + Sync>,
    ) -> Self {
        self.lesson = Some(service);
        self
    }

    pub fn with_textbook_service(
        mut self,
        service: Arc<dyn TextbookService + Send + Sync>,
    ) -> Self {
        self.textbook = Some(service);
        self
    }

    /// Создать сервисы с настройками по умолчанию
    pub fn build(self) -> Services {
        // Создаем сервисы
        let lesson_video = self.lesson_video.unwrap_or_else(|| {
            let repo = PostgresLessonVideoRepository::new(self.db_pool.clone());
            Arc::new(LessonVideoServiceImpl::new(repo))
        });

        let lesson = self.lesson.unwrap_or_else(|| {
            let repo = PostgresLessonRepository::new(self.db_pool.clone());
            Arc::new(LessonServiceImpl::new(repo))
        });

        let textbook = self.textbook.unwrap_or_else(|| {
            let repo = PostgresTextbookRepository::new(self.db_pool.clone());
            Arc::new(TextbookServiceImpl::new(repo))
        });

        Services {
            lesson_video,
            lesson,
            textbook
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