# Развертывание czn-dioxus

Этот документ описывает процессы развертывания приложения czn-dioxus в различных средах.

## Обзор развертывания

Наша система развертывания обеспечивает:

- **Автоматизацию** - Минимизация ручных операций
- **Надежность** - Гарантированная доставка изменений
- **Безопасность** - Контроль доступа и проверка изменений
- **Гибкость** - Поддержка разных сред и платформ
- **Мониторинг** - Контроль состояния и производительности

## Среды развертывания

### 1. Development

**Описание**: Среда для разработки и тестирования

**Характеристики**:
- Автоматическое развертывание при каждом push
- Полный набор сервисов
- Возможность быстрой итерации
- Доступ к тестовым данным

**Требования**:
- Docker
- Docker Compose
- Node.js 16+
- Rust 1.70+

**Развертывание**:
```bash
# Клонирование репозитория
git clone https://github.com/yourusername/czn-dioxus.git
cd czn-dioxus

# Запуск development среды
docker-compose -f docker-compose.dev.yml up -d

# Проверка состояния
docker-compose -f docker-compose.dev.yml ps
```

**Конфигурация**:
```yaml
# docker-compose.dev.yml
version: '3.8'

services:
  app:
    build: .
    environment:
      - RUST_ENV=development
      - DATABASE_URL=postgresql://dev:dev@db:5432/dev
      - LOG_LEVEL=debug
    ports:
      - "3000:3000"
    volumes:
      - ./src:/app/src
      - ./logs:/app/logs
    depends_on:
      - db
      - redis
      
  db:
    image: postgres:13
    environment:
      POSTGRES_DB: dev
      POSTGRES_USER: dev
      POSTGRES_PASSWORD: dev
    volumes:
      - db_data:/var/lib/postgresql/data
      - ./scripts/init-db.sql:/docker-entrypoint-initdb.d/init.sql
      
  redis:
    image: redis:6-alpine
    ports:
      - "6379:6379"
    volumes:
      - redis_data:/data
      
  nginx:
    image: nginx:alpine
    ports:
      - "80:80"
    volumes:
      - ./nginx.dev.conf:/etc/nginx/nginx.conf
    depends_on:
      - app

volumes:
  db_data:
  redis_data:
```

### 2. Staging

**Описание**: Тестовая среда, максимально приближенная к production

**Характеристики**:
- Развертывание при merge в develop
- Продакшен-подобная конфигурация
- Тестирование перед production
- Автоматическое тестирование

**Развертывание**:
```bash
# Развертывание на staging
./scripts/deploy-staging.sh

# Проверка развертывания
./scripts/health-check.sh staging
```

**Скрипт развертывания**:
```bash
#!/bin/bash
# deploy-staging.sh

set -e

echo "Deploying to staging environment..."

# Сборка образа
docker build -t czn-dioxus:staging .

# Остановка старой версии
docker-compose -f docker-compose.staging.yml down

# Запуск новой версии
docker-compose -f docker-compose.staging.yml up -d

# Проверка работоспособности
sleep 30
./scripts/health-check.sh staging

echo "Staging deployment completed successfully"
```

### 3. Production

**Описание**: Продакшен среда для конечных пользователей

**Характеристики**:
- Ручное развертывание после тестирования
- Высокая доступность
- Мониторинг и логирование
- Резервное копирование
- Автоматический откат при ошибках

**Развертывание**:
```bash
# Подготовка к развертыванию
./scripts/pre-deploy-check.sh

# Создание резервной копии
./scripts/backup.sh

# Развертывание
./scripts/deploy-production.sh

# Проверка после развертывания
./scripts/post-deploy-check.sh
```

**Скрипт production развертывания**:
```bash
#!/bin/bash
# deploy-production.sh

set -e

echo "Starting production deployment..."

# Проверка окружения
if ! ./scripts/pre-deploy-check.sh; then
    echo "Pre-deployment checks failed"
    exit 1
fi

# Создание резервной копии
echo "Creating backup..."
./scripts/backup.sh

# Развертывание с использованием blue-green стратегии
echo "Deploying using blue-green strategy..."
./scripts/blue-green-deploy.sh

# Проверка после развертывания
echo "Running post-deployment checks..."
if ! ./scripts/post-deploy-check.sh; then
    echo "Post-deployment checks failed, rolling back..."
    ./scripts/rollback.sh
    exit 1
fi

echo "Production deployment completed successfully"
```

## Платформы развертывания

### 1. Docker

**Преимущества**:
- Изоляция окружения
- Повторяемость
- Простота развертывания
- Поддержка разных платформ

**Dockerfile**:
```dockerfile
# Multi-stage build
FROM rust:1.70 as builder

WORKDIR /app

# Копирование зависимостей
COPY Cargo.toml Cargo.lock ./
COPY frontend/package.json frontend/package-lock.json ./frontend/

# Установка зависимостей
RUN cargo install --locked --target x86_64-unknown-linux-musl --path .

# Сборка frontend
RUN cd frontend && npm install && npm run build

# Копирование исходного кода
COPY src ./src
COPY frontend/src ./frontend/src
COPY assets ./assets

# Сборка приложения
RUN cargo build --release --target x86_64-unknown-linux-musl

# Production stage
FROM gcr.io/distroless/static:nonroot

WORKDIR /app

# Копирование бинарника
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/czn-dioxus /app/czn-dioxus

# Копирование статических файлов
COPY --from=builder /app/frontend/dist /app/static

# Установка прав
USER nonroot:nonroot

# Запуск приложения
ENTRYPOINT ["./czn-dioxus"]
```

### 2. Kubernetes

**Преимущества**:
- Масштабируемость
- Высокая доступность
- Автоматическое управление
- Мониторинг и логирование

**Kubernetes манифесты**:

#### Deployment
```yaml
# k8s/deployment.yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: czn-dioxus
  labels:
    app: czn-dioxus
spec:
  replicas: 3
  selector:
    matchLabels:
      app: czn-dioxus
  template:
    metadata:
      labels:
        app: czn-dioxus
    spec:
      containers:
      - name: czn-dioxus
        image: ghcr.io/yourusername/czn-dioxus:latest
        ports:
        - containerPort: 3000
        env:
        - name: RUST_ENV
          value: "production"
        - name: DATABASE_URL
          valueFrom:
            secretKeyRef:
              name: czn-dioxus-secrets
              key: database-url
        resources:
          requests:
            memory: "256Mi"
            cpu: "250m"
          limits:
            memory: "512Mi"
            cpu: "500m"
        livenessProbe:
          httpGet:
            path: /health
            port: 3000
          initialDelaySeconds: 30
          periodSeconds: 10
        readinessProbe:
          httpGet:
            path: /ready
            port: 3000
          initialDelaySeconds: 5
          periodSeconds: 5
```

#### Service
```yaml
# k8s/service.yaml
apiVersion: v1
kind: Service
metadata:
  name: czn-dioxus-service
spec:
  selector:
    app: czn-dioxus
  ports:
  - protocol: TCP
    port: 80
    targetPort: 3000
  type: LoadBalancer
```

#### Ingress
```yaml
# k8s/ingress.yaml
apiVersion: networking.k8s.io/v1
kind: Ingress
metadata:
  name: czn-dioxus-ingress
  annotations:
    kubernetes.io/ingress.class: nginx
    cert-manager.io/cluster-issuer: letsencrypt-prod
    nginx.ingress.kubernetes.io/ssl-redirect: "true"
spec:
  tls:
  - hosts:
    - app.czn-dioxus.com
    secretName: czn-dioxus-tls
  rules:
  - host: app.czn-dioxus.com
    http:
      paths:
      - path: /
        pathType: Prefix
        backend:
          service:
            name: czn-dioxus-service
            port:
              number: 80
```

### 3. Cloud Platforms

#### AWS

**Использование AWS ECS**:
```yaml
# aws/ecs-task-definition.json
{
  "family": "czn-dioxus",
  "networkMode": "awsvpc",
  "requiresCompatibilities": ["FARGATE"],
  "cpu": "256",
  "memory": "512",
  "executionRoleArn": "arn:aws:iam::account:role/ecsTaskExecutionRole",
  "taskRoleArn": "arn:aws:iam::account:role/ecsTaskRole",
  "containerDefinitions": [
    {
      "name": "czn-dioxus",
      "image": "ghcr.io/yourusername/czn-dioxus:latest",
      "portMappings": [
        {
          "containerPort": 3000,
          "protocol": "tcp"
        }
      ],
      "environment": [
        {
          "name": "RUST_ENV",
          "value": "production"
        }
      ],
      "secrets": [
        {
          "name": "DATABASE_URL",
          "valueFrom": "arn:aws:secretsmanager:region:account:secret:czn-dioxus-db-url"
        }
      ],
      "logConfiguration": {
        "logDriver": "awslogs",
        "options": {
          "awslogs-group": "/ecs/czn-dioxus",
          "awslogs-region": "us-east-1",
          "awslogs-stream-prefix": "ecs"
        }
      }
    }
  ]
}
```

#### Azure

**Использование Azure Container Instances**:
```bash
# azure/deploy.sh
az container create \
  --resource-group czn-dioxus-rg \
  --name czn-dioxus-app \
  --image ghcr.io/yourusername/czn-dioxus:latest \
  --cpu 2 \
  --memory 4 \
  --ports 3000 \
  --environment-variables RUST_ENV=production \
  --secure-environment-variables DATABASE_URL=@/path/to/db-url.txt \
  --restart-policy Always
```

#### Google Cloud

**Использование Google Cloud Run**:
```yaml
# gcp/cloud-run.yaml
apiVersion: serving.knative.dev/v1
kind: Service
metadata:
  name: czn-dioxus
  annotations:
    run.googleapis.com/ingress: all
    run.googleapis.com/ingress-status: all
spec:
  template:
    metadata:
      annotations:
        autoscaling.knative.dev/minScale: "1"
        autoscaling.knative.dev/maxScale: "10"
        run.googleapis.com/cpu-throttling: "false"
    spec:
      containerConcurrency: 80
      timeoutSeconds: 900
      containers:
      - image: ghcr.io/yourusername/czn-dioxus:latest
        env:
        - name: RUST_ENV
          value: "production"
        resources:
          limits:
            cpu: "1000m"
            memory: "512Mi"
```

## Стратегии развертывания

### 1. Blue-Green Deployment

**Описание**: Две идентичные production среды

**Преимущества**:
- Минимальное время простоя
- Быстрый откат
- Полное тестирование перед переключением

**Реализация**:
```bash
#!/bin/bash
# blue-green-deploy.sh

set -e

CURRENT_ENV=$(get_current_environment)
NEW_ENV=$(get_new_environment $CURRENT_ENV)

echo "Deploying to $NEW_ENV environment..."

# Развертывание на новую среду
deploy_to_environment $NEW_ENV

# Проверка работоспособности
if health_check $NEW_ENV; then
    echo "Health check passed for $NEW_ENV"
    
    # Переключение трафика
    switch_traffic $NEW_ENV
    
    # Удаление старой среды
    cleanup_environment $CURRENT_ENV
    
    echo "Blue-Green deployment completed successfully"
else
    echo "Health check failed for $NEW_ENV"
    echo "Rolling back..."
    
    # Откат
    cleanup_environment $NEW_ENV
    switch_traffic $CURRENT_ENV
    exit 1
fi
```

### 2. Canary Deployment

**Описание**: Постепенное переключение трафика

**Преимущества**:
- Минимизация рисков
- Возможность быстрого отката
- Сбор метрик на реальных пользователях

**Реализация**:
```bash
#!/bin/bash
# canary-deploy.sh

set -e

# Начальный процент трафика
CANARY_PERCENTAGE=10

echo "Starting canary deployment with $CANARY_PERCENTAGE% traffic..."

# Развертывание canary версии
deploy_canary $CANARY_PERCENTAGE

# Мониторинг метрик
if monitor_metrics 5; then
    echo "Canary deployment successful"
    
    # Постепенное увеличение трафика
    for percentage in 30 60 100; do
        echo "Increasing canary traffic to $percentage%"
        deploy_canary $percentage
        
        if ! monitor_metrics 2; then
            echo "Canary deployment failed at $percentage%"
            rollback_canary
            exit 1
        fi
    done
    
    echo "Full canary deployment completed"
else
    echo "Canary deployment failed"
    rollback_canary
    exit 1
fi
```

### 3. Rolling Update

**Описание**: Поэтапное обновление инстансов

**Преимущества**:
- Непрерывная доступность
- Минимальные затраты
- Простота реализации

**Kubernetes Rolling Update**:
```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: czn-dioxus
spec:
  strategy:
    type: RollingUpdate
    rollingUpdate:
      maxSurge: 1
      maxUnavailable: 0
  replicas: 5
  # ... остальная конфигурация
```

## Мониторинг и логирование

### 1. Health Checks

**Реализация health check endpoint**:
```rust
// src/health.rs
use axum::{
    routing::get,
    Router,
    Json,
};
use serde::Serialize;
use sysinfo::{System, SystemExt};

#[derive(Serialize)]
struct HealthResponse {
    status: String,
    timestamp: String,
    version: String,
    memory_usage: u64,
    cpu_usage: f32,
}

async fn health_check() -> Json<HealthResponse> {
    let mut sys = System::new_all();
    sys.refresh_all();
    
    Json(HealthResponse {
        status: "healthy".to_string(),
        timestamp: chrono::Utc::now().to_rfc3339(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        memory_usage: sys.used_memory(),
        cpu_usage: sys.global_cpu_info().cpu_usage(),
    })
}

pub fn create_health_router() -> Router {
    Router::new()
        .route("/health", get(health_check))
        .route("/ready", get(health_check))
        .route("/live", get(health_check))
}
```

### 2. Логирование

**Centralized logging**:
```rust
// src/logging.rs
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub fn init_logging() {
    let file_appender = tracing_appender::rolling::daily("./logs", "app.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
    
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::fmt::layer()
                .with_writer(std::io::stdout)
                .with_target(false)
                .with_thread_ids(true)
                .with_thread_names(true)
        )
        .with(
            tracing_subscriber::fmt::layer()
                .with_writer(non_blocking)
                .with_ansi(false)
        )
        .init();
}
```

### 3. Метрики

**Prometheus metrics**:
```rust
// src/metrics.rs
use prometheus::{register_counter, register_histogram, Counter, Histogram};

lazy_static! {
    static ref HTTP_REQUESTS_TOTAL: Counter = register_counter!(
        "http_requests_total",
        "Total number of HTTP requests"
    ).unwrap();
    
    static ref HTTP_REQUEST_DURATION: Histogram = register_histogram!(
        "http_request_duration_seconds",
        "Duration of HTTP requests"
    ).unwrap();
}

pub fn record_request(duration: f64) {
    HTTP_REQUESTS_TOTAL.inc();
    HTTP_REQUEST_DURATION.observe(duration);
}
```

## Безопасность развертывания

### 1. Секреты

**Kubernetes Secrets**:
```yaml
# k8s/secrets.yaml
apiVersion: v1
kind: Secret
metadata:
  name: czn-dioxus-secrets
type: Opaque
data:
  database-url: <base64-encoded-database-url>
  encryption-key: <base64-encoded-encryption-key>
  api-key: <base64-encoded-api-key>
```

**Docker Secrets**:
```yaml
# docker-compose.prod.yml
version: '3.8'

services:
  app:
    image: czn-dioxus:latest
    secrets:
      - database_url
      - encryption_key
      
secrets:
  database_url:
    external: true
  encryption_key:
    external: true
```

### 2. Network Security

**Network policies**:
```yaml
# k8s/network-policy.yaml
apiVersion: networking.k8s.io/v1
kind: NetworkPolicy
metadata:
  name: czn-dioxus-network-policy
spec:
  podSelector:
    matchLabels:
      app: czn-dioxus
  policyTypes:
  - Ingress
  - Egress
  ingress:
  - from:
    - podSelector:
        matchLabels:
          app: ingress-nginx
    ports:
    - protocol: TCP
      port: 3000
  egress:
  - to:
    - podSelector:
        matchLabels:
          app: database
    ports:
    - protocol: TCP
      port: 5432
```

## Best Practices

### 1. Подготовка

- **Тестирование** - Полное тестирование перед развертыванием
- **Резервное копирование** - Создание бэкапов перед изменениями
- **Документирование** - Документирование изменений и процедур

### 2. Развертывание

- **Постепенное развертывание** - Использование стратегий blue-green или canary
- **Мониторинг** - Наблюдение за метриками во время развертывания
- **Готовность к откату** - Подготовка процедур отката

### 3. После развертывания

- **Проверка работоспособности** - Тестирование ключевых функций
- **Мониторинг** - Наблюдение за метриками и логами
- **Документирование** - Фиксация результатов развертывания

## Поддержка развертывания

### 1. Документация

- **Runbooks** - Пошаговые инструкции
- **Troubleshooting** - Решение типовых проблем
- **Checklists** - Контрольные списки

### 2. Инструменты

- **Monitoring** - Prometheus, Grafana
- **Logging** - ELK Stack, Fluentd
- **Alerting** - AlertManager, PagerDuty

### 3. Контакты

- **DevOps Team**: devops@czn-dioxus.com
- **On-call**: +7 (495) 123-45-67
- **Emergency**: emergency@czn-dioxus.com

## Обновления

Последнее обновление: 2024-01-01

Для получения актуальной информации:
- [Kubernetes Documentation](https://kubernetes.io/docs/)
- [Docker Documentation](https://docs.docker.com/)
- [AWS ECS Documentation](https://docs.aws.amazon.com/ecs/)
